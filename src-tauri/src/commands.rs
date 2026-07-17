use std::path::Path;
use crate::config::{load_config, save_config, load_scenarios, save_scenarios, AppConfig};
use crate::scanner::{scan_all, get_skill_detail, find_duplicates};
use crate::installer::{install_from_github, install_from_local, uninstall_skill, sync_to_editors};
use crate::audit::audit_skills;
use crate::project::{
    open_project, get_project_skills, set_project_skill, get_project_config,
    set_project_extra_paths, activate_project_skills, export_project_template,
    import_project_template, get_feedback_stats,
};
use crate::marketplace::{
    load_sources, add_source, update_source, remove_source, browse_marketplace,
    install_from_marketplace, audit_content,
};
use crate::annotations::{load_annotations, save_annotations};
use crate::models::*;

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn now_iso() -> String {
    crate::scanner::format_iso(now_ms() / 1000)
}

#[tauri::command]
pub fn get_editors() -> Vec<EditorWithCount> {
    let cfg = load_config();
    let (_all, per) = scan_all(&cfg.editors);
    cfg.editors
        .iter()
        .map(|e| {
            let exists = Path::new(&e.skill_dir).exists();
            let count = per.get(&e.id).map(|v| v.len()).unwrap_or(0);
            EditorWithCount {
                id: e.id.clone(),
                name: e.name.clone(),
                icon: e.icon.clone(),
                skill_dir: e.skill_dir.clone(),
                color: e.color.clone(),
                exists,
                skill_count: count,
            }
        })
        .collect()
}

#[tauri::command]
pub fn get_skills(editor_id: Option<String>, search: Option<String>) -> SkillList {
    let cfg = load_config();
    let (all, per) = scan_all(&cfg.editors);
    let mut skills = if let Some(eid) = &editor_id {
        if eid == "all" || eid.is_empty() {
            all
        } else {
            per.get(eid).cloned().unwrap_or_default()
        }
    } else {
        all
    };
    if let Some(q) = &search {
        if !q.is_empty() {
            let ql = q.to_lowercase();
            skills.retain(|s| {
                s.name.to_lowercase().contains(&ql)
                    || s.summary.to_lowercase().contains(&ql)
                    || s.triggers.iter().any(|t| t.to_lowercase().contains(&ql))
            });
        }
    }
    SkillList {
        total: skills.len(),
        skills,
    }
}

#[tauri::command]
pub fn get_skill_detail_cmd(editor_id: String, dir_name: String) -> Option<SkillDetail> {
    let cfg = load_config();
    get_skill_detail(&cfg.editors, &editor_id, &dir_name)
}

#[tauri::command]
pub fn get_stats() -> Stats {
    let cfg = load_config();
    let (all, per) = scan_all(&cfg.editors);
    let mut per_editor = std::collections::HashMap::new();
    for e in &cfg.editors {
        per_editor.insert(
            e.id.clone(),
            EditorStat {
                name: e.name.clone(),
                icon: e.icon.clone(),
                color: e.color.clone(),
                count: per.get(&e.id).map(|v| v.len()).unwrap_or(0),
            },
        );
    }
    Stats {
        total_skills: all.len(),
        total_editors: cfg.editors.len(),
        active_editors: cfg
            .editors
            .iter()
            .filter(|e| Path::new(&e.skill_dir).exists())
            .count(),
        per_editor,
        duplicates: find_duplicates(&all),
    }
}

#[tauri::command]
pub fn install_github(repo_url: String, target_editor_id: Option<String>) -> Result<InstallResult, String> {
    let cfg = load_config();
    let target = target_editor_id.unwrap_or_else(|| "central".into());
    install_from_github(&repo_url, &target, &cfg)
}

#[tauri::command]
pub fn install_local(local_path: String, target_editor_id: Option<String>) -> Result<InstallResult, String> {
    let cfg = load_config();
    let target = target_editor_id.unwrap_or_else(|| "central".into());
    install_from_local(&local_path, &target, &cfg)
}

#[tauri::command]
pub fn uninstall(editor_id: String, dir_name: String) -> Result<UninstallResult, String> {
    let cfg = load_config();
    uninstall_skill(&editor_id, &dir_name, &cfg)
}

