import {ref, type Ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import type {AppSettings} from "./useSettings";

export interface ProcessInfo {
    name: string;
    pid: number;
    cpu_usage: number;
}

export function useProcesses(settings?: Ref<AppSettings>) {
    const processes = ref<ProcessInfo[]>([]);
    const originalProcesses = ref<ProcessInfo[]>([]);
    const isLoading = ref(false);
    const message = ref("");
    const pinnedProcess = ref<ProcessInfo | null>(null);
    const pinnedPosition = ref<number>(-1);
    let refreshInterval: number | null = null;

    async function getTopProcesses() {
        isLoading.value = true;
        try {
            const result = await invoke<ProcessInfo[]>("get_top_cpu_processes");
            originalProcesses.value = result; // 保存原始排序
            processes.value = arrangeProcesses(result);
        } catch (error) {
            console.error("获取进程信息失败:", error);
            processes.value = [];
            originalProcesses.value = [];
        } finally {
            isLoading.value = false;
        }
    }

    function arrangeProcesses(newProcesses: ProcessInfo[]): ProcessInfo[] {
        if (pinnedProcess.value && pinnedPosition.value >= 0) {
            // 查找固定进程的最新信息
            const updatedPinnedProcess = newProcesses.find(p => p.pid === pinnedProcess.value!.pid);

            if (updatedPinnedProcess) {
                // 从新列表中移除固定的进程
                const filteredProcesses = newProcesses.filter(p => p.pid !== pinnedProcess.value!.pid);

                // 在指定位置插入固定的进程
                const result = [...filteredProcesses];
                result.splice(pinnedPosition.value, 0, updatedPinnedProcess);

                // 如果超过10个，只保留前10个
                return result.slice(0, 10);
            } else {
                // 如果固定的进程不存在了，清除固定状态
                clearPinnedProcess();
                return newProcesses;
            }
        }

        return newProcesses;
    }

    function pinProcess(process: ProcessInfo, index: number) {
        if (isPinnedProcess(process)) {
            // 取消固定
            clearPinnedProcess();
        } else {
            // 固定进程
            pinnedProcess.value = process;
            pinnedPosition.value = index;
        }
    }

    function clearPinnedProcess() {
        pinnedProcess.value = null;
        pinnedPosition.value = -1;
    }

    function isPinnedProcess(process: ProcessInfo): boolean {
        return pinnedProcess.value?.pid === process.pid;
    }

    function getRealRank(process: ProcessInfo, index: number): number {
        if (isPinnedProcess(process)) {
            // 从原始进程列表中查找真实排名
            const rank = originalProcesses.value.findIndex(p => p.pid === process.pid);
            return rank >= 0 ? rank + 1 : index + 1;
        }
        return index + 1;
    }

    async function terminateProcess(pid: number) {
        try {
            const result = await invoke<string>("terminate_process", {pid});
            message.value = result;
            setTimeout(() => message.value = "", 3000);
            // 刷新进程列表
            await getTopProcesses();
        } catch (error) {
            message.value = `终止进程失败: ${error}`;
            setTimeout(() => message.value = "", 3000);
        }
    }

    async function forceKillProcess(pid: number) {
        if (!confirm(`确定要强制终止进程 ${pid} 吗？这可能导致数据丢失。`)) {
            return;
        }

        try {
            const result = await invoke<string>("force_kill_process", {pid});
            message.value = result;
            setTimeout(() => message.value = "", 3000);
            // 刷新进程列表
            await getTopProcesses();
        } catch (error) {
            message.value = `强制终止进程失败: ${error}`;
            setTimeout(() => message.value = "", 3000);
        }
    }

    async function restartProcess(processName: string) {
        try {
            const result = await invoke<string>("restart_process", {processName});
            message.value = result;
            setTimeout(() => message.value = "", 3000);
        } catch (error) {
            message.value = `重启进程失败: ${error}`;
            setTimeout(() => message.value = "", 3000);
        }
    }

    function getCpuUsageClass(cpuUsage: number) {
        if (cpuUsage > 50) return 'high-cpu';
        if (cpuUsage > 20) return 'medium-cpu';
        return 'low-cpu';
    }

    function startAutoRefresh() {
        if (refreshInterval) return;
        getTopProcesses().catch();
        if (settings?.value.autoRefresh) {
            const interval = 2 * 1000;
            refreshInterval = setInterval(getTopProcesses, interval);
        }
    }

    function updateAutoRefresh(enabled: boolean, intervalSeconds: number) {
        if (refreshInterval) {
            clearInterval(refreshInterval);
            refreshInterval = null;
        }

        if (enabled) {
            refreshInterval = setInterval(getTopProcesses, intervalSeconds * 1000);
        }
    }

    function stopAutoRefresh() {
        if (refreshInterval) {
            clearInterval(refreshInterval);
            refreshInterval = null;
        }
    }

    // 监听设置变化
    if (settings) {
        watch(
            () => [settings.value.autoRefresh, settings.value.refreshInterval],
            ([autoRefresh, interval]) => {
                updateAutoRefresh(autoRefresh as boolean, interval as number);
            }
        );
    }

    return {
        // 响应式数据
        processes,
        originalProcesses,
        isLoading,
        message,
        pinnedProcess,
        pinnedPosition,

        // 方法
        getTopProcesses,
        arrangeProcesses,
        pinProcess,
        clearPinnedProcess,
        isPinnedProcess,
        getRealRank,
        terminateProcess,
        forceKillProcess,
        restartProcess,
        getCpuUsageClass,
        startAutoRefresh,
        stopAutoRefresh,
        updateAutoRefresh
    };
}