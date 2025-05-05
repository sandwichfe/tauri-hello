// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use reqwest::Client;
mod file_system;
use file_system::read_directory;
use tauri::command;

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

#[tauri::command]
async fn fetch_ffmpeg_file(url: String) -> Result<Vec<u8>, String> {
    let client = Client::new();
    let res = client.get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    if !res.status().is_success() {
        return Err(format!("HTTP请求失败: {}", res.status()));
    }
    
    res.bytes()
        .await
        .map_err(|e| e.to_string())
        .map(|b| b.to_vec())
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![my_custom_command, read_directory, read_binary_file,fetch_ffmpeg_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
