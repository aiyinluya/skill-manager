use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::{central_skills_dir, data_dir, load_config, AppConfig};
use crate::models::{AuditIssue, InstallResult, InstallItem, MarketplaceSkill, MarketplaceSource};
use crate::parser::parse_skill_md;
use crate::scanner::find_skill_md_files;

// ---------- 源存储 (D1) ----------

fn marketplace_file() -> PathBuf {
    data_dir().join("marketplace.json")
}

pub fn load_sources() -> Vec<MarketplaceSource> {
    let f = marketplace_file();
    if f.exists() {
        if let Ok(s) = fs::read_to_string(&f) {
            if let Ok(v) = serde_json::from_str::<Vec<MarketplaceSource>>(&s) {
                return v;
            }
        }
    }
    // 默认预置一个安全、离线的本地源（中心仓库）
    let def = vec![MarketplaceSource {
        id: "builtin-central".into(),
        name: "本机中心仓库".into(),
        url: central_skills_dir().to_string_lossy().into_owned(),
        source_type: "local".into(),
        branch: String::new(),
        subpath: String::new(),
    }];
    save_sources_pub(&def);
    def
}

pub fn save_sources_pub(sources: &[MarketplaceSource]) {
    if let Some(parent) = marketplace_file().parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(s) = serde_json::to_string_pretty(sources) {
        let _ = fs::write(marketplace_file(), s);
    }
}

fn gen_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("src_{}", n % 1_000_000_000)
}

// ---------- 浏览 (D2 / A14) ----------

fn is_installed(dir_name: &str, cfg: &AppConfig) -> bool {
    let central = Path::new(&cfg.central_skills_dir).join(dir_name);
    if central.exists() {
        return true;
    }
    for e in &cfg.editors {
        if Path::new(&e.skill_dir).join(dir_name).exists() {
            return true;
        }
    }
    false
}

fn cache_dir_for(id: &str) -> PathBuf {
    data_dir().join("cache").join(id)
}

fn ensure_github_clone(source: &MarketplaceSource) -> Option<PathBuf> {
    let id = &source.id;
    let cache = cache_dir_for(id);
    let needs_clone = !cache.join(".git").exists();
    if needs_clone {
        let _ = fs::remove_dir_all(&cache);
        let url = normalize_github_url(&source.url);
        let branch = if source.branch.is_empty() {
            "main".into()
        } else {
            source.branch.clone()
        };
        let clone_url = if url.contains("github.com") {
            url.clone()
        } else {
            format!("https://github.com/{}", url.trim_start_matches("git@github.com:"))
        };
        let out = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--branch",
                &branch,
                &clone_url,
                &cache.to_string_lossy(),
            ])
            .output();
        if !out.as_ref().map(|o| o.status.success()).unwrap_or(false) {
            let out2 = Command::new("git")
                .args([
                    "clone",
                    "--depth",
                    "1",
                    "--branch",
                    "master",
                    &clone_url,
                    &cache.to_string_lossy(),
                ])
                .output();
            if !out2.map(|o| o.status.success()).unwrap_or(false) {
                return None;
            }
        }
    }
    Some(cache)
}

fn scan_dir_for_skills(root: &Path) -> Vec<(PathBuf, String)> {
    let mut found: Vec<(PathBuf, String)> = Vec::new();
    find_skill_md_files(root, root, &mut found);
    found
}

fn build_skill(
    source: &MarketplaceSource,
    name: String,
    summary: String,
    triggers: Vec<String>,
    dir_name: String,
    skill_dir: String,
    installed: bool,
    extra: serde_json::Value,
) -> MarketplaceSkill {
    MarketplaceSkill {
        source_id: source.id.clone(),
        source_name: source.name.clone(),
        name,
        dir_name,
        summary,
        triggers,
        skill_dir,
        installed,
        update_available: false,
        extra,
    }
}

