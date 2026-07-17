use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editor {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub skill_dir: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub dir_name: String,
    pub summary: String,
    pub triggers: Vec<String>,
    pub editor_id: String,
    pub editor_name: String,
    pub editor_icon: String,
    pub editor_color: String,
    pub skill_dir: String,
    pub file_path: Option<String>,
    pub file_size: u64,
    pub file_size_formatted: String,
    pub related_files: Vec<String>,
    pub frontmatter: serde_json::Value,
    pub content: String,
    pub is_valid: bool,
    pub installed_at: Option<String>,
    pub is_symlink: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDetail {
    pub editor_id: String,
    pub editor_name: String,
    pub editor_icon: String,
    pub dir_name: String,
    pub skill_dir: String,
    pub exists: bool,
    pub name: String,
    pub summary: String,
    pub triggers: Vec<String>,
    pub frontmatter: serde_json::Value,
    pub content: String,
    pub file_size: u64,
    pub file_size_formatted: String,
    pub related_files: Vec<String>,
    pub all_files: Vec<FileInfo>,
    pub is_valid: bool,
    pub is_symlink: bool,
    pub symlink_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub size_formatted: String,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub name: String,
    pub description: String,
    pub skills: Vec<ScenarioSkill>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioSkill {
    pub editor_id: String,
    pub dir_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateLocation {
    pub editor_id: String,
    pub editor_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditIssue {
    pub skill_id: String,
    pub skill_name: String,
    pub severity: String,
    pub issue_type: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorWithCount {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub skill_dir: String,
    pub color: String,
    pub exists: bool,
    pub skill_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorStat {
    pub name: String,
    pub icon: String,
    pub color: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_skills: usize,
    pub total_editors: usize,
    pub active_editors: usize,
    pub per_editor: HashMap<String, EditorStat>,
    pub duplicates: HashMap<String, Vec<DuplicateLocation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillList {
    pub total: usize,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallItem {
    pub name: String,
    pub editor: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    pub success: bool,
    pub installed: Vec<InstallItem>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UninstallResult {
    pub success: bool,
    pub removed: String,
    pub was_symlink: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncItem {
    pub editor: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub synced: Vec<SyncItem>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyItem {
    pub skill: String,
    pub editor: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResult {
    pub success: bool,
    pub results: Vec<ApplyItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub notes: String,
    pub importance: String, // normal | important | critical
}

// ============ Tier B — 项目感知 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub path: String,
    pub name: String,
    pub project_id: String,
    pub has_config: bool,
    pub config_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSkill {
    pub skill: Skill,
    pub layer: String,       // project | recommended | available
    pub source: String,      // project | global
    pub signals: Vec<String>, // 可解释的相关性信号
    pub enabled: bool,       // 有效状态（覆盖或默认）
    pub default_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub version: u8,
    pub project_id: Option<String>,
    pub extra_paths: Vec<String>,
    pub overrides: std::collections::HashMap<String, String>, // skill_key -> enable|disable
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectView {
    pub info: ProjectInfo,
    pub stack: Vec<String>,                  // 检测到的技术栈标签
    pub project: Vec<ProjectSkill>,          // 区① 本项目 skills
    pub recommended: Vec<ProjectSkill>,      // 区② 推荐用于本项目
    pub available: Vec<ProjectSkill>,        // 区③ 全局其他（用不上）
    pub overrides: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateResult {
    pub success: bool,
    pub activated: Vec<String>,
    pub count: usize,
}

// ============ Tier D — 技能应用商店 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSource {
    pub id: String,
    pub name: String,
    pub url: String,        // github: repo url; local: 目录; remote: 索引 URL
    pub source_type: String, // github | local | remote
    pub branch: String,
    pub subpath: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSkill {
    pub source_id: String,
    pub source_name: String,
    pub name: String,
    pub dir_name: String,
    pub summary: String,
    pub triggers: Vec<String>,
    pub skill_dir: String, // 本地缓存/目录中的实际路径
    pub installed: bool,
    pub update_available: bool,
    pub extra: serde_json::Value, // remote 索引附带的附加字段
}

// ============ Tier E3 — 反馈闭环 ============

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeedbackStats {
    pub total_events: usize,
    pub enabled: usize,
    pub disabled: usize,
    pub per_project: std::collections::HashMap<String, usize>,
}
