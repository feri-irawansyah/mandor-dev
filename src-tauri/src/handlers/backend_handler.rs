use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use tauri::{Emitter, State};

use crate::handlers::frontend_handler::ProcessState;

#[tauri::command]
pub fn run_backend(
    app: tauri::AppHandle,
    state: State<ProcessState>,
    project_path: String,
) -> Result<String, String> {
    let mut lock = state.backend.lock().unwrap();

    if lock.is_some() {
        return Ok("🟢 Backend udah kerja bos.".into());
    }

    let mut child = Command::new("dotnet")
        .arg("run")
        .current_dir(project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    if let Some(stdout) = child.stdout.take() {
        let app_clone = app.clone();

        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);

            for line in reader.lines().flatten() {
                let _ = app_clone.emit("backend-log", line);
            }
        });
    }

    if let Some(stderr) = child.stderr.take() {
        let app_clone = app.clone();

        std::thread::spawn(move || {
            let reader = BufReader::new(stderr);

            for line in reader.lines().flatten() {
                let _ = app_clone.emit("backend-log", line);
            }
        });
    }

    *lock = Some(child);

    Ok("👷 Mandor nyuruh backend kerja.".into())
}

#[tauri::command]
pub fn stop_backend(state: State<ProcessState>) -> Result<String, String> {
    let mut lock = state.backend.lock().unwrap();

    if let Some(child) = lock.as_mut() {
        let pid = child.id();

        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .output()
            .map_err(|e| e.to_string())?;

        std::thread::sleep(std::time::Duration::from_millis(1200));
    }

    *lock = None;

    Ok("🛑 Backend dibubarin bos.".into())
}

#[tauri::command]
pub fn check_backend_status(state: State<ProcessState>) -> Result<bool, String> {
    let mut lock = state.backend.lock().unwrap();

    if let Some(child) = lock.as_mut() {
        match child.try_wait() {
            Ok(None) => return Ok(true),
            Ok(Some(_)) => {
                *lock = None;
                return Ok(false);
            }
            Err(_) => {
                *lock = None;
                return Ok(false);
            }
        }
    }

    Ok(false)
}
