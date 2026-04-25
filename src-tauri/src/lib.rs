mod handlers {
    pub mod project_handler;
}

mod services {
    pub mod app_service;
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