#[tauri::command]
pub fn sync_skill(skill_name: String, target_editor_ids: Vec<String>) -> Result<SyncResult, String> {
    let cfg = load_config();
    sync_to_editors(&skill_name, &target_editor_ids, &cfg)
}

#[tauri::command]
pub fn get_scenarios() -> Vec<Scenario> {
    load_scenarios()
}

#[tauri::command]
pub fn create_scenario(
    name: String,
    description: String,
    skills: Vec<ScenarioSkill>,
) -> Scenario {
    let mut scenarios = load_scenarios();
    let sc = Scenario {
        id: format!("scn_{}", now_ms()),
        name,
        description,
        skills,
        created_at: now_iso(),
    };
    scenarios.push(sc.clone());
    save_scenarios(&scenarios);
    sc
}

#[tauri::command]
pub fn delete_scenario(id: String) {
    let scenarios = load_scenarios();
    let filtered: Vec<_> = scenarios.into_iter().filter(|s| s.id != id).collect();
    save_scenarios(&filtered);
}

#[tauri::command]
pub fn apply_scenario(id: String) -> Result<ApplyResult, String> {
    let scenarios = load_scenarios();
    let sc = scenarios.iter().find(|s| s.id == id).ok_or("场景不存在")?;
    let cfg = load_config();
    let mut results = Vec::new();
    for entry in &sc.skills {
        let src_editor = cfg.editors.iter().find(|e| e.id == entry.editor_id);
        let src_path = match src_editor {
            Some(e) => Path::new(&e.skill_dir).join(&entry.dir_name),
            None => continue,
        };
        if !src_path.exists() {
            continue;
        }
        for editor in &cfg.editors {
            if !Path::new(&editor.skill_dir).exists() {
                continue;
            }
            let target_path = Path::new(&editor.skill_dir).join(&entry.dir_name);
            crate::installer::sync_to_editor(
                &src_path.to_string_lossy(),
                &target_path.to_string_lossy(),
                &cfg.sync_mode,
            );
            results.push(ApplyItem {
                skill: entry.dir_name.clone(),
                editor: editor.name.clone(),
                status: "synced".into(),
            });
        }
    }
    Ok(ApplyResult {
        success: true,
        results,
    })
}

#[tauri::command]
pub fn audit_skills_cmd() -> Vec<AuditIssue> {
    let cfg = load_config();
    let (all, _per) = scan_all(&cfg.editors);
    audit_skills(&all)
}

#[tauri::command]
pub fn get_config() -> AppConfig {
    load_config()
}

#[tauri::command]
pub fn set_sync_mode(mode: String) -> Result<(), String> {
    let mut cfg = load_config();
    if mode != "symlink" && mode != "copy" {
        return Err("sync mode 必须是 symlink 或 copy".into());
    }
    cfg.sync_mode = mode;
    save_config(&cfg);
    Ok(())
}

