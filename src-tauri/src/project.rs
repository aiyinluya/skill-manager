use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::load_config;
use crate::installer::sync_to_editor;
use crate::models::{
    ActivateResult, Editor, ProjectConfig, ProjectInfo, ProjectSkill, ProjectView, Skill,
};
use crate::scanner::{scan_all, scan_editor_skills};

// ---------- 路径与配置 ----------

fn config_path_of(project_path: &Path) -> PathBuf {
    project_path.join(".skillconfig")
}

pub fn load_project_config(project_path: &Path) -> ProjectConfig {
    let p = config_path_of(project_path);
    if p.exists() {
        if let Ok(s) = fs::read_to_string(&p) {
            if let Ok(cfg) = serde_json::from_str::<ProjectConfig>(&s) {
                return cfg;
            }
        }
    }
    ProjectConfig::default()
}

pub fn save_project_config(project_path: &Path, cfg: &ProjectConfig) {
    if let Some(parent) = config_path_of(project_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(s) = serde_json::to_string_pretty(cfg) {
        let _ = fs::write(config_path_of(project_path), s);
    }
}

// ---------- 项目识别 (B1) ----------

fn git_remote(project_path: &Path) -> Option<String> {
    let out = Command::new("git")
        .args(["-C", &project_path.to_string_lossy(), "remote", "get-url", "origin"])
        .output()
        .ok()?;
    if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !s.is_empty() {
            return Some(s);
        }
    }
    None
}

fn path_fingerprint(project_path: &Path) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let canon = fs::canonicalize(project_path)
        .unwrap_or_else(|_| project_path.to_path_buf())
        .to_string_lossy()
        .into_owned();
    let mut h = DefaultHasher::new();
    canon.hash(&mut h);
    format!("{:x}", h.finish())
}

pub fn resolve_project_id(project_path: &Path, cfg: &ProjectConfig) -> String {
    if let Some(id) = &cfg.project_id {
        if !id.is_empty() {
            return id.clone();
        }
    }
    if let Some(remote) = git_remote(project_path) {
        return format!("git:{}", remote.trim_end_matches(".git").replace('/', "_"));
    }
    format!("path:{}", path_fingerprint(project_path))
}

pub fn open_project(path: String) -> ProjectInfo {
    let project_path = Path::new(&path);
    let name = project_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.clone());
    let cfg = load_project_config(project_path);
    let project_id = resolve_project_id(project_path, &cfg);
    ProjectInfo {
        path: path.clone(),
        name,
        project_id,
        has_config: config_path_of(project_path).exists(),
        config_path: config_path_of(project_path).to_string_lossy().into_owned(),
    }
}

// ---------- 项目层级编辑器 (复用全局 editor 映射，改写 skill_dir) ----------

fn project_editor(editor: &Editor, project_path: &Path) -> Editor {
    let sub = Path::new(&editor.skill_dir)
        .parent()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let pd = project_path.join(sub).join("skills");
    Editor {
        id: editor.id.clone(),
        name: editor.name.clone(),
        icon: editor.icon.clone(),
        skill_dir: pd.to_string_lossy().into_owned(),
        color: editor.color.clone(),
    }
}

// ---------- 技术栈检测 (B3) ----------

