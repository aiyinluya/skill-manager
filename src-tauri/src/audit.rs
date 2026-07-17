use crate::models::{AuditIssue, Skill};

/// Scan installed skills for risky patterns before/after install.
pub fn audit_skills(skills: &[Skill]) -> Vec<AuditIssue> {
    let mut issues = Vec::new();
    for s in skills {
        if let Some(fp) = &s.file_path {
            if let Ok(content) = std::fs::read_to_string(fp) {
                check_patterns(s, &content, &mut issues);
            }
        }
    }
    issues
}

/// 仅对一段 SKILL 内容做静态安全审计（用于商店安装前预览 / A15）。
pub fn audit_content_pub(content: &str) -> Vec<AuditIssue> {
    let mut issues = Vec::new();
    // 构造一个轻量 Skill 占位（审计只看 content 文本）
    let placeholder = Skill {
        id: String::new(),
        name: String::new(),
        dir_name: String::new(),
        summary: String::new(),
        triggers: Vec::new(),
        editor_id: String::new(),
        editor_name: String::new(),
        editor_icon: String::new(),
        editor_color: String::new(),
        skill_dir: String::new(),
        file_path: None,
        file_size: 0,
        file_size_formatted: String::new(),
        related_files: Vec::new(),
        frontmatter: serde_json::Value::Null,
        content: String::new(),
        is_valid: true,
        installed_at: None,
        is_symlink: false,
    };
    check_patterns(&placeholder, content, &mut issues);
    issues
}

fn check_patterns(skill: &Skill, content: &str, issues: &mut Vec<AuditIssue>) {
    let patterns: &[(&str, &str, &str)] = &[
        (r"curl\s+https?://\S+\s*\|\s*(sh|bash)", "high", "通过 curl|sh 下载并执行"),
        (r"wget\s+https?://\S+\s*\|\s*(sh|bash)", "high", "通过 wget|sh 下载并执行"),
        (r"rm\s+-rf\s+/", "high", "对根目录进行破坏性递归删除"),
        (r"eval\s*\(", "medium", "动态代码执行 (eval)"),
        (r"(?i)\b(sudo|runas)\b", "medium", "尝试提权"),
        (r"(?i)(api_key|secret|token|password)\s*[:=]", "medium", "可能存在硬编码凭证"),
        (r#"https?://[^\s'"`]+"#, "low", "存在网络外联端点"),
    ];
    for (pat, severity, desc) in patterns {
        if let Ok(re) = regex::Regex::new(pat) {
            if re.is_match(content) {
                issues.push(AuditIssue {
                    skill_id: skill.id.clone(),
                    skill_name: skill.name.clone(),
                    severity: severity.to_string(),
                    issue_type: desc.to_string(),
                    detail: format!("命中模式: {}", pat),
                });
            }
        }
    }
}
