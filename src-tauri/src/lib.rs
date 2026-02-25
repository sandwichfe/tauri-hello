// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
mod file_system;
use file_system::read_directory;
use tauri::{AppHandle, Manager};

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
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            read_directory,
            convert_video_ffmpeg
        ])
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
                std::thread::sleep(std::time::Duration::from_millis(600));
                
                // 确保启动画面保持在前台
                splashscreen.set_focus().unwrap();
                
                // 模拟资源加载过程 - 给主窗口更多时间准备内容
                std::thread::sleep(std::time::Duration::from_millis(100));
                
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

#[tauri::command]
fn convert_video_ffmpeg(
    app: AppHandle,
    input: String,
    output: String,
    convert_type: String,
    quality: String,
) -> Result<String, String> {
    let mut args: Vec<String> = Vec::new();
    args.push("-y".to_string());
    args.push("-i".to_string());
    args.push(input.clone());

    match convert_type.as_str() {
        "avi-to-mp4" => {
            let (preset, crf) = match quality.as_str() {
                "high" => ("slow", "18"),
                "low" => ("fast", "28"),
                _ => ("medium", "23"),
            };
            args.push("-c:v".to_string());
            args.push("libx264".to_string());
            args.push("-preset".to_string());
            args.push(preset.to_string());
            args.push("-crf".to_string());
            args.push(crf.to_string());
            args.push("-c:a".to_string());
            args.push("aac".to_string());
            args.push("-b:a".to_string());
            args.push("192k".to_string());
        }
        "m3u8-to-mp4" => {
            args.push("-c".to_string());
            args.push("copy".to_string());
            args.push("-bsf:a".to_string());
            args.push("aac_adtstoasc".to_string());
        }
        other => {
            return Err(format!("不支持的转换类型: {}", other));
        }
    }

    args.push(output.clone());

    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?;

    let ffmpeg_path: PathBuf = resource_dir
        .join("resources")
        .join("ffmpeg")
        .join("ffmpeg.exe");

    if !ffmpeg_path.is_file() {
        return Err(
            "未找到 ffmpeg 可执行文件，请确认 src-tauri/resources/ffmpeg/ffmpeg.exe 是否存在".to_string(),
        );
    }

    let output_result = Command::new(ffmpeg_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    let stderr_text = String::from_utf8_lossy(&output_result.stderr).to_string();
    let stdout_text = String::from_utf8_lossy(&output_result.stdout).to_string();

    if output_result.status.success() {
        let combined = if stderr_text.trim().is_empty() {
            stdout_text
        } else if stdout_text.trim().is_empty() {
            stderr_text
        } else {
            format!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout_text, stderr_text)
        };
        Ok(combined)
    } else {
        let code = output_result.status.code().unwrap_or(-1);
        let message = if stderr_text.trim().is_empty() {
            stdout_text
        } else {
            stderr_text
        };
        Err(format!("ffmpeg 执行失败，退出码 {}:\n{}", code, message))
    }
}