/// 关键词 -> 人类可读标签
fn keyword_labels() -> Vec<(String, String)> {
    vec![
        // JS/TS 生态
        ("react".into(), "React".into()),
        ("next".into(), "Next.js".into()),
        ("vue".into(), "Vue".into()),
        ("nuxt".into(), "Nuxt".into()),
        ("svelte".into(), "Svelte".into()),
        ("angular".into(), "Angular".into()),
        ("express".into(), "Express".into()),
        ("nest".into(), "NestJS".into()),
        ("fastify".into(), "Fastify".into()),
        ("vite".into(), "Vite".into()),
        ("webpack".into(), "Webpack".into()),
        ("tailwind".into(), "TailwindCSS".into()),
        ("typescript".into(), "TypeScript".into()),
        ("javascript".into(), "JavaScript".into()),
        ("node".into(), "Node.js".into()),
        ("electron".into(), "Electron".into()),
        // Python 生态
        ("python".into(), "Python".into()),
        ("fastapi".into(), "FastAPI".into()),
        ("django".into(), "Django".into()),
        ("flask".into(), "Flask".into()),
        ("pandas".into(), "Pandas".into()),
        ("numpy".into(), "NumPy".into()),
        ("pytorch".into(), "PyTorch".into()),
        ("tensorflow".into(), "TensorFlow".into()),
        ("pytest".into(), "Pytest".into()),
        ("poetry".into(), "Poetry".into()),
        // Rust
        ("rust".into(), "Rust".into()),
        ("tokio".into(), "Tokio".into()),
        ("actix".into(), "Actix".into()),
        ("axum".into(), "Axum".into()),
        ("rocket".into(), "Rocket".into()),
        ("tauri".into(), "Tauri".into()),
        ("wasm".into(), "WebAssembly".into()),
        // Go
        ("golang".into(), "Go".into()),
        ("gin".into(), "Gin".into()),
        ("echo".into(), "Echo".into()),
        ("fiber".into(), "Fiber".into()),
        // Java / JVM
        ("java".into(), "Java".into()),
        ("spring".into(), "Spring".into()),
        ("kotlin".into(), "Kotlin".into()),
        ("gradle".into(), "Gradle".into()),
        ("maven".into(), "Maven".into()),
        // 其他
        ("docker".into(), "Docker".into()),
        ("kubernetes".into(), "Kubernetes".into()),
        ("terraform".into(), "Terraform".into()),
        ("postgres".into(), "PostgreSQL".into()),
        ("mysql".into(), "MySQL".into()),
        ("mongodb".into(), "MongoDB".into()),
        ("redis".into(), "Redis".into()),
        ("graphql".into(), "GraphQL".into()),
        ("supabase".into(), "Supabase".into()),
        ("rails".into(), "Ruby on Rails".into()),
        ("ruby".into(), "Ruby".into()),
        ("php".into(), "PHP".into()),
        ("swift".into(), "Swift".into()),
        ("flutter".into(), "Flutter".into()),
        ("android".into(), "Android".into()),
        ("ios".into(), "iOS".into()),
    ]
}

fn manifest_hits(project_path: &Path) -> HashSet<String> {
    let mut hits: HashSet<String> = HashSet::new();
    let add = |hits: &mut HashSet<String>, key: &str| {
        hits.insert(key.to_string());
    };

    // package.json
    let pj = project_path.join("package.json");
    if pj.exists() {
        if let Ok(s) = fs::read_to_string(&pj) {
            let low = s.to_lowercase();
            add(&mut hits, "node");
            add(&mut hits, "javascript");
            if low.contains("\"typescript\"") || low.contains("\"ts-node\"") {
                add(&mut hits, "typescript");
            }
            for kw in [
                "react", "next", "vue", "nuxt", "svelte", "angular", "express", "nest",
                "fastify", "vite", "webpack", "tailwind", "electron", "graphql",
            ] {
                if low.contains(&format!("\"{}\"", kw)) || low.contains(&format!("\"@{}\"", kw)) {
                    add(&mut hits, kw);
                }
            }
        }
    }
    // pyproject.toml / requirements.txt
    for fname in ["pyproject.toml", "requirements.txt", "setup.py", "Pipfile"] {
        let f = project_path.join(fname);
        if f.exists() {
            if let Ok(s) = fs::read_to_string(&f) {
                let low = s.to_lowercase();
                add(&mut hits, "python");
                for kw in [
                    "fastapi", "django", "flask", "pandas", "numpy", "pytorch", "tensorflow",
                    "pytest", "poetry", "supabase",
                ] {
                    if low.contains(kw) {
                        add(&mut hits, kw);
                    }
                }
            }
        }
    }
    // Cargo.toml
    let cargo = project_path.join("Cargo.toml");
    if cargo.exists() {
        add(&mut hits, "rust");
        if let Ok(s) = fs::read_to_string(&cargo) {
            let low = s.to_lowercase();
            for kw in ["tokio", "actix", "axum", "rocket", "tauri", "wasm"] {
                if low.contains(kw) {
                    add(&mut hits, kw);
                }
            }
        }
    }
    // go.mod
    let gomod = project_path.join("go.mod");
    if gomod.exists() {
        add(&mut hits, "golang");
        if let Ok(s) = fs::read_to_string(&gomod) {
            let low = s.to_lowercase();
            for kw in ["gin", "echo", "fiber"] {
                if low.contains(kw) {
                    add(&mut hits, kw);
                }
            }
        }
    }
    // JVM
    for fname in ["pom.xml", "build.gradle", "build.gradle.kts"] {
        let f = project_path.join(fname);
        if f.exists() {
            add(&mut hits, if fname == "pom.xml" { "maven" } else { "gradle" });
            add(&mut hits, "java");
            if let Ok(s) = fs::read_to_string(&f) {
                let low = s.to_lowercase();
                if low.contains("spring") {
                    add(&mut hits, "spring");
                }
                if low.contains("kotlin") {
                    add(&mut hits, "kotlin");
                }
            }
        }
    }
    // 框架标记文件
    let markers: &[(&str, &str)] = &[
        ("next.config", "next"),
        ("vite.config", "vite"),
        ("tailwind.config", "tailwind"),
        ("tsconfig.json", "typescript"),
        ("vue.config", "vue"),
        ("angular.json", "angular"),
        ("manage.py", "django"),
        ("Gemfile", "ruby"),
        ("config.ru", "rails"),
        ("dockerfile", "docker"),
        ("docker-compose", "docker"),
        ("compose.yaml", "docker"),
        ("terraform", "terraform"),
        ("supabase", "supabase"),
    ];
    if let Ok(entries) = fs::read_dir(project_path) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_lowercase();
            for (m, kw) in markers {
                if name.contains(m) {
                    add(&mut hits, kw);
                }
            }
        }
    }
    // .claude / .cursor 项目级 skills 存在 → 该项目系 AI 编辑器项目（弱信号，不单独打标）
    hits
}

