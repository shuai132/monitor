use serde::{Deserialize, Serialize};
use std::time::Duration;
use sysinfo::System;
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

async fn update_tray_tooltip(app_handle: AppHandle) {
    let mut interval = interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        if let Ok(processes) = get_top_cpu_processes().await {
            let tooltip_text = generate_tooltip_text(&processes);

            if let Some(tray) = app_handle.tray_by_id("main-tray") {
                let _ = tray.set_tooltip(Some(tooltip_text));
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_top_cpu_processes])
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
                update_tray_tooltip(app_handle_clone).await;
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
