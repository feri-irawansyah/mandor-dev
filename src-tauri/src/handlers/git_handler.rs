use std::process::Command;

#[tauri::command]
pub fn get_git_branch(project_path: String) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(project_path)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("Bukan repository git.".into());
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(branch)
}

#[tauri::command]
pub fn get_git_branches(project_path: String) -> Result<Vec<String>, String> {
    Command::new("git")
        .current_dir(&project_path)
        .args(["fetch", "--all", "--prune"])
        .output()
        .ok();

    let output = Command::new("git")
        .current_dir(project_path)
        .args(["branch", "-a"])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let branches = stdout
        .lines()
        .map(|x| x.replace('*', "").trim().replace("remotes/origin/", ""))
        .filter(|x| !x.is_empty() && x != "HEAD -> origin/main")
        .collect::<Vec<_>>();

    Ok(branches)
}

#[tauri::command]
pub fn git_fetch(project_path: String) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(project_path)
        .args(["fetch", "--all", "--prune"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok("👷 Mandor narik update branch.".into())
}

#[tauri::command]
pub fn git_checkout(project_path: String, branch: String) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(&project_path)
        .args(["switch", &branch])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        return Ok(format!("🌿 Pindah ke branch {}", branch));
    }

    // fallback remote branch
    let output2 = Command::new("git")
        .current_dir(&project_path)
        .args([
            "switch",
            "-c",
            &branch,
            "--track",
            &format!("origin/{}", branch),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output2.status.success() {
        return Err(String::from_utf8_lossy(&output2.stderr).to_string());
    }

    Ok(format!("🌿 Branch {} dipasang dari remote.", branch))
}
