use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Duration;
use sysinfo::{System, Pid, Signal};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tokio::time::interval;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub cpu_usage: f32,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_top_cpu_processes() -> Result<Vec<ProcessInfo>, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // 等待一秒再次刷新以获得更准确的CPU使用率
    tokio::time::sleep(Duration::from_secs(1)).await;
    sys.refresh_processes();

    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            name: process.name().to_string(),
            pid: pid.as_u32(),
            cpu_usage: process.cpu_usage(),
        })
        .collect();

    // 按CPU使用率降序排列
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    // 取前10个
    processes.truncate(10);

    Ok(processes)
}

#[tauri::command]
async fn terminate_process(pid: u32) -> Result<String, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        let process_name = process.name().to_string();

        if process.kill_with(Signal::Term).is_some() {
            Ok(format!("已成功终止进程: {} (PID: {})", process_name, pid))
        } else {
            Err(format!("无法终止进程: {} (PID: {})", process_name, pid))
        }
    } else {
        Err(format!("找不到PID为 {} 的进程", pid))
    }
}

#[tauri::command]
async fn force_kill_process(pid: u32) -> Result<String, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        let process_name = process.name().to_string();

        if process.kill_with(Signal::Kill).is_some() {
            Ok(format!("已强制终止进程: {} (PID: {})", process_name, pid))
        } else {
            Err(format!("无法强制终止进程: {} (PID: {})", process_name, pid))
        }
    } else {
        Err(format!("找不到PID为 {} 的进程", pid))
    }
}

#[tauri::command]
async fn restart_process(process_name: String) -> Result<String, String> {
    // 尝试重启进程（这是一个简化的实现）
    // 注意：重启进程在macOS上比较复杂，这里提供基本实现

    // 首先尝试通过 open 命令启动应用程序
    let result = Command::new("open")
        .arg("-a")
        .arg(&process_name)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(format!("尝试重启应用程序: {}", process_name))
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                Err(format!("重启失败: {}", error))
            }
        }
        Err(e) => Err(format!("无法重启进程 {}: {}", process_name, e))
    }
}

fn generate_tooltip_text(processes: &[ProcessInfo]) -> String {
    let mut tooltip = "🖥️ CPU监控器 - 前10进程:\n\n".to_string();

    for (i, process) in processes.iter().enumerate() {
        tooltip.push_str(&format!(
            "{}. {} ({}): {:.1}%\n",
            i + 1,
            process.name,
            process.pid,
            process.cpu_usage
        ));
    }

    if processes.is_empty() {
        tooltip.push_str("暂无进程数据");
    }

    tooltip
}

async fn update_tray_info(app_handle: AppHandle) {
    let mut interval = interval(Duration::from_secs(3));

    loop {
        interval.tick().await;

        if let Ok(processes) = get_top_cpu_processes().await {
            if let Some(tray) = app_handle.tray_by_id("main-tray") {
                if let Some(top_process) = processes.first() {
                    // 限制进程名称长度，避免托盘标题过长
                    let process_name = if top_process.name.len() > 12 {
                        format!("{}...", &top_process.name[..9])
                    } else {
                        top_process.name.clone()
                    };

                    // 设置托盘图标标题为最高CPU占用的进程
                    let title = format!("{}: {:.1}%", process_name, top_process.cpu_usage);
                    let _ = tray.set_title(Some(&title));

                    // 设置详细的工具提示
                    let tooltip_text = generate_tooltip_text(&processes);
                    let _ = tray.set_tooltip(Some(tooltip_text));
                } else {
                    let _ = tray.set_title(Some("CPU监控器"));
                    let _ = tray.set_tooltip(Some("暂无进程数据"));
                }
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_top_cpu_processes,
            terminate_process,
            force_kill_process,
            restart_process
        ])
        .setup(|app| {
            let app_handle = app.app_handle().clone();

            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("CPU监控器 - 加载中...")
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(move |tray, event| {
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            println!("托盘图标被左键点击！");
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        TrayIconEvent::Click {
                            button: MouseButton::Right,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            println!("右键点击托盘图标，退出程序");
                            let app = tray.app_handle();
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // 延迟启动后台任务，避免初始化问题
            let app_handle_clone = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                // 等待应用完全启动
                tokio::time::sleep(Duration::from_secs(2)).await;
                update_tray_info(app_handle_clone).await;
            });

            // 隐藏主窗口，只在托盘中运行
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
