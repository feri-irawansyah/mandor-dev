use tauri::AppHandle;

use crate::{
    models::app_config::{AppConfig, FileNode},
    services::{app_service, project_service},
};

#[tauri::command]
pub fn import_project(app: AppHandle, path: String) -> Result<(), String> {
    app_service::save_last_project(&app, &path)?;

    Ok(())
}

#[tauri::command]
pub fn get_project_tree(path: String) -> Result<Vec<FileNode>, String> {
    Ok(project_service::read_folder(std::path::Path::new(&path)))
}

#[tauri::command]
pub fn get_folder_children(path: String) -> Result<Vec<FileNode>, String> {
    Ok(project_service::read_folder(std::path::Path::new(&path)))
}

#[tauri::command]
pub fn get_app_config(app: AppHandle) -> Result<AppConfig, String> {
    app_service::get_config(&app)
}
