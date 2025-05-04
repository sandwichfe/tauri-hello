// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
mod file_system;
use file_system::read_directory;

#[tauri::command]
async fn my_custom_command(command: String) -> Result<String, String> {
    let output = Command::new("cmd")
        .args(["/C", &command])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
async fn read_binary_file(path: String) -> Result<Vec<u8>, String> {
    println!("正在读取文件: {}", path);
    let result = std::fs::read(&path)
        .map_err(|e| {
            println!("文件读取错误: {}", e);
            e.to_string()
        });
    if let Ok(data) = &result {
        println!("成功读取文件: {}, 大小: {} 字节", path, data.len());
    }
    result
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![my_custom_command, read_directory, read_binary_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
