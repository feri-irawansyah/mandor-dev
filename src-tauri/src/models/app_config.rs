use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub last_project: Option<String>,
    pub recent_projects: Vec<String>,
    pub theme: String,
    pub dotnet_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub loaded: bool,
    pub children: Vec<FileNode>,
}
