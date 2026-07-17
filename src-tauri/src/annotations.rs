use std::collections::HashMap;
use std::fs;

use crate::config::{data_dir, ensure_data_dir};
use crate::models::Annotation;

pub fn annotations_file() -> std::path::PathBuf {
    data_dir().join("annotations.json")
}

pub fn load_annotations() -> HashMap<String, Annotation> {
    ensure_data_dir();
    let f = annotations_file();
    if f.exists() {
        if let Ok(s) = fs::read_to_string(&f) {
            if let Ok(m) = serde_json::from_str::<HashMap<String, Annotation>>(&s) {
                return m;
            }
        }
    }
    HashMap::new()
}

pub fn save_annotations(map: &HashMap<String, Annotation>) {
    ensure_data_dir();
    if let Ok(s) = serde_json::to_string_pretty(map) {
        let _ = fs::write(annotations_file(), s);
    }
}

#[tauri::command]
pub fn get_annotations() -> HashMap<String, Annotation> {
    load_annotations()
}

#[tauri::command]
pub fn set_annotation(skill_id: String, notes: String, importance: String) -> Annotation {
    let mut map = load_annotations();
    let ann = Annotation {
        notes,
        importance: if importance == "important" || importance == "critical" {
            importance
        } else {
            "normal".to_string()
        },
    };
    map.insert(skill_id.clone(), ann.clone());
    save_annotations(&map);
    ann
}

#[tauri::command]
pub fn delete_annotation(skill_id: String) {
    let mut map = load_annotations();
    map.remove(&skill_id);
    save_annotations(&map);
}
