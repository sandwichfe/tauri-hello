use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use trash;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    name: String,
    path: String,
    size: u64,
    is_dir: bool,
    modified_time: String,
}

#[tauri::command]
pub fn read_directory(path: String) -> Result<Vec<FileInfo>, String> {
    let dir = fs::read_dir(&path).map_err(|e| e.to_string())?;

    let mut files = Vec::new();
    for entry in dir {
        if let Ok(entry) = entry {
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            let modified = metadata.modified().map_err(|e| e.to_string())?;
            let modified_time: DateTime<Local> = modified.into();

            files.push(FileInfo {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: entry.path().to_string_lossy().into_owned(),
                size: metadata.len(),
                is_dir: metadata.is_dir(),
                modified_time: modified_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            });
        }
    }

    Ok(files)
}

#[tauri::command]
pub fn move_to_recycle_bin(path: String) -> Result<(), String> {
    trash::delete(path).map_err(|e| e.to_string())
}
