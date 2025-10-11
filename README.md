# CPU 监控器

一个专为 macOS 设计的系统托盘 CPU 监控应用程序，使用 Tauri + Vue 构建。

## 功能特性

### ✨ 主要功能
- 🖥️ **系统托盘集成**: 在 macOS 状态栏显示监控图标
- 📊 **实时 CPU 监控**: 显示 CPU 占用率最高的前10个进程
- 🖱️ **悬停显示信息**: 鼠标悬停在托盘图标上显示 CPU 使用情况
- 🔄 **自动刷新**: 每5秒自动更新托盘工具提示信息
- 📱 **直观界面**: 点击托盘图标打开详细的监控窗口

### 🎯 界面特性
- 现代化的 UI 设计，支持明暗主题
- 实时显示进程名称、PID 和 CPU 使用率
- 高 CPU 占用进程的特殊视觉提示
- 可视化的 CPU 使用率进度条
- 手动刷新和自动刷新切换功能

## 系统要求

- macOS 10.13+ 
- Node.js 16+
- Rust 1.60+

## 安装和运行

### 开发环境设置

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd monitor
   ```

2. **安装依赖**
   ```bash
   # 安装前端依赖
   pnpm install
   
   # 安装 Rust 依赖 (自动)
   ```

3. **开发模式运行**
   ```bash
   pnpm tauri dev
   ```

4. **构建生产版本**
   ```bash
   pnpm tauri build
   ```

## 使用说明

### 托盘功能
- **左键单击托盘图标**: 显示/隐藏主监控窗口
- **右键单击托盘图标**: 显示上下文菜单
- **悬停托盘图标**: 显示当前 CPU 占用率前10的进程信息

### 主窗口功能
- **🔄 手动刷新**: 立即获取最新的进程 CPU 数据
- **▶️ 自动刷新控制**: 开启/停止每2秒的自动刷新
- **进程排序**: 按 CPU 使用率降序显示
- **高亮显示**: CPU 使用率超过50%的进程会特殊标记

## 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust + Tauri 2.0
- **系统信息**: sysinfo crate
- **UI 框架**: 原生 CSS + Vue Composition API
- **异步运行时**: Tokio

## 项目结构

```
monitor/
├── src/                    # Vue 前端代码
│   ├── App.vue            # 主应用组件
│   ├── main.ts            # 前端入口
│   └── assets/            # 静态资源
├── src-tauri/             # Rust 后端代码
│   ├── src/
│   │   ├── main.rs        # 应用入口
│   │   └── lib.rs         # 核心逻辑
│   ├── Cargo.toml         # Rust 依赖配置
│   ├── tauri.conf.json    # Tauri 配置
│   └── icons/             # 应用图标
├── package.json           # Node.js 依赖
└── README.md             # 项目说明
```

## 核心功能实现

### 系统托盘
- 使用 Tauri 2.0 的 `tray-icon` 功能
- 支持 macOS 原生的托盘图标显示
- 动态更新工具提示文本显示实时 CPU 信息

### CPU 监控
- 使用 `sysinfo` crate 获取系统进程信息
- 异步处理避免 UI 阻塞
- 智能排序和过滤显示最重要的进程

### 实时更新
- 后台 Tokio 任务定期刷新数据
- 前端自动刷新机制
- 手动刷新选项

## 开发说明

### 添加新功能
1. 在 `src-tauri/src/lib.rs` 中添加 Tauri 命令
2. 在 `src/App.vue` 中调用新的命令
3. 更新 UI 以显示新功能

### 自定义刷新频率
在 `lib.rs` 中修改 `update_tray_tooltip` 函数的间隔时间：
```rust
let mut interval = interval(Duration::from_secs(5)); // 修改这里的秒数
```

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

---

**注意**: 这是一个专门为 macOS 优化的应用程序，在其他操作系统上可能需要适配。
