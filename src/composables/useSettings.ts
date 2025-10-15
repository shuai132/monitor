import {ref, watch} from 'vue';

export interface AppSettings {
    autoRefresh: boolean;
    refreshInterval: number; // 秒
    trayShowProcess: boolean;
    trayShowPercentage: boolean;
    trayDisplayMode: "always" | "warning-only";
    highCpuThreshold: number; // 百分比
    highCpuDuration: number; // 秒
    enableHighCpuPopup: boolean; // 是否启用高CPU警告弹窗
}

const defaultSettings: AppSettings = {
    autoRefresh: true,
    refreshInterval: 5,
    trayShowProcess: true,
    trayShowPercentage: true,
    trayDisplayMode: 'warning-only',
    highCpuThreshold: 95,
    highCpuDuration: 10,
    enableHighCpuPopup: false,
};

const SETTINGS_KEY = 'cpu-monitor-settings';
const settings = ref<AppSettings>({...defaultSettings});

export function useSettings() {
    // 从本地存储加载设置
    function loadSettings() {
        try {
            const stored = localStorage.getItem(SETTINGS_KEY);
            if (stored) {
                const parsed = JSON.parse(stored);
                settings.value = {...defaultSettings, ...parsed};
            }
        } catch (error) {
            console.error('加载设置失败:', error);
            settings.value = {...defaultSettings};
        }
    }

    // 保存设置到本地存储
    function saveSettings() {
        try {
            localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings.value));
        } catch (error) {
            console.error('保存设置失败:', error);
        }
    }

    // 重置设置为默认值
    function resetSettings() {
        settings.value = {...defaultSettings};
        saveSettings();
    }

    // 监听设置变化并自动保存
    watch(settings, saveSettings, {deep: true});

    // 初始化时加载设置
    loadSettings();

    return {
        settings,
        loadSettings,
        saveSettings,
        resetSettings
    };
}