use std::{os::windows::process::CommandExt, process::Command};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[tauri::command]
pub async fn get_git_branch(project_path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let output = Command::new("git")
            .current_dir(project_path)
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err("Bukan repository git.".into());
        }

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(branch)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_git_branches(project_path: String) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        Command::new("git")
            .current_dir(&project_path)
            .args(["fetch", "--all", "--prune"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .ok();

        let output = Command::new("git")
            .current_dir(project_path)
            .args(["branch", "-a"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut branches = stdout
            .lines()
            .map(|x| x.replace('*', "").trim().replace("remotes/origin/", ""))
            .filter(|x| !x.is_empty() && !x.contains("HEAD ->"))
            .collect::<Vec<_>>();

        branches.sort();
        branches.dedup();

        Ok(branches)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn git_fetch(project_path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let output = Command::new("git")
            .current_dir(project_path)
            .args(["fetch", "--all", "--prune"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok("👷 Mandor narik update branch.".into())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn git_checkout(project_path: String, branch: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let output = Command::new("git")
            .current_dir(&project_path)
            .args(["switch", &branch])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            return Ok(format!("🌿 Pindah ke branch {}", branch));
        }

        let output2 = Command::new("git")
            .current_dir(&project_path)
            .args([
                "switch",
                "-c",
                &branch,
                "--track",
                &format!("origin/{}", branch),
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| e.to_string())?;

        if !output2.status.success() {
            return Err(String::from_utf8_lossy(&output2.stderr).to_string());
        }

        Ok(format!("🌿 Branch {} dipasang dari remote.", branch))
    })
    .await
    .map_err(|e| e.to_string())?
}
