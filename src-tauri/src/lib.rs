// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::process::Command; // 引入 `Command` 来执行外部命令



#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // 注册函数 用于js调用
        .invoke_handler(tauri::generate_handler![greet,my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command()-> String {
  println!("I was invoked from JavaScript!");
  


    // 调用 Windows 命令，比如执行 `echo` 命令
    let output = Command::new("cmd")
        .args(&["/C", "chcp 65001 && ipconfig"])  // /C 代表运行后关闭命令窗口
        .output()  // 获取命令的输出
        .expect("Failed to execute command");
    // 打印命令的输出
    if !output.stdout.is_empty() {
        println!("Command Output: {}", String::from_utf8_lossy(&output.stdout));
    }

    let ret = String::from_utf8_lossy(&output.stdout);

    return String::from(ret);
}
