use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Duration;
use sysinfo::{System, Pid, Signal};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent, Position, LogicalPosition,
};
use tokio::time::interval;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub cpu_usage: f32,
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

fn create_tray_popup(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::{WebviewWindowBuilder, WebviewUrl};

    // 获取屏幕尺寸来计算位置
    let popup_width = 420.0;
    let popup_height = 600.0;

    // 获取主显示器的尺寸
    let (screen_width, screen_height) = get_screen_size();

    // 获取托盘位置并计算弹窗位置
    let (x, y) = calculate_tray_popup_position(screen_width, screen_height, popup_width, popup_height);

    println!("Screen size: {}x{}, Popup position: ({}, {})", screen_width, screen_height, x, y);

    let window = WebviewWindowBuilder::new(&app, "tray-popup", WebviewUrl::App("index.html".into()))
        .title("CPU监控器 - 托盘弹窗")
        .inner_size(popup_width, popup_height)
        .position(x, y)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .skip_taskbar(true)
        .always_on_top(true)
        .decorations(false)  // 无边框窗口
        .shadow(true)        // 添加阴影
        .build()?;

    // 暂时移除原生圆角设置，避免运行时错误
    // 圆角效果将通过CSS实现

    // 添加失焦隐藏功能
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            let _ = window_clone.hide();
        }
    });

    Ok(())
}

#[cfg(target_os = "macos")]
fn get_tray_icon_position() -> Option<(f64, f64)> {
    // 在macOS上，托盘图标通常位于屏幕右上角
    // 由于无法直接获取托盘图标的确切位置，我们使用估算
    let (screen_width, _) = get_screen_size();

    // macOS菜单栏高度通常是24像素
    let menu_bar_height = 24.0;

    // 托盘图标通常在右侧，我们估算一个位置
    // 假设托盘图标宽度约22像素，距离右边缘有一些边距
    let estimated_tray_x = screen_width - 100.0; // 估算托盘图标位置
    let estimated_tray_y = menu_bar_height / 2.0;

    Some((estimated_tray_x, estimated_tray_y))
}

#[cfg(not(target_os = "macos"))]
fn get_tray_icon_position() -> Option<(f64, f64)> {
    None
}

fn calculate_tray_popup_position(screen_width: f64, screen_height: f64, popup_width: f64, popup_height: f64) -> (f64, f64) {
    let margin = 8.0;
    let menu_bar_height = 24.0;

    // 尝试获取托盘图标位置
    let (tray_x, _tray_y) = get_tray_icon_position().unwrap_or((screen_width - 50.0, menu_bar_height / 2.0));

    // 计算弹窗的Y位置 - 紧贴菜单栏下方
    let y = menu_bar_height + margin;

    // 计算弹窗的X位置
    let x = if tray_x + popup_width + margin <= screen_width {
        // 如果托盘图标右侧有足够空间，左对齐到托盘图标
        tray_x
    } else {
        // 如果右侧空间不够，右对齐到屏幕边缘
        screen_width - popup_width - margin
    };

    // 确保弹窗不会超出屏幕边界
    let x = x.max(margin).min(screen_width - popup_width - margin);
    let y = y.max(margin).min(screen_height - popup_height - margin);

    println!("托盘位置估算: ({}, {}), 弹窗最终位置: ({}, {})", tray_x, _tray_y, x, y);

    (x, y)
}

#[cfg(target_os = "macos")]
fn get_screen_size() -> (f64, f64) {
    use core_graphics::display::{CGDisplay, CGDirectDisplayID};

    // 获取主显示器
    let display_id: CGDirectDisplayID = CGDisplay::main().id;
    let display = CGDisplay::new(display_id);

    // 获取显示器尺寸
    let width = display.pixels_wide() as f64;
    let height = display.pixels_high() as f64;

    (width, height)
}

#[cfg(not(target_os = "macos"))]
fn get_screen_size() -> (f64, f64) {
    // 其他平台的默认值
    (1920.0, 1080.0)
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

                            // 检查是否已经有托盘弹窗存在
                            if let Some(popup_window) = app.get_webview_window("tray-popup") {
                                let is_visible = popup_window.is_visible().unwrap_or(false);
                                println!("托盘弹窗可见性: {}", is_visible);

                                if is_visible {
                                    let _ = popup_window.hide();
                                    println!("隐藏托盘弹窗");
                                } else {
                                    // 重新计算位置，确保在正确的位置显示
                                    let (screen_width, screen_height) = get_screen_size();
                                    let popup_width = 420.0;
                                    let popup_height = 600.0;
                                    let (x, y) = calculate_tray_popup_position(screen_width, screen_height, popup_width, popup_height);

                                    let _ = popup_window.set_position(Position::Logical(LogicalPosition::new(x, y)));
                                    let _ = popup_window.show();
                                    let _ = popup_window.set_focus();
                                    println!("显示托盘弹窗");
                                }
                            } else {
                                // 创建托盘弹窗
                                println!("创建新的托盘弹窗");
                                create_tray_popup(app.clone()).unwrap_or_else(|e| {
                                    println!("创建托盘弹窗失败: {}", e);
                                });
                            }
                        }
                        TrayIconEvent::Click {
                            button: MouseButton::Right,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            println!("右键点击托盘图标，显示主界面");
                            let app = tray.app_handle();

                            // 显示主窗口
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        TrayIconEvent::DoubleClick {
                            button: MouseButton::Left,
                            ..
                        } => {
                            println!("双击托盘图标，退出程序");
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

                // 阻止窗口关闭，改为隐藏
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // 阻止关闭，改为隐藏窗口
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