/// 扫描项目文本文件，收集被"引用"的 skill 名称集合 (B3 信号4)
fn referenced_skills(project_path: &Path, candidates: &[Skill]) -> HashSet<String> {
    let mut found: HashSet<String> = HashSet::new();
    if candidates.is_empty() {
        return found;
    }
    let keys: Vec<(String, String)> = candidates
        .iter()
        .map(|s| (s.dir_name.to_lowercase(), s.name.to_lowercase()))
        .collect();
    let mut buf = String::new();
    let mut bytes = 0usize;
    let walk = |dir: &Path, buf: &mut String, bytes: &mut usize| -> () {
        let mut stack = vec![dir.to_path_buf()];
        while let Some(d) = stack.pop() {
            if *bytes > 4_000_000 {
                return;
            }
            let entries = match fs::read_dir(&d) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for e in entries.flatten() {
                let p = e.path();
                let name = e.file_name().to_string_lossy().into_owned();
                if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    if name == "node_modules" || name == ".git" || name.starts_with('.') {
                        continue;
                    }
                    stack.push(p);
                } else if let Some(ext) = p.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if matches!(
                        ext.as_str(),
                        "md" | "txt" | "json" | "toml" | "yaml" | "yml" | "rs" | "py"
                            | "js" | "ts" | "tsx" | "jsx"
                    ) {
                        if let Ok(s) = fs::read_to_string(&p) {
                            *bytes += s.len();
                            buf.push_str(&s.to_lowercase());
                            buf.push(' ');
                        }
                    }
                }
            }
        }
    };
    walk(project_path, &mut buf, &mut bytes);
    for (dir, name) in &keys {
        if !dir.is_empty() && buf.contains(dir) {
            found.insert(dir.clone());
        } else if !name.is_empty() && name.len() > 2 && buf.contains(name) {
            found.insert(dir.clone());
        }
    }
    found
}

// ---------- 主聚合 (B2/B3) ----------

fn skill_tokens(s: &Skill) -> Vec<String> {
    let mut v: Vec<String> = vec![
        s.name.to_lowercase(),
        s.dir_name.to_lowercase(),
        s.summary.to_lowercase(),
    ];
    v.extend(s.triggers.iter().map(|t| t.to_lowercase()));
    // frontmatter 中可能声明的适用条件
    if let Some(obj) = s.frontmatter.as_object() {
        for key in ["applies_to", "tags", "stack", "category", "categories", "keywords"] {
            if let Some(val) = obj.get(key) {
                match val {
                    serde_json::Value::Array(arr) => {
                        for it in arr {
                            v.push(it.to_string().to_lowercase());
                        }
                    }
                    _ => v.push(val.to_string().to_lowercase()),
                }
            }
        }
    }
    v
}

