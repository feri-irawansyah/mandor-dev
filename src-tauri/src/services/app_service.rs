use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::models::app_config::AppConfig;

pub fn get_config(app: &AppHandle) -> Result<AppConfig, String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;

    let config = store
        .get("config")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    Ok(config)
}

pub fn save_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;

    store.set("config", serde_json::to_value(config).unwrap());

    store.save().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn save_last_project(app: &AppHandle, path: &str) -> Result<(), String> {
    let mut config = get_config(app)?;

    config.last_project = Some(path.to_string());

    save_config(app, &config)
}

pub fn strip_ansi(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            while let Some(n) = chars.next() {
                if n == 'm' || n == 'K' {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}
