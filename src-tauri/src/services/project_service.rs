use std::{fs, path::Path};

use crate::models::app_config::FileNode;

const IGNORE: &[&str] = &[".git", "node_modules", "bin", "obj", "dist", "target"];

pub fn read_folder(path: &Path) -> Vec<FileNode> {
    let mut nodes = vec![];

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path_buf = entry.path();

            let name = entry.file_name().to_string_lossy().to_string();

            if IGNORE.contains(&name.as_str()) {
                continue;
            }

            let is_dir = path_buf.is_dir();

            nodes.push(FileNode {
                name,
                path: path_buf.display().to_string(),
                is_dir,
                loaded: false,
                children: vec![],
            });
        }
    }

    // folder di atas file
    nodes.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    nodes
}
