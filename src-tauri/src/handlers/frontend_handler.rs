use std::{
    io::{BufRead, BufReader},
    os::windows::process::CommandExt,
    process::{Child, Command, Stdio},
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};

use tauri::{AppHandle, Emitter, State};

use crate::services::app_service::strip_ansi;

pub struct ProcessState {
    pub frontend: Mutex<Option<Child>>,
    pub backend: Mutex<Option<Child>>,
}

fn pipe_logs<R: std::io::Read + Send + 'static>(reader: R, app: AppHandle, event: &'static str) {
    thread::spawn(move || {
        let reader = BufReader::new(reader);

        for line in reader.lines().flatten() {
            let clean = strip_ansi(&line);
            let _ = app.emit(event, clean);
        }
    });
}

#[tauri::command]
pub fn run_frontend(
    app: AppHandle,
    state: State<ProcessState>,
    project_path: String,
) -> Result<String, String> {
    {
        let lock = state.frontend.lock().map_err(|e| e.to_string())?;

        if lock.is_some() {
            return Ok("👷 Frontend udah kerja dari tadi bos.".into());
        }
    }

    let mut child = Command::new("cmd")
        .args(["/C", "pnpm dev"])
        .current_dir(project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()
        .map_err(|e| e.to_string())?;

    if let Some(stdout) = child.stdout.take() {
        pipe_logs(stdout, app.clone(), "frontend-log");
    }

    if let Some(stderr) = child.stderr.take() {
        pipe_logs(stderr, app.clone(), "frontend-log");
    }

    let mut lock = state.frontend.lock().map_err(|e| e.to_string())?;
    *lock = Some(child);

    Ok("👷 Frontend mulai kerja bos.".into())
}

#[tauri::command]
pub fn stop_frontend(state: State<ProcessState>) -> Result<String, String> {
    let mut lock = state.frontend.lock().map_err(|e| e.to_string())?;

    if let Some(child) = lock.as_mut() {
        let pid = child.id();

        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| e.to_string())?;

        let start = Instant::now();

        while start.elapsed() < Duration::from_secs(3) {
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) => thread::sleep(Duration::from_millis(150)),
                Err(_) => break,
            }
        }
    }

    *lock = None;

    Ok("🛑 Frontend dibubarin bos.".into())
}

#[tauri::command]
pub fn check_frontend_status(state: State<ProcessState>) -> Result<bool, String> {
    let mut lock = state.frontend.lock().map_err(|e| e.to_string())?;

    if let Some(child) = lock.as_mut() {
        match child.try_wait() {
            Ok(None) => Ok(true),
            Ok(Some(_)) => {
                *lock = None;
                Ok(false)
            }
            Err(e) => {
                *lock = None;
                Err(e.to_string())
            }
        }
    } else {
        Ok(false)
    }
}
