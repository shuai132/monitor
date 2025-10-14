import {invoke} from '@tauri-apps/api/core';
import {AppSettings} from './useSettings';

export interface RustAppSettings {
    auto_refresh: boolean;
    refresh_interval: number;
    tray_show_process: boolean;
    tray_show_percentage: boolean;
    tray_display_mode: string;
    high_cpu_alert: boolean;
    high_cpu_threshold: number;
    high_cpu_duration: number;
}

export function useTrayUpdater() {
    // 将前端设置转换为Rust后端需要的格式
    function convertToRustSettings(settings: AppSettings): RustAppSettings {
        return {
            auto_refresh: settings.autoRefresh,
            refresh_interval: settings.refreshInterval,
            tray_show_process: settings.trayShowProcess,
            tray_show_percentage: settings.trayShowPercentage,
            tray_display_mode: settings.trayDisplayMode,
            high_cpu_alert: settings.highCpuAlert,
            high_cpu_threshold: settings.highCpuThreshold,
            high_cpu_duration: settings.highCpuDuration
        };
    }

    // 更新托盘显示
    async function updateTrayDisplay(settings: AppSettings): Promise<void> {
        try {
            const rustSettings = convertToRustSettings(settings);
            await invoke('update_tray_with_settings', {settings: rustSettings});
        } catch (error) {
            console.error('更新托盘显示失败:', error);
            throw error;
        }
    }

    return {
        updateTrayDisplay
    };
}