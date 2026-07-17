use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;
use crate::config::AppConfig;
use crate::models::{DuplicateLocation, Editor, FileInfo, Skill, SkillDetail};
use crate::parser::{format_size, parse_skill_md};

/// Recursively find SKILL.md files. Continues into subdirectories even after
/// finding one, so nested skills (e.g. gstack/autoplan) are discovered too.
pub fn find_skill_md_files(dir: &Path, base_dir: &Path, results: &mut Vec<(PathBuf, String)>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut skill_md: Option<PathBuf> = None;
    let mut subdirs: Vec<PathBuf> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if is_dir {
            if name == "node_modules" || name == ".git" {
                continue;
            }
            subdirs.push(path);
        } else if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
            if name == "SKILL.md" || name == "skill.md" {
                skill_md = Some(path);
            }
        }
    }

    if let Some(p) = skill_md {
        let rel = path_diff(dir, base_dir);
        results.push((p, rel));
    }

    for sub in subdirs {
        find_skill_md_files(&sub, base_dir, results);
    }
}

fn path_diff(dir: &Path, base: &Path) -> String {
    dir.strip_prefix(base)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| dir.to_string_lossy().replace('\\', "/"))
}

fn is_symlink(p: &Path) -> bool {
    fs::symlink_metadata(p)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

/// Format a UNIX timestamp (seconds) as a basic UTC ISO-8601 string.
pub fn format_iso(secs: u64) -> String {
    const DAY: u64 = 86400;
    let days = secs / DAY;
    let sec_of_day = secs % DAY;
    let hour = sec_of_day / 3600;
    let min = (sec_of_day % 3600) / 60;
    let sec = sec_of_day % 60;

    let mut y = 1970u32;
    let mut d = days;
    loop {
        let leap = if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
            366
        } else {
            365
        };
        if d < leap {
            break;
        }
        d -= leap;
        y += 1;
    }
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mut m = 0usize;
    let mut rem = d;
    loop {
        let md = month_days[m] + if m == 1 && leap { 1 } else { 0 };
        if rem < md {
            break;
        }
        rem -= md;
        m += 1;
    }
    let day = rem + 1;
    let month = m + 1;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, month, day, hour, min, sec
    )
}

pub fn scan_editor_skills(editor: &Editor) -> Vec<Skill> {
    let mut skills = Vec::new();
    let skill_dir = Path::new(&editor.skill_dir);
    if !skill_dir.exists() {
        return skills;
    }

    let mut found: Vec<(PathBuf, String)> = Vec::new();
    find_skill_md_files(skill_dir, skill_dir, &mut found);

    for (skill_md_path, rel_path) in &found {
        let sdir = skill_md_path.parent().unwrap_or(skill_dir);
        if let Some(parsed) = parse_skill_md(skill_md_path) {
            let installed_at = fs::metadata(sdir)
                .ok()
                .and_then(|m| m.modified().ok())
                .map(|t| {
                    let secs = t
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    format_iso(secs)
                });
            let name = if parsed.name.is_empty() {
                sdir.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            } else {
                parsed.name.clone()
            };
            let is_sym = is_symlink(sdir);
            skills.push(Skill {
                id: format!("{}:{}", editor.id, rel_path),
                name,
                dir_name: rel_path.clone(),
                summary: parsed.summary,
                triggers: parsed.triggers,
                editor_id: editor.id.clone(),
                editor_name: editor.name.clone(),
                editor_icon: editor.icon.clone(),
                editor_color: editor.color.clone(),
                skill_dir: sdir.to_string_lossy().into_owned(),
                file_path: Some(skill_md_path.to_string_lossy().into_owned()),
                file_size: parsed.file_size,
                file_size_formatted: format_size(parsed.file_size),
                related_files: parsed.related_files,
                frontmatter: parsed.frontmatter,
                content: if parsed.content.chars().count() > 500 {
                    parsed.content.chars().take(500).collect::<String>()
                } else {
                    parsed.content.clone()
                },
                is_valid: parsed.is_valid,
                installed_at,
                is_symlink: is_sym,
            });
        }
    }
    skills
}

