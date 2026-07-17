pub mod models;
pub mod config;
pub mod parser;
pub mod scanner;
pub mod installer;
pub mod audit;
pub mod project;
pub mod marketplace;
pub mod commands;
pub mod annotations;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_editors,
            commands::get_skills,
            commands::get_skill_detail_cmd,
            commands::get_stats,
            commands::install_github,
            commands::install_local,
            commands::uninstall,
            commands::sync_skill,
            commands::get_scenarios,
            commands::create_scenario,
            commands::delete_scenario,
            commands::apply_scenario,
            commands::audit_skills_cmd,
            commands::get_config,
            commands::set_sync_mode,
            commands::copy_skill,
            annotations::get_annotations,
            annotations::set_annotation,
            annotations::delete_annotation,
            commands::open_project_cmd,
            commands::get_project_skills_cmd,
            commands::set_project_skill_cmd,
            commands::get_project_config_cmd,
            commands::set_project_extra_paths_cmd,
            commands::activate_project_skills_cmd,
            commands::export_project_template_cmd,
            commands::import_project_template_cmd,
            commands::get_feedback_stats_cmd,
            commands::get_marketplace_sources_cmd,
            commands::add_marketplace_source_cmd,
            commands::update_marketplace_source_cmd,
            commands::remove_marketplace_source_cmd,
            commands::browse_marketplace_cmd,
            commands::install_from_marketplace_cmd,
            commands::audit_content_cmd,
            commands::read_text_file_cmd,
            commands::export_archive_cmd,
            commands::import_archive_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
