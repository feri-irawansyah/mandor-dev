use std::process::Command;

use crate::services::migration_service::{find_latest_migration, run_pwsh};

#[tauri::command]
pub fn run_add_migration(
    project_path: String,
    name: String,
    tags: Vec<String>,
) -> Result<String, String> {
    let safe_name = name.replace('"', "");

    let tag_command = if tags.is_empty() {
        String::new()
    } else {
        let joined = tags
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(",");

        format!(" -Tags {}", joined)
    };

    let ps_command = format!(
        r#"
        . .\.script\fm.ps1;
        Add-Migration -Name "{}"{}
        "#,
        safe_name, tag_command
    );

    let output = Command::new("pwsh")
        .current_dir(&project_path)
        .args([
            "-ExecutionPolicy",
            "Bypass",
            "-NoProfile",
            "-Command",
            &ps_command,
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);

        return Err(format!(
            "💥 Aduh bos, kerjaan gagal.\n👷 Error nih...\n\n{}",
            err
        ));
    }

    let file = find_latest_migration(&project_path, &safe_name);

    match file {
        Some(path) => Ok(format!(
        "👷 Mandor mulai kerja...\n🔨 Tukang lagi masang bata...\n✅ Kerjaan beres bos!\n📄 {}\n",
        path.display()
    )),
        None => Ok("👷 Mandor mulai kerja...\n✅ Kerjaan beres bos!".into()),
    }
}

#[tauri::command]
pub fn run_show_migration(project_path: String, tags: Vec<String>) -> Result<String, String> {
    let tag_command = if tags.is_empty() {
        String::new()
    } else {
        let joined = tags
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(",");

        format!(" -Tags {}", joined)
    };

    let ps_command = format!(
        r#"
        . .\.script\fm.ps1;
        Show-Migration{} | Out-String
        "#,
        tag_command
    );

    let output = Command::new("pwsh")
        .current_dir(&project_path)
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
        return Err(format!("💥 Mandor gagal buka map proyek.\n\n{}", stderr));
    }

    if stdout.is_empty() {
        return Ok("👷 Mandor udah nyari, tapi daftar migration kosong bos.".into());
    }

    Ok(format!("👷 Mandor lagi buka map proyek...\n\n{}", stdout))
}

#[tauri::command]
pub fn run_up_migration(
    project_path: String,
    tags: Vec<String>,
    target: Option<String>,
) -> Result<String, String> {
    let tag_command = if tags.is_empty() {
        String::new()
    } else {
        let joined = tags
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(",");

        format!(" -Tags {}", joined)
    };

    let target_command = match target {
        Some(x) if !x.trim().is_empty() => {
            format!(" -Target {}", x)
        }
        _ => String::new(),
    };

    let ps_command = format!(
        r#"
        . .\.script\fm.ps1;
        Deploy-Migration{}{} | Out-String
        "#,
        tag_command, target_command
    );

    run_pwsh(
        project_path,
        ps_command,
        "👷 Mandor nyuruh tukang kerja...",
        "💥 Gagal deploy bos.",
    )
}

#[tauri::command]
pub fn run_down_migration(
    project_path: String,
    tags: Vec<String>,
    step: i32,
    target: Option<String>,
    all: bool,
) -> Result<String, String> {
    let tag_command = if tags.is_empty() {
        String::new()
    } else {
        let joined = tags
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(",");

        format!(" -Tags {}", joined)
    };

    let mode = if all {
        " -All".to_string()
    } else if let Some(x) = target {
        if !x.trim().is_empty() {
            format!(" -Target {}", x)
        } else {
            format!(" -Step {}", step)
        }
    } else {
        format!(" -Step {}", step)
    };

    let ps_command = format!(
        r#"
        . .\.script\fm.ps1;
        Undo-Migration{}{} | Out-String
        "#,
        tag_command, mode
    );

    run_pwsh(
        project_path,
        ps_command,
        "👷 Mandor nyuruh bongkar kerjaan lama...",
        "💥 Gagal rollback bos.",
    )
}