pub fn scan_all(editors: &[Editor]) -> (Vec<Skill>, HashMap<String, Vec<Skill>>) {
    let mut all = Vec::new();
    let mut per: HashMap<String, Vec<Skill>> = HashMap::new();
    for e in editors {
        let sk = scan_editor_skills(e);
        per.insert(e.id.clone(), sk.clone());
        all.extend(sk);
    }
    (all, per)
}

pub fn get_skill_detail(
    editors: &[Editor],
    editor_id: &str,
    dir_name: &str,
) -> Option<SkillDetail> {
    let editor = editors.iter().find(|e| e.id == editor_id)?;
    let skill_path = Path::new(&editor.skill_dir).join(dir_name);
    if !skill_path.exists() {
        return None;
    }
    let skill_md = skill_path.join("SKILL.md");
    let mut detail = SkillDetail {
        editor_id: editor.id.clone(),
        editor_name: editor.name.clone(),
        editor_icon: editor.icon.clone(),
        dir_name: dir_name.to_string(),
        skill_dir: skill_path.to_string_lossy().into_owned(),
        exists: true,
        name: skill_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        summary: String::new(),
        triggers: Vec::new(),
        frontmatter: Value::Object(serde_json::Map::new()),
        content: String::new(),
        file_size: 0,
        file_size_formatted: "—".to_string(),
        related_files: Vec::new(),
        all_files: Vec::new(),
        is_valid: false,
        is_symlink: is_symlink(&skill_path),
        symlink_target: None,
    };
    if detail.is_symlink {
        if let Ok(target) = fs::read_link(&skill_path) {
            detail.symlink_target = Some(target.to_string_lossy().into_owned());
        }
    }
    if skill_md.exists() {
        if let Some(parsed) = parse_skill_md(&skill_md) {
            if !parsed.name.is_empty() {
                detail.name = parsed.name;
            }
            detail.summary = parsed.summary;
            detail.triggers = parsed.triggers;
            detail.frontmatter = parsed.frontmatter;
            detail.content = parsed.content;
            detail.file_size = parsed.file_size;
            detail.file_size_formatted = format_size(parsed.file_size);
            detail.related_files = parsed.related_files;
            detail.is_valid = parsed.is_valid;
        }
    }
    detail.all_files = list_files_recursive(&skill_path, &skill_path);
    Some(detail)
}

pub fn list_files_recursive(dir: &Path, base: &Path) -> Vec<FileInfo> {
    let mut files = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return files,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let rel = path
            .strip_prefix(base)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| name.clone());
        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                if name == "node_modules" || name == ".git" {
                    continue;
                }
                files.extend(list_files_recursive(&path, base));
            } else if ft.is_file() {
                let meta = fs::metadata(&path).ok();
                let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                let modified = meta
                    .and_then(|m| m.modified().ok())
                    .map(|t| {
                        let secs = t
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0);
                        format_iso(secs)
                    })
                    .unwrap_or_default();
                files.push(FileInfo {
                    name: name.clone(),
                    path: rel,
                    size,
                    size_formatted: format_size(size),
                    modified,
                });
            }
        }
    }
    files
}

pub fn find_duplicates(skills: &[Skill]) -> HashMap<String, Vec<DuplicateLocation>> {
    let mut by_name: HashMap<String, Vec<DuplicateLocation>> = HashMap::new();
    for s in skills {
        by_name.entry(s.dir_name.clone()).or_default().push(DuplicateLocation {
            editor_id: s.editor_id.clone(),
            editor_name: s.editor_name.clone(),
        });
    }
    by_name.into_iter().filter(|(_, v)| v.len() > 1).collect()
}

#[allow(dead_code)]
pub fn app_config_ref(_cfg: &AppConfig) {}
