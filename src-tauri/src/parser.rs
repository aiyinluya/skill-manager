use std::fs;
use std::path::Path;
use serde_json::{Map, Value};

pub struct ParsedSkill {
    pub name: String,
    pub summary: String,
    pub triggers: Vec<String>,
    pub frontmatter: Value,
    pub content: String,
    pub file_size: u64,
    pub related_files: Vec<String>,
    pub is_valid: bool,
}

pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{} B", bytes);
    } else if bytes < 1024 * 1024 {
        return format!("{:.1} KB", bytes as f64 / 1024.0);
    }
    format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
}

pub fn parse_skill_md(path: &Path) -> Option<ParsedSkill> {
    let content = fs::read_to_string(path).ok()?;
    Some(parse_skill_content(&content, path))
}

pub fn parse_skill_content(content: &str, path: &Path) -> ParsedSkill {
    let mut result = ParsedSkill {
        name: String::new(),
        summary: String::new(),
        triggers: Vec::new(),
        frontmatter: Value::Object(Map::new()),
        content: content.to_string(),
        file_size: 0,
        related_files: Vec::new(),
        is_valid: false,
    };

    if let Ok(meta) = fs::metadata(path) {
        result.file_size = meta.len();
    }

    if let Some((fm_text, body)) = split_frontmatter(content) {
        result.frontmatter = parse_simple_yaml(&fm_text);
        result.content = body.trim().to_string();
        result.is_valid = true;
        if let Some(obj) = result.frontmatter.as_object() {
            result.name = obj
                .get("name")
                .or_else(|| obj.get("title"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            result.summary = obj
                .get("summary")
                .or_else(|| obj.get("description"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
        }
        result.triggers = extract_triggers(&fm_text, &body);
    } else {
        if let Some(caps) = content.lines().find_map(|l| l.strip_prefix("# ")) {
            result.name = caps.trim().to_string();
        }
        if result.name.is_empty() {
            if let Some(parent) = path.parent() {
                result.name = parent
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
            }
        }
        result.is_valid = true;
    }

    if let Some(parent) = path.parent() {
        if let Ok(entries) = fs::read_dir(parent) {
            for e in entries.flatten() {
                let fname = e.file_name().to_string_lossy().into_owned();
                if (fname != "SKILL.md" && fname != "skill.md") && !fname.starts_with('.') {
                    result.related_files.push(fname);
                }
            }
        }
    }

    result
}

/// Split a leading YAML frontmatter block delimited by `---` lines.
fn split_frontmatter(content: &str) -> Option<(String, String)> {
    let stripped = content.strip_prefix("---")?;
    let pos = stripped.find("\n---")?;
    let fm = &stripped[..pos];
    let rest = &stripped[pos + 4..];
    Some((fm.to_string(), rest.to_string()))
}

/// Minimal YAML parser: flat key/value plus simple `- item` arrays.
fn parse_simple_yaml(text: &str) -> Value {
    let mut map: Map<String, Value> = Map::new();
    let mut current_array: Option<Vec<Value>> = None;
    let mut current_key: Option<String> = None;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("- ") {
            if let (Some(arr), Some(key)) = (&mut current_array, &current_key) {
                arr.push(Value::String(strip_quotes(rest.trim())));
                map.insert(key.clone(), Value::Array(arr.clone()));
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix('-') {
            let r = rest.trim();
            if !r.is_empty() {
                if let (Some(arr), Some(key)) = (&mut current_array, &current_key) {
                    arr.push(Value::String(strip_quotes(r)));
                    map.insert(key.clone(), Value::Array(arr.clone()));
                }
            }
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_string();
            let val = strip_quotes(v.trim());
            if val.is_empty() {
                current_array = Some(Vec::new());
                current_key = Some(key.clone());
                map.insert(key, Value::Array(vec![]));
            } else {
                current_array = None;
                current_key = None;
                map.insert(key, Value::String(val));
            }
        }
    }
    Value::Object(map)
}

fn strip_quotes(s: &str) -> String {
    let s = s.trim();
    if s.len() >= 2
        && ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')))
    {
        let mut chars: Vec<char> = s.chars().collect();
        chars.remove(0);
        chars.pop();
        return chars.into_iter().collect();
    }
    s.to_string()
}

fn extract_triggers(fm_text: &str, body: &str) -> Vec<String> {
    let mut triggers = Vec::new();
    let patterns = ["read_when", "trigger", "triggers", "when_to_use"];
    let lower = fm_text.to_lowercase();
    for p in patterns {
        let pat = format!("{}:", p);
        if let Some(pos) = lower.find(&pat) {
            let from = &fm_text[pos + pat.len()..];
            if let Some(line_end) = from.find('\n') {
                let line = &from[..line_end];
                for part in line.split(|c| c == ',' || c == '，' || c == '、' || c == '|') {
                    let t = part.trim().trim_matches(|c| c == '"' || c == '\'').to_string();
                    if !t.is_empty() {
                        triggers.push(t);
                    }
                }
            }
        }
    }
    // fallback: scan body for trigger keywords
    if triggers.is_empty() {
        for line in body.lines() {
            let l = line.to_lowercase();
            if l.contains("触发词") || l.contains("trigger") || l.contains("when to use") {
                if let Some(sep) = line.find(|c| c == ':' || c == '：') {
                    let rest = &line[sep + 1..];
                    for part in rest.split(|c| c == ',' || c == '，' || c == '、' || c == '|') {
                        let t = part.trim().to_string();
                        if !t.is_empty() {
                            triggers.push(t);
                        }
                    }
                }
            }
        }
    }
    triggers
}