pub fn browse_marketplace(refresh: bool) -> Vec<MarketplaceSkill> {
    let cfg = load_config();
    let sources = load_sources();
    let mut out = Vec::new();
    for source in &sources {
        match source.source_type.as_str() {
            "local" => {
                let dir = Path::new(&source.url);
                if !dir.exists() {
                    continue;
                }
                for (md, rel) in scan_dir_for_skills(dir) {
                    let sdir = md.parent().unwrap_or(dir).to_string_lossy().into_owned();
                    let parsed = parse_skill_md(&md);
                    let pname = parsed.as_ref().map(|p| p.name.clone()).unwrap_or_default();
                    let name = if pname.is_empty() {
                        Path::new(&rel)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned()
                    } else {
                        pname
                    };
                    let installed = is_installed(&rel, &cfg);
                    out.push(build_skill(
                        source,
                        name,
                        parsed.as_ref().map(|p| p.summary.clone()).unwrap_or_default(),
                        parsed.as_ref().map(|p| p.triggers.clone()).unwrap_or_default(),
                        rel,
                        sdir,
                        installed,
                        serde_json::Value::Null,
                    ));
                }
            }
            "github" => {
                if refresh {
                    let _ = fs::remove_dir_all(cache_dir_for(&source.id));
                }
                if let Some(cache) = ensure_github_clone(source) {
                    let root = if source.subpath.is_empty() {
                        cache.clone()
                    } else {
                        cache.join(&source.subpath)
                    };
                    for (md, rel) in scan_dir_for_skills(&root) {
                        let sdir = md.parent().unwrap_or(&root).to_string_lossy().into_owned();
                        let parsed = parse_skill_md(&md);
                        let pname = parsed.as_ref().map(|p| p.name.clone()).unwrap_or_default();
                        let name = if pname.is_empty() {
                            Path::new(&rel)
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .into_owned()
                        } else {
                            pname
                        };
                        let installed = is_installed(&rel, &cfg);
                        out.push(build_skill(
                            source,
                            name,
                            parsed.as_ref().map(|p| p.summary.clone()).unwrap_or_default(),
                            parsed.as_ref().map(|p| p.triggers.clone()).unwrap_or_default(),
                            rel,
                            sdir,
                            installed,
                            serde_json::Value::Null,
                        ));
                    }
                }
            }
            "remote" => {
                if let Some(json) = fetch_remote_index(&source.url) {
                    if let Some(arr) = json.get("skills").and_then(|x| x.as_array()) {
                        for item in arr {
                            let name = item
                                .get("name")
                                .and_then(|x| x.as_str())
                                .unwrap_or("")
                                .to_string();
                            let dir_name = item
                                .get("dir_name")
                                .and_then(|x| x.as_str())
                                .unwrap_or(&name)
                                .to_string();
                            let summary = item
                                .get("summary")
                                .and_then(|x| x.as_str())
                                .unwrap_or("")
                                .to_string();
                            let triggers = item
                                .get("triggers")
                                .and_then(|x| x.as_array())
                                .map(|a| {
                                    a.iter()
                                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                                        .collect()
                                })
                                .unwrap_or_default();
                            let installed = is_installed(&dir_name, &cfg);
                            out.push(build_skill(
                                source,
                                name,
                                summary,
                                triggers,
                                dir_name,
                                String::new(),
                                installed,
                                item.clone(),
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

fn fetch_remote_index(url: &str) -> Option<serde_json::Value> {
    let out = Command::new("curl")
        .args(["-fsSL", url])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    serde_json::from_slice(&out.stdout).ok()
}

// ---------- 安装 (D3) ----------

fn normalize_github_url(url: &str) -> String {
    let s = url.trim();
    if s.starts_with("http") || s.starts_with("git@") {
        s.to_string()
    } else {
        format!("https://github.com/{}", s)
    }
}

pub fn install_from_marketplace(
    source_id: String,
    dir_name: String,
    target_editor_id: Option<String>,
) -> Result<InstallResult, String> {
    let cfg = load_config();
    let sources = load_sources();
    let source = sources
        .iter()
        .find(|s| s.id == source_id)
        .ok_or("商店源不存在")?;
    let target = target_editor_id.unwrap_or_else(|| "central".into());

    // 找到 skill 实际目录
    let skill_dir = match source.source_type.as_str() {
        "local" => Path::new(&source.url).join(&dir_name).to_string_lossy().into_owned(),
        "github" => {
            let cache = ensure_github_clone(source)
                .ok_or("GitHub 源克隆失败（请检查网络/仓库地址）")?;
            let root = if source.subpath.is_empty() {
                cache
            } else {
                cache.join(&source.subpath)
            };
            root.join(&dir_name).to_string_lossy().into_owned()
        }
        "remote" => {
            // 由索引附带的 repo/subpath 安装
            let repo = source
                .url
                .clone();
            let branch = source.branch.clone();
            let subpath = if source.subpath.is_empty() {
                dir_name.clone()
            } else {
                format!("{}/{}", source.subpath, dir_name)
            };
            return install_remote(repo, branch, subpath, &target, &cfg);
        }
        _ => return Err("未知源类型".into()),
    };

    if !Path::new(&skill_dir).exists() {
        return Err(format!("技能目录不存在: {}", skill_dir));
    }

    // 复用到 central 库 + 目标编辑器
    let central = Path::new(&cfg.central_skills_dir).join(&dir_name);
    let _ = fs::remove_dir_all(&central);
    copy_dir(&skill_dir, &central.to_string_lossy());
    let mut installed = Vec::new();
    if target != "central" {
        if let Some(editor) = cfg.editors.iter().find(|e| e.id == target) {
            let target_path = Path::new(&editor.skill_dir).join(&dir_name);
            crate::installer::sync_to_editor(
                &central.to_string_lossy(),
                &target_path.to_string_lossy(),
                &cfg.sync_mode,
            );
            installed.push(InstallItem {
                name: dir_name.clone(),
                editor: editor.name.clone(),
                status: "installed".into(),
            });
        }
    } else {
        installed.push(InstallItem {
            name: dir_name.clone(),
            editor: "central library".into(),
            status: "installed".into(),
        });
    }
    Ok(InstallResult {
        success: true,
        count: installed.len(),
        installed,
    })
}

fn install_remote(
    repo: String,
    branch: String,
    subpath: String,
    target: &str,
    cfg: &AppConfig,
) -> Result<InstallResult, String> {
    let url = normalize_github_url(&repo);
    let (user, reponame, _sp, _br) = crate::installer::parse_github_url_pub(&url)?;
    let tmp = std::env::temp_dir().join(format!("sm-mkt-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let clone_url = format!("https://github.com/{}/{}.git", user, reponame);
    let br = if branch.is_empty() { "main".into() } else { branch };
    let out = Command::new("git")
        .args(["clone", "--depth", "1", "--branch", &br, &clone_url, &tmp.to_string_lossy()])
        .output();
    if !out.map(|o| o.status.success()).unwrap_or(false) {
        let _ = fs::remove_dir_all(&tmp);
        return Err("远程索引仓库克隆失败".into());
    }
    let source = tmp.join(&subpath);
    if !source.exists() {
        let _ = fs::remove_dir_all(&tmp);
        return Err(format!("远程索引中未找到子路径: {}", subpath));
    }
    let central = Path::new(&cfg.central_skills_dir).join(
        Path::new(&subpath)
            .file_name()
            .unwrap_or_default(),
    );
    let _ = fs::remove_dir_all(&central);
    copy_dir(&source.to_string_lossy(), &central.to_string_lossy());
    let mut installed = Vec::new();
    if target != "central" {
        if let Some(editor) = cfg.editors.iter().find(|e| e.id == target) {
            let target_path = Path::new(&editor.skill_dir).join(
                central
                    .file_name()
                    .unwrap_or_default(),
            );
            crate::installer::sync_to_editor(
                &central.to_string_lossy(),
                &target_path.to_string_lossy(),
                &cfg.sync_mode,
            );
            installed.push(InstallItem {
                name: central
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                editor: editor.name.clone(),
                status: "installed".into(),
            });
        }
    }
    let _ = fs::remove_dir_all(&tmp);
    Ok(InstallResult {
        success: true,
        count: installed.len(),
        installed,
    })
}

fn copy_dir(src: &str, dest: &str) {
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
                copy_dir(&sp.to_string_lossy(), &dp.to_string_lossy());
            } else if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                let _ = fs::copy(&sp, &dp);
            }
        }
    }
}

// ---------- 命令式增改 (D1) ----------

pub fn add_source(
    name: String,
    url: String,
    source_type: String,
    branch: String,
    subpath: String,
) -> Vec<MarketplaceSource> {
    let mut sources = load_sources();
    sources.push(MarketplaceSource {
        id: gen_id(),
        name,
        url,
        source_type,
        branch,
        subpath,
    });
    save_sources_pub(&sources);
    sources
}

pub fn update_source(
    id: String,
    name: String,
    url: String,
    source_type: String,
    branch: String,
    subpath: String,
) -> Vec<MarketplaceSource> {
    let mut sources = load_sources();
    if let Some(s) = sources.iter_mut().find(|x| x.id == id) {
        s.name = name;
        s.url = url;
        s.source_type = source_type;
        s.branch = branch;
        s.subpath = subpath;
    }
    save_sources_pub(&sources);
    sources
}

pub fn remove_source(id: String) -> Vec<MarketplaceSource> {
    let mut sources = load_sources();
    sources.retain(|s| s.id != id);
    save_sources_pub(&sources);
    sources
}

// ---------- A15 内容审计 ----------

pub fn audit_content(content: String) -> Vec<AuditIssue> {
    crate::audit::audit_content_pub(&content)
}
