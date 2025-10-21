use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, Signal, System};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, LogicalPosition, Manager, Position, State, WindowEvent,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub memory_usage: u64,
    pub cpu_usage: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub auto_refresh: bool,
    pub refresh_interval: u32,
    pub tray_show_process: bool,
    pub tray_show_percentage: bool,
    pub tray_display_mode: String, // "always" or "warning-only"
    pub high_cpu_threshold: f32,
    pub high_cpu_duration: u32,
    pub enable_high_cpu_popup: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_refresh: true,
            refresh_interval: 3,
            tray_show_process: true,
            tray_show_percentage: true,
            tray_display_mode: "always".to_string(),
            high_cpu_threshold: 100.0,
            high_cpu_duration: 30, // 改为30秒
            enable_high_cpu_popup: false,
        }
    }
}

// 全局设置状态管理
pub struct AppState {
    pub settings: Arc<RwLock<AppSettings>>,
}

#[tauri::command]
async fn get_top_cpu_processes() -> Result<Vec<ProcessInfo>, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // 等待一秒再次刷新以获得更准确的CPU使用率
    tokio::time::sleep(Duration::from_secs(1)).await;
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            name: process.name().to_str().unwrap().to_owned(),
            pid: pid.as_u32(),
            memory_usage: process.memory(),
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
        let process_name = process.name().to_str().unwrap().to_owned();

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
        let process_name = process.name().to_str().unwrap().to_owned();

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
    let result = Command::new("open").arg("-a").arg(&process_name).output();

    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(format!("尝试重启应用程序: {}", process_name))
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                Err(format!("重启失败: {}", error))
            }
        }
        Err(e) => Err(format!("无法重启进程 {}: {}", process_name, e)),
    }
}

