use crate::handlers::frontend_handler::ProcessState;

mod handlers {
    pub mod backend_handler;
    pub mod frontend_handler;
    pub mod migration_handler;
    pub mod project_handler;
}

mod services {
    pub mod app_service;
    pub mod migration_service;
    pub mod project_service;
}

mod models {
    pub mod app_config;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handlers::project_handler::import_project,
            handlers::project_handler::get_project_tree,
            handlers::project_handler::get_app_config,
            handlers::project_handler::get_folder_children,
            handlers::migration_handler::run_add_migration,
            handlers::migration_handler::run_show_migration,
            handlers::migration_handler::run_up_migration,
            handlers::migration_handler::run_down_migration,
            handlers::frontend_handler::run_frontend,
            handlers::frontend_handler::stop_frontend,
            handlers::frontend_handler::check_frontend_status,
            handlers::backend_handler::run_backend,
            handlers::backend_handler::stop_backend,
            handlers::backend_handler::check_backend_status
        ])
        .manage(ProcessState {
            frontend: std::sync::Mutex::new(None),
            backend: std::sync::Mutex::new(None),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
