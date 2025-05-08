// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio}; 
mod file_system;
use file_system::read_directory;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, my_custom_command, read_directory])
        .setup(|app| {
            // 先获取启动画面窗口，确保它优先显示
            let splashscreen = app.get_webview_window("splashscreen").expect("无法获取启动窗口");
            let main_window = app.get_webview_window("main").expect("无法获取主窗口");
            
            // 确保启动画面立即显示并置于前台
            splashscreen.show().unwrap();
            splashscreen.set_focus().unwrap();
            
            // 确保主窗口初始隐藏
            main_window.hide().unwrap();
            
            // 延迟关闭启动画面并显示主窗口
            std::thread::spawn(move || {
                // 等待启动画面完全加载并显示
                std::thread::sleep(std::time::Duration::from_millis(500));
                
                // 确保启动画面保持在前台
                splashscreen.set_focus().unwrap();
                
                // 模拟资源加载过程 - 给主窗口更多时间准备内容
                std::thread::sleep(std::time::Duration::from_secs(1));
                
                // 关闭启动画面并显示主窗口
                splashscreen.close().unwrap();
                main_window.show().unwrap();
                main_window.set_focus().unwrap();
            });
            
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行应用失败");
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