#[tauri::command]
pub fn copy_skill(
    editor_id: String,
    dir_name: String,
    target_editor_ids: Vec<String>,
) -> Result<SyncResult, String> {
    let cfg = load_config();
    let src_editor = cfg
        .editors
        .iter()
        .find(|e| e.id == editor_id)
        .ok_or("编辑器不存在")?;
    let src_path = Path::new(&src_editor.skill_dir).join(&dir_name);
    if !src_path.exists() {
        return Err("源技能不存在".into());
    }
    let mut synced = Vec::new();
    for eid in target_editor_ids {
        if let Some(editor) = cfg.editors.iter().find(|e| e.id == eid) {
            let target_path = Path::new(&editor.skill_dir).join(&dir_name);
            crate::installer::sync_to_editor(
                &src_path.to_string_lossy(),
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

// ============ Tier B — 项目感知 ============

#[tauri::command]
pub fn open_project_cmd(path: String) -> ProjectInfo {
    open_project(path)
}

#[tauri::command]
pub fn get_project_skills_cmd(path: String) -> ProjectView {
    get_project_skills(path)
}

#[tauri::command]
pub fn set_project_skill_cmd(path: String, skill_key: String, enabled: bool) -> ProjectConfig {
    set_project_skill(path, skill_key, enabled)
}

#[tauri::command]
pub fn get_project_config_cmd(path: String) -> ProjectConfig {
    get_project_config(path)
}

#[tauri::command]
pub fn set_project_extra_paths_cmd(path: String, extra_paths: Vec<String>) -> ProjectConfig {
    set_project_extra_paths(path, extra_paths)
}

#[tauri::command]
pub fn activate_project_skills_cmd(path: String, project_id: Option<String>) -> ActivateResult {
    activate_project_skills(path, project_id)
}

#[tauri::command]
pub fn export_project_template_cmd(path: String) -> String {
    export_project_template(path)
}

#[tauri::command]
pub fn import_project_template_cmd(path: String, template_json: String) -> ProjectConfig {
    import_project_template(path, template_json)
}

#[tauri::command]
pub fn get_feedback_stats_cmd() -> FeedbackStats {
    get_feedback_stats()
}

// ============ Tier D — 技能应用商店 ============

#[tauri::command]
pub fn get_marketplace_sources_cmd() -> Vec<MarketplaceSource> {
    load_sources()
}

#[tauri::command]
pub fn add_marketplace_source_cmd(
    name: String,
    url: String,
    source_type: String,
    branch: String,
    subpath: String,
) -> Vec<MarketplaceSource> {
    add_source(name, url, source_type, branch, subpath)
}

#[tauri::command]
pub fn update_marketplace_source_cmd(
    id: String,
    name: String,
    url: String,
    source_type: String,
    branch: String,
    subpath: String,
) -> Vec<MarketplaceSource> {
    update_source(id, name, url, source_type, branch, subpath)
}

#[tauri::command]
pub fn remove_marketplace_source_cmd(id: String) -> Vec<MarketplaceSource> {
    remove_source(id)
}

#[tauri::command]
pub fn browse_marketplace_cmd(refresh: bool) -> Vec<MarketplaceSkill> {
    browse_marketplace(refresh)
}

#[tauri::command]
pub fn install_from_marketplace_cmd(
    source_id: String,
    dir_name: String,
    target_editor_id: Option<String>,
) -> Result<InstallResult, String> {
    install_from_marketplace(source_id, dir_name, target_editor_id)
}

#[tauri::command]
pub fn audit_content_cmd(content: String) -> Vec<AuditIssue> {
    audit_content(content)
}

#[tauri::command]
pub fn read_text_file_cmd(path: String) -> String {
    std::fs::read_to_string(&path).unwrap_or_default()
}

// ============ A17 — 导入/导出归档 ============

#[tauri::command]
pub fn export_archive_cmd() -> String {
    let archive = serde_json::json!({
        "version": 1,
        "kind": "skill-manager-archive",
        "config": load_config(),
        "annotations": load_annotations(),
        "scenarios": load_scenarios(),
        "marketplace": load_sources(),
    });
    serde_json::to_string_pretty(&archive).unwrap_or_else(|_| "{}".into())
}

#[tauri::command]
pub fn import_archive_cmd(json: String) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("JSON 解析失败: {}", e))?;
    if v.get("kind").and_then(|x| x.as_str()) != Some("skill-manager-archive") {
        return Err("不是有效的归档文件".into());
    }
    if let Some(cfg) = v.get("config") {
        if let Ok(c) = serde_json::from_value::<AppConfig>(cfg.clone()) {
            save_config(&c);
        }
    }
    if let Some(ann) = v.get("annotations") {
        if let Ok(m) = serde_json::from_value::<std::collections::HashMap<String, Annotation>>(ann.clone()) {
            save_annotations(&m);
        }
    }
    if let Some(sc) = v.get("scenarios") {
        if let Ok(s) = serde_json::from_value::<Vec<Scenario>>(sc.clone()) {
            save_scenarios(&s);
        }
    }
    if let Some(mp) = v.get("marketplace") {
        if let Ok(s) = serde_json::from_value::<Vec<MarketplaceSource>>(mp.clone()) {
            crate::marketplace::save_sources_pub(&s);
        }
    }
    Ok("归档已导入".into())
}
