use std::{fs, path::PathBuf, process::Command, time::SystemTime};

pub fn find_latest_migration(project_path: &str, name: &str) -> Option<PathBuf> {
    let dir = PathBuf::from(project_path)
        .join("src")
        .join("Atlas.Migrations");

    let mut latest: Option<(SystemTime, PathBuf)> = None;

    let entries = fs::read_dir(dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();

        let file_name = path.file_name()?.to_string_lossy();

        if file_name.ends_with(&format!("_{}.cs", name)) {
            let modified = entry.metadata().ok()?.modified().ok()?;

            match &latest {
                Some((time, _)) if modified <= *time => {}
                _ => latest = Some((modified, path)),
            }
        }
    }

    latest.map(|(_, path)| path)
}

pub fn run_pwsh(
    project_path: String,
    ps_command: String,
    success_title: &str,
    fail_title: &str,
) -> Result<String, String> {
    let output = Command::new("pwsh")
        .current_dir(project_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &ps_command,
        ])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if !output.status.success() {
        return Err(format!("{}\n\n{}", fail_title, stderr));
    }

    Ok(format!("{}\n\n{}", success_title, stdout))
}