#[tauri::command]
async fn show_high_cpu_alert(
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 首先检查是否启用了高CPU警告弹窗
    let settings = if let Ok(settings) = state.settings.read() {
        settings.clone()
    } else {
        return Err("无法读取应用设置".to_string());
    };

    // 如果用户没有启用高CPU警告弹窗，直接返回
    if !settings.enable_high_cpu_popup {
        return Ok(());
    }

    // 检查是否已经有高CPU警告弹窗存在
    if let Some(alert_window) = app_handle.get_webview_window("high-cpu-alert") {
        let is_visible = alert_window.is_visible().unwrap_or(false);

        if !is_visible {
            // 重新计算位置，确保在正确的位置显示
            let (screen_width, screen_height) = get_screen_size();
            let popup_width = 420.0;
            let popup_height = 200.0;
            let (x, y) = calculate_tray_popup_position(
                screen_width,
                screen_height,
                popup_width,
                popup_height,
            );
            let alert_x = x + 0.0;
            let alert_y = y + 10.0;

            let _ = alert_window
                .set_position(Position::Logical(LogicalPosition::new(alert_x, alert_y)));
            let _ = alert_window.show();
        }
    } else {
        // 创建高CPU警告弹窗
        create_high_cpu_alert(app_handle).map_err(|e| format!("创建高CPU警告弹窗失败: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
async fn hide_high_cpu_alert(app_handle: AppHandle) -> Result<(), String> {
    if let Some(alert_window) = app_handle.get_webview_window("high-cpu-alert") {
        let _ = alert_window.hide();
    }

    Ok(())
}

#[tauri::command]
async fn update_settings(settings: AppSettings, state: State<'_, AppState>) -> Result<(), String> {
    println!("update_settings: {:?}", settings);

    // 更新全局设置状态
    if let Ok(mut global_settings) = state.settings.write() {
        *global_settings = settings.clone();
    }

    Ok(())
}

#[tauri::command]
fn update_tray_title(app_handle: AppHandle, tooltip: String, title: String) -> Result<(), String> {
    if let Some(tray) = app_handle.tray_by_id("main-tray") {
        let _ = tray.set_tooltip(Some(&tooltip));
        let _ = tray.set_title(Some(&title));
    }
    Ok(())
}

#[tauri::command]
async fn exit_app(app_handle: AppHandle) -> Result<(), String> {
    println!("正在退出应用程序...");
    app_handle.exit(0);
    Ok(())
}

fn create_tray_popup(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // 获取屏幕尺寸来计算位置
    let popup_width = 420.0;
    let popup_height = 400.0; // 减少初始高度，让内容自适应

    // 获取主显示器的尺寸
    let (screen_width, screen_height) = get_screen_size();

    // 获取托盘位置并计算弹窗位置
    let (x, y) =
        calculate_tray_popup_position(screen_width, screen_height, popup_width, popup_height);

    println!(
        "Screen size: {}x{}, Popup position: ({}, {})",
        screen_width, screen_height, x, y
    );

    let window =
        WebviewWindowBuilder::new(&app, "tray-popup", WebviewUrl::App("index.html".into()))
            .title("CPU监控器 - 托盘弹窗")
            .inner_size(popup_width, popup_height)
            .position(x, y)
            .resizable(false)
            .maximizable(false)
            .minimizable(false)
            .skip_taskbar(true)
            .always_on_top(true)
            .decorations(false) // 无边框窗口
            .shadow(true) // 添加阴影
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

fn create_high_cpu_alert(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // 获取屏幕尺寸来计算位置
    let popup_width = 420.0;
    let popup_height = 200.0;

    // 获取主显示器的尺寸
    let (screen_width, screen_height) = get_screen_size();

    // 计算高CPU警告弹窗位置（稍微偏移，避免与托盘弹窗重叠）
    let (x, y) =
        calculate_tray_popup_position(screen_width, screen_height, popup_width, popup_height);
    let alert_x = x + 0.0;
    let alert_y = y + 10.0;

    println!("High CPU Alert position: ({}, {})", alert_x, alert_y);

    let window =
        WebviewWindowBuilder::new(&app, "high-cpu-alert", WebviewUrl::App("index.html".into()))
            .title("CPU监控器 - 高CPU警告")
            .inner_size(popup_width, popup_height)
            .position(alert_x, alert_y)
            .resizable(false)
            .maximizable(false)
            .minimizable(false)
            .skip_taskbar(true)
            .always_on_top(true)
            .decorations(false) // 无边框窗口
            .shadow(true) // 添加阴影
            .focusable(false)
            .focused(false)
            .build()?;

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

fn calculate_tray_popup_position(
    screen_width: f64,
    screen_height: f64,
    popup_width: f64,
    popup_height: f64,
) -> (f64, f64) {
    let margin = 8.0;
    let menu_bar_height = 24.0;

    // 尝试获取托盘图标位置
    let (tray_x, _tray_y) =
        get_tray_icon_position().unwrap_or((screen_width - 50.0, menu_bar_height / 2.0));

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

    println!(
        "托盘位置估算: ({}, {}), 弹窗最终位置: ({}, {})",
        tray_x, _tray_y, x, y
    );

    (x, y)
}

#[cfg(target_os = "macos")]
fn get_screen_size() -> (f64, f64) {
    use core_graphics::display::{CGDirectDisplayID, CGDisplay};

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_top_cpu_processes,
            terminate_process,
            force_kill_process,
            restart_process,
            show_high_cpu_alert,
            hide_high_cpu_alert,
            update_settings,
            update_tray_title,
            exit_app
        ])
        .setup(|app| {
            // 在macOS上隐藏Dock图标
            #[cfg(target_os = "macos")]
            {
                use tauri::utils::platform;
                if let Err(e) = platform::current_exe() {
                    println!("获取可执行文件路径失败: {}", e);
                } else {
                    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                }
            }

            // 初始化应用状态
            let app_state = AppState {
                settings: Arc::new(RwLock::new(AppSettings::default())),
            };
            app.manage(app_state);

            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("加载中...")
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
                                    let (x, y) = calculate_tray_popup_position(
                                        screen_width,
                                        screen_height,
                                        popup_width,
                                        popup_height,
                                    );

                                    let _ = popup_window.set_position(Position::Logical(
                                        LogicalPosition::new(x, y),
                                    ));
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
