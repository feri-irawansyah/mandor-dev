use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::Mutex,
    thread::{self, sleep},
    time::Duration,
};

use tauri::{AppHandle, Emitter, State};

use crate::services::app_service::strip_ansi;

pub struct ProcessState {
    pub frontend: Mutex<Option<Child>>,
    pub backend: Mutex<Option<Child>>,
}

#[tauri::command]
pub fn run_frontend(
    app: AppHandle,
    state: State<ProcessState>,
    project_path: String,
) -> Result<String, String> {
    let mut lock = state.frontend.lock().unwrap();

    if lock.is_some() {
        return Ok("Frontend udah jalan bos.".into());
    }

    let mut child = Command::new("cmd")
        .args(["/C", "pnpm dev"])
        .current_dir(project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let app_clone = app.clone();

    thread::spawn(move || {
        let reader = BufReader::new(stdout);

        for line in reader.lines().flatten() {
            let clean = strip_ansi(&line);
            let _ = app_clone.emit("frontend-log", clean);
        }
    });

    let app_clone = app.clone();

    thread::spawn(move || {
        let reader = BufReader::new(stderr);

        for line in reader.lines().flatten() {
            let clean = strip_ansi(&line);
            let _ = app_clone.emit("frontend-log", clean);
        }
    });

    *lock = Some(child);

    Ok("👷 Frontend mulai kerja bos.".into())
}

#[tauri::command]
pub fn stop_frontend(state: State<ProcessState>) -> Result<String, String> {
    let mut lock = state.frontend.lock().unwrap();

    if let Some(child) = lock.as_mut() {
        let pid = child.id();

        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .output()
            .map_err(|e| e.to_string())?;

        // kasih waktu windows beberes
        sleep(Duration::from_millis(1200));
    }

    *lock = None;

    Ok("🛑 Frontend dibubarin bos.".into())
}

#[tauri::command]
pub fn check_frontend_status(state: State<ProcessState>) -> Result<bool, String> {
    let mut lock = state.frontend.lock().unwrap();

    if let Some(child) = lock.as_mut() {
        match child.try_wait() {
            Ok(None) => {
                // masih hidup
                return Ok(true);
            }
            Ok(Some(_)) => {
                // sudah mati
                *lock = None;
                return Ok(false);
            }
            Err(e) => {
                *lock = None;
                return Err(e.to_string());
            }
        }
    }

    Ok(false)
}