pub fn get_project_skills(path: String) -> ProjectView {
    let cfg = load_config();
    let project_path = Path::new(&path);
    let pinfo = open_project(path.clone());

    // 1. 检测技术栈
    let hits = manifest_hits(project_path);
    let labels: Vec<String> = {
        let kl = keyword_labels();
        let mut out = Vec::new();
        for (kw, label) in &kl {
            if hits.contains(kw) {
                out.push(label.clone());
            }
        }
        out.sort();
        out.dedup();
        out
    };

    // 2. 扫描项目级 skills
    let mut project_skills: Vec<ProjectSkill> = Vec::new();
    for e in &cfg.editors {
        let pe = project_editor(e, project_path);
        for s in scan_editor_skills(&pe) {
            project_skills.push(ProjectSkill {
                skill: s,
                layer: "project".into(),
                source: "project".into(),
                signals: vec!["位于项目目录内（项目自带技能）".into()],
                enabled: true,
                default_enabled: true,
            });
        }
    }

    // 3. 全局 skills
    let (all, _per) = scan_all(&cfg.editors);

    // 4. 引用的 skill（项目已引用）
    let refs = referenced_skills(project_path, &all);

    // 5. 分类
    let override_map = load_project_config(project_path).overrides;
    let kl = keyword_labels();
    let mut recommended: Vec<ProjectSkill> = Vec::new();
    let mut available: Vec<ProjectSkill> = Vec::new();

    for s in &all {
        let tokens = skill_tokens(s);
        let mut signals: Vec<String> = Vec::new();
        for (kw, label) in &kl {
            if hits.contains(kw) {
                for t in &tokens {
                    if t.contains(kw) {
                        signals.push(format!("命中技术栈: {}", label));
                        break;
                    }
                }
            }
        }
        if refs.contains(&s.dir_name.to_lowercase()) {
            signals.push("项目已引用此技能".into());
        }

        let (layer, default_enabled): (String, bool) = if !signals.is_empty() {
            (String::from("recommended"), true)
        } else {
            (String::from("available"), false)
        };
        let key = format!("{}:{}", s.editor_id, s.dir_name);
        let enabled = match override_map.get(&key).map(|v| v.as_str()) {
            Some("enable") => true,
            Some("disable") => false,
            _ => default_enabled,
        };
        let ps = ProjectSkill {
            skill: s.clone(),
            layer: layer.clone(),
            source: "global".into(),
            signals,
            enabled,
            default_enabled,
        };
        if layer == "recommended" {
            recommended.push(ps);
        } else {
            available.push(ps);
        }
    }

    // 排序：推荐按重要性/名称；其他按名称
    recommended.sort_by(|a, b| a.skill.name.cmp(&b.skill.name));
    available.sort_by(|a, b| a.skill.name.cmp(&b.skill.name));

    ProjectView {
        info: pinfo,
        stack: labels,
        project: project_skills,
        recommended,
        available,
        overrides: override_map,
    }
}

// ---------- 用户覆盖配置 (B4) ----------

pub fn set_project_skill(path: String, skill_key: String, enabled: bool) -> ProjectConfig {
    let project_path = Path::new(&path);
    let mut cfg = load_project_config(project_path);
    cfg.version = 1;
    cfg.overrides.insert(
        skill_key.clone(),
        if enabled { "enable".into() } else { "disable".into() },
    );
    save_project_config(project_path, &cfg);
    record_feedback(&project_path.to_string_lossy(), &skill_key, enabled);
    cfg
}

pub fn get_project_config(path: String) -> ProjectConfig {
    load_project_config(Path::new(&path))
}

pub fn set_project_extra_paths(path: String, paths: Vec<String>) -> ProjectConfig {
    let project_path = Path::new(&path);
    let mut cfg = load_project_config(project_path);
    cfg.version = 1;
    cfg.extra_paths = paths;
    save_project_config(project_path, &cfg);
    cfg
}

// ---------- E2 激活联动 ----------

pub fn activate_project_skills(path: String, project_id: Option<String>) -> ActivateResult {
    let cfg = load_config();
    let view = get_project_skills(path.clone());
    let mut activated = Vec::new();
    for ps in &view.project {
        if !ps.enabled {
            continue;
        }
        let eid = &ps.skill.editor_id;
        if let Some(editor) = cfg.editors.iter().find(|e| e.id == *eid) {
            let src = Path::new(&ps.skill.skill_dir);
            let target = Path::new(&editor.skill_dir).join(&ps.skill.dir_name);
            sync_to_editor(
                &src.to_string_lossy(),
                &target.to_string_lossy(),
                &cfg.sync_mode,
            );
            activated.push(format!("{} → {}", ps.skill.name, editor.name));
        }
    }
    // 读取项目配置，把"启用"的推荐/可用也同步进全局编辑器
    let project_path = Path::new(&path);
    let pcfg = load_project_config(project_path);
    for ps in view.recommended.iter().chain(view.available.iter()) {
        let key = format!("{}:{}", ps.skill.editor_id, ps.skill.dir_name);
        let is_enabled = match pcfg.overrides.get(&key).map(|v| v.as_str()) {
            Some("enable") => true,
            Some("disable") => false,
            _ => ps.default_enabled,
        };
        if !is_enabled {
            continue;
        }
        // 把 central 库作为中转：确保存在（项目自带副本已在 project 区处理）
        let central = PathBuf::from(&cfg.central_skills_dir).join(&ps.skill.dir_name);
        if !central.exists() {
            if let Ok(meta) = fs::symlink_metadata(&ps.skill.skill_dir) {
                if meta.file_type().is_symlink() {
                    let _ = std::os::windows::fs::symlink_dir(&ps.skill.skill_dir, &central);
                } else {
                    copy_dir_simple(&ps.skill.skill_dir, &central.to_string_lossy());
                }
            }
        }
        if let Some(editor) = cfg.editors.iter().find(|e| e.id == ps.skill.editor_id) {
            let target = Path::new(&editor.skill_dir).join(&ps.skill.dir_name);
            sync_to_editor(
                &central.to_string_lossy(),
                &target.to_string_lossy(),
                &cfg.sync_mode,
            );
            activated.push(format!("{} → {}", ps.skill.name, editor.name));
        }
    }
    let _ = project_id;
    ActivateResult {
        success: true,
        count: activated.len(),
        activated,
    }
}

