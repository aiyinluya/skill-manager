use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::models::Editor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub editors: Vec<Editor>,
    pub sync_mode: String,
    pub theme: String,
    pub central_skills_dir: String,
}

pub fn home_dir() -> PathBuf {
    if let Ok(p) = std::env::var("USERPROFILE") {
        return PathBuf::from(p);
    }
    if let Ok(p) = std::env::var("HOME") {
        return PathBuf::from(p);
    }
    PathBuf::from(".")
}

pub fn data_dir() -> PathBuf {
    home_dir().join(".skill-manager")
}

pub fn central_skills_dir() -> PathBuf {
    data_dir().join("central-skills")
}

pub fn config_file() -> PathBuf {
    data_dir().join("config.json")
}

pub fn scenarios_file() -> PathBuf {
    data_dir().join("scenarios.json")
}

pub fn default_editors() -> Vec<Editor> {
    let home = home_dir();
    let e = |id: &str, name: &str, icon: &str, sub: &str, color: &str| Editor {
        id: id.to_string(),
        name: name.to_string(),
        icon: icon.to_string(),
        skill_dir: home.join(sub).join("skills").to_string_lossy().into_owned(),
        color: color.to_string(),
    };
    vec![
        e("claude", "Claude Code", "🤖", ".claude", "#D97706"),
        e("codex", "Codex CLI", "⚡", ".codex", "#059669"),
        e("cursor", "Cursor", "🖱", ".cursor", "#7C3AED"),
        e("workbuddy", "WorkBuddy", "🛠", ".workbuddy", "#2563EB"),
        e("agents", "Agents", "🌐", ".agents", "#DC2626"),
        e("gemini", "Gemini CLI", "💎", ".gemini", "#0891B2"),
        e("windsurf", "Windsurf", "🏄", ".windsurf", "#0D9488"),
        e("opencode", "OpenCode", "📦", ".opencode", "#9333EA"),
        e("copilot", "GitHub Copilot", "🐙", ".copilot", "#6B7280"),
        e("augment", "Augment", "🔧", ".augment", "#F59E0B"),
        e("trae", "Trae", "🚀", ".trae", "#EF4444"),
        e("kiro", "Kiro", "🦊", ".kiro", "#8B5CF6"),
        e("codebuddy", "CodeBuddy", "🤖", ".codebuddy", "#3B82F6"),
        e("aider", "Aider", "🤝", ".aider", "#10B981"),
    ]
}

pub fn ensure_data_dir() {
    let d = data_dir();
    if !d.exists() {
        let _ = fs::create_dir_all(&d);
    }
    let c = central_skills_dir();
    if !c.exists() {
        let _ = fs::create_dir_all(&c);
    }
}

pub fn load_config() -> AppConfig {
    ensure_data_dir();
    let cf = config_file();
    if cf.exists() {
        if let Ok(s) = fs::read_to_string(&cf) {
            if let Ok(cfg) = serde_json::from_str::<AppConfig>(&s) {
                return cfg;
            }
        }
    }
    let default = AppConfig {
        editors: default_editors(),
        sync_mode: "symlink".to_string(),
        theme: "dark".to_string(),
        central_skills_dir: central_skills_dir().to_string_lossy().into_owned(),
    };
    save_config(&default);
    default
}

pub fn save_config(cfg: &AppConfig) {
    ensure_data_dir();
    if let Ok(s) = serde_json::to_string_pretty(cfg) {
        let _ = fs::write(config_file(), s);
    }
}

pub fn load_scenarios() -> Vec<crate::models::Scenario> {
    ensure_data_dir();
    let sf = scenarios_file();
    if sf.exists() {
        if let Ok(s) = fs::read_to_string(&sf) {
            if let Ok(sc) = serde_json::from_str::<Vec<crate::models::Scenario>>(&s) {
                return sc;
            }
        }
    }
    vec![]
}

pub fn save_scenarios(scenarios: &Vec<crate::models::Scenario>) {
    ensure_data_dir();
    if let Ok(s) = serde_json::to_string_pretty(scenarios) {
        let _ = fs::write(scenarios_file(), s);
    }
}
