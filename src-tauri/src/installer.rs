use std::fs;
use std::os::windows::fs::symlink_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::config::AppConfig;
use crate::models::{InstallItem, InstallResult, SyncItem, SyncResult, UninstallResult};

pub fn install_from_github(
    repo_url: &str,
    target_editor_id: &str,
    cfg: &AppConfig,
) -> Result<InstallResult, String> {
    let url = normalize_github_url(repo_url);
    let (user, repo, subpath, branch) = parse_github_url(&url)?;
    let tmp = std::env::temp_dir().join(format!("sm-clone-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let clone_url = format!("https://github.com/{}/{}.git", user, repo);
    let tmp_str = tmp.to_string_lossy().into_owned();

    let out = Command::new("git")
        .args(["clone", "--depth", "1", "--branch", &branch, &clone_url, &tmp_str])
        .output();

    match out {
        Ok(o) if o.status.success() => {}
        Ok(o) => {
            let o2 = Command::new("git")
                .args(["clone", "--depth", "1", "--branch", "master", &clone_url, &tmp_str])
                .output();
            match o2 {
                Ok(o2) if o2.status.success() => {}
                _ => {
                    let err = String::from_utf8_lossy(&o.stderr);
                    let _ = fs::remove_dir_all(&tmp);
                    return Err(format!("Git clone 失败: {}", err));
                }
            }
        }
        Err(e) => {
            let _ = fs::remove_dir_all(&tmp);
            return Err(format!("未找到 git 命令: {}", e));
        }
    }

    let source = if subpath.is_empty() {
        tmp_str.clone()
    } else {
        Path::new(&tmp_str).join(&subpath).to_string_lossy().into_owned()
    };

    let skill_dirs = find_skill_dirs(&source);
    if skill_dirs.is_empty() {
        let _ = fs::remove_dir_all(&tmp);
        return Err("仓库中未找到任何 SKILL.md".to_string());
    }

    let central = PathBuf::from(&cfg.central_skills_dir);
    let mut installed = Vec::new();
    for sd in skill_dirs {
        let name = Path::new(&sd)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let central_path = central.join(&name);
        let _ = fs::remove_dir_all(&central_path);
        copy_dir(&sd, &central_path.to_string_lossy());
        if !target_editor_id.is_empty() && target_editor_id != "central" {
            if let Some(editor) = cfg.editors.iter().find(|e| e.id == target_editor_id) {
                let target_path = Path::new(&editor.skill_dir).join(&name);
                sync_to_editor(
                    &central_path.to_string_lossy(),
                    &target_path.to_string_lossy(),
                    &cfg.sync_mode,
                );
                installed.push(InstallItem {
                    name,
                    editor: editor.name.clone(),
                    status: "installed".into(),
                });
            }
        } else {
            installed.push(InstallItem {
                name,
                editor: "central library".into(),
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

pub fn install_from_local(
    local_path: &str,
    target_editor_id: &str,
    cfg: &AppConfig,
) -> Result<InstallResult, String> {
    let src = Path::new(local_path);
    if !src.exists() {
        return Err(format!("路径不存在: {}", local_path));
    }
    let stat = fs::metadata(src).map_err(|e| e.to_string())?;
    let mut skill_dirs = Vec::new();
    if stat.is_dir() {
        if src.join("SKILL.md").exists() {
            skill_dirs.push(src.to_string_lossy().into_owned());
        } else {
            skill_dirs = find_skill_dirs(&src.to_string_lossy());
        }
    } else {
        return Err("本地路径必须是目录".into());
    }
    if skill_dirs.is_empty() {
        return Err("本地路径中未找到 SKILL.md".into());
    }
    let central = PathBuf::from(&cfg.central_skills_dir);
    let mut installed = Vec::new();
    for sd in skill_dirs {
        let name = Path::new(&sd)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let central_path = central.join(&name);
        let _ = fs::remove_dir_all(&central_path);
        copy_dir(&sd, &central_path.to_string_lossy());
        if !target_editor_id.is_empty() && target_editor_id != "central" {
            if let Some(editor) = cfg.editors.iter().find(|e| e.id == target_editor_id) {
                let target_path = Path::new(&editor.skill_dir).join(&name);
                sync_to_editor(
                    &central_path.to_string_lossy(),
                    &target_path.to_string_lossy(),
                    &cfg.sync_mode,
                );
                installed.push(InstallItem {
                    name,
                    editor: editor.name.clone(),
                    status: "installed".into(),
                });
            }
        } else {
            installed.push(InstallItem {
                name,
                editor: "central library".into(),
                status: "installed".into(),
            });
        }
    }
    Ok(InstallResult {
        success: true,
        count: installed.len(),
        installed,
    })
}

pub fn uninstall_skill(
    editor_id: &str,
    dir_name: &str,
    cfg: &AppConfig,
) -> Result<UninstallResult, String> {
    let editor = cfg
        .editors
        .iter()
        .find(|e| e.id == editor_id)
        .ok_or_else(|| format!("未找到编辑器: {}", editor_id))?;
    let skill_path = Path::new(&editor.skill_dir).join(dir_name);
    if !skill_path.exists() {
        return Err(format!("未找到技能: {}", skill_path.to_string_lossy()));
    }
    let lstat = fs::symlink_metadata(&skill_path).map_err(|e| e.to_string())?;
    if lstat.file_type().is_symlink() {
        let _ = fs::remove_file(&skill_path);
        return Ok(UninstallResult {
            success: true,
            removed: dir_name.to_string(),
            was_symlink: true,
        });
    }
    cleanup_dir(&skill_path);
    Ok(UninstallResult {
        success: true,
        removed: dir_name.to_string(),
        was_symlink: false,
    })
}

pub fn sync_to_editors(
    skill_name: &str,
    target_editor_ids: &[String],
    cfg: &AppConfig,
) -> Result<SyncResult, String> {
    let source = PathBuf::from(&cfg.central_skills_dir).join(skill_name);
    if !source.exists() {
        return Err(format!("central 库中未找到技能: {}", skill_name));
    }
    let mut synced = Vec::new();
    for eid in target_editor_ids {
        if let Some(editor) = cfg.editors.iter().find(|e| e.id == *eid) {
            let target_path = Path::new(&editor.skill_dir).join(skill_name);
            sync_to_editor(
                &source.to_string_lossy(),
                &target_path.to_string_lossy(),
                &cfg.sync_mode,
            );
            synced.push(SyncItem {
                editor: editor.name.clone(),
                status: "synced".into(),
            });
        }
    }
    Ok(SyncResult {
        success: true,
        count: synced.len(),
        synced,
    })
}

pub fn sync_to_editor(source: &str, target: &str, mode: &str) {
    let target_path = Path::new(target);
    if target_path.exists() {
        if let Ok(ls) = fs::symlink_metadata(target_path) {
            if ls.file_type().is_symlink() {
                let _ = fs::remove_file(target_path);
            } else {
                cleanup_dir(target_path);
            }
        }
    }
    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }
    if mode == "symlink" {
        if let Err(e) = symlink_dir(source, target) {
            eprintln!("[sync] 符号链接失败，回退为复制: {}", e);
            copy_dir(source, target);
        }
    } else {
        copy_dir(source, target);
    }
}

fn find_skill_dirs(dir: &str) -> Vec<String> {
    let mut results = Vec::new();
    fn scan(d: &Path, results: &mut Vec<String>) {
        let skill_md = d.join("SKILL.md");
        if skill_md.exists() {
            results.push(d.to_string_lossy().into_owned());
            return;
        }
        if let Ok(entries) = fs::read_dir(d) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with('.') || name == "node_modules" {
                        continue;
                    }
                    scan(&entry.path(), results);
                }
            }
        }
    }
    scan(Path::new(dir), &mut results);
    results
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

fn cleanup_dir(dir: &Path) {
    if !dir.exists() {
        return;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                cleanup_dir(&p);
            } else {
                let _ = fs::remove_file(&p);
            }
        }
    }
    let _ = fs::remove_dir(dir);
}

fn normalize_github_url(url: &str) -> String {
    let s = url.trim();
    if s.starts_with("http") || s.starts_with("git@") {
        return s.to_string();
    }
    format!("https://github.com/{}", s)
}

pub fn parse_github_url_pub(url: &str) -> Result<(String, String, String, String), String> {
    parse_github_url(url)
}

fn parse_github_url(url: &str) -> Result<(String, String, String, String), String> {
    let s = url
        .trim()
        .replace("git@github.com:", "")
        .replace("https://", "")
        .replace("http://", "")
        .replace("git@", "")
        .replace("github.com/", "");
    let parts: Vec<&str> = s.split('/').filter(|p| !p.is_empty()).collect();
    if parts.len() < 2 {
        return Err("无效的 GitHub URL: 期望 user/repo 格式".into());
    }
    let repo = parts[1].trim_end_matches(".git").to_string();
    let user = parts[0].to_string();
    let mut branch = "main".to_string();
    let mut subpath = String::new();
    if parts.len() > 3 && parts[2] == "tree" {
        branch = parts[3].to_string();
        subpath = parts[4..].join("/");
    } else if parts.len() > 2 {
        subpath = parts[2..].join("/");
    }
    Ok((user, repo, subpath, branch))
}