fn copy_dir_simple(src: &str, dest: &str) {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);
    if !dest_path.exists() {
        let _ = fs::create_dir_all(dest_path);
    }
    if let Ok(entries) = fs::read_dir(src_path) {
        for entry in entries.flatten() {
            let sp = entry.path();
            let dp = dest_path.join(entry.file_name());
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name == "node_modules" || name == ".git" {
                    continue;
                }
                copy_dir_simple(&sp.to_string_lossy(), &dp.to_string_lossy());
            } else if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                let _ = fs::copy(&sp, &dp);
            }
        }
    }
}

// ---------- E1 项目复用模板 ----------

pub fn export_project_template(path: String) -> String {
    let cfg = load_project_config(Path::new(&path));
    let template = serde_json::json!({
        "version": 1,
        "kind": "skill-manager-project-template",
        "extra_paths": cfg.extra_paths,
        "overrides": cfg.overrides,
    });
    serde_json::to_string_pretty(&template).unwrap_or_else(|_| "{}".into())
}

pub fn import_project_template(path: String, template_json: String) -> ProjectConfig {
    let project_path = Path::new(&path);
    let mut cfg = load_project_config(project_path);
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&template_json) {
        if let Some(ep) = v.get("extra_paths").and_then(|x| x.as_array()) {
            cfg.extra_paths = ep
                .iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .collect();
        }
        if let Some(ov) = v.get("overrides").and_then(|x| x.as_object()) {
            for (k, val) in ov {
                if let Some(s) = val.as_str() {
                    if s == "enable" || s == "disable" {
                        cfg.overrides.insert(k.clone(), s.to_string());
                    }
                }
            }
        }
    }
    cfg.version = 1;
    save_project_config(project_path, &cfg);
    cfg
}

// ---------- E3 反馈闭环 ----------

fn feedback_file() -> PathBuf {
    crate::config::data_dir().join("feedback.json")
}

pub fn record_feedback(project_path: &str, skill_key: &str, enabled: bool) {
    let f = feedback_file();
    let mut map: HashMap<String, usize> = HashMap::new();
    if f.exists() {
        if let Ok(s) = fs::read_to_string(&f) {
            if let Ok(m) = serde_json::from_str::<HashMap<String, usize>>(&s) {
                map = m;
            }
        }
    }
    let counter = if enabled { "enabled" } else { "disabled" };
    *map.entry(format!("{}:{}", counter, skill_key)).or_insert(0) += 1;
    *map.entry(format!("project:{}", project_path)).or_insert(0) += 1;
    *map.entry("total".into()).or_insert(0) += 1;
    if let Ok(s) = serde_json::to_string_pretty(&map) {
        let _ = fs::write(&f, s);
    }
}

pub fn get_feedback_stats() -> crate::models::FeedbackStats {
    let f = feedback_file();
    let mut stats = crate::models::FeedbackStats::default();
    if f.exists() {
        if let Ok(s) = fs::read_to_string(&f) {
            if let Ok(m) = serde_json::from_str::<HashMap<String, usize>>(&s) {
                stats.total_events = *m.get("total").unwrap_or(&0);
                for (k, v) in &m {
                    if let Some(rest) = k.strip_prefix("enabled:") {
                        stats.enabled += v;
                        stats.per_project
                            .entry(rest.to_string())
                            .or_insert(0);
                    } else if let Some(_rest) = k.strip_prefix("disabled:") {
                        stats.disabled += v;
                    } else if let Some(rest) = k.strip_prefix("project:") {
                        *stats.per_project.entry(rest.to_string()).or_insert(0) += v;
                    }
                }
            }
        }
    }
    stats
}
