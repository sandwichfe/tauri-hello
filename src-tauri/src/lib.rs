// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio}; // 引入 `Command` 来执行外部命令

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // 注册函数 用于js调用
        .invoke_handler(tauri::generate_handler![greet, my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command(command: String) -> String {
    // 构建命令行字符串，设置编码为 UTF-8
    let full_command = format!(" {}", command);

    // 执行传入的命令
    let output = Command::new("cmd")
        .creation_flags(0x08000000) // 隐藏运行cmd
        .arg("/C") // 让命令执行完后退出
        .arg(&full_command) // 借用 full_command 的引用
        .stdout(Stdio::piped()) // 捕获输出
        .output() // 执行命令并获取输出
        .expect("Failed to execute command");

    // 打印传入的命令（可选）
    println!("Command Input: {}", full_command);

    // 打印命令的输出（如果有输出）
    if !output.stdout.is_empty() {
        println!(
            "Command Output: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    // 返回命令的输出作为字符串
    String::from_utf8_lossy(&output.stdout).to_string()
}
