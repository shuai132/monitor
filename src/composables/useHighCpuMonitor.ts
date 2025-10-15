import {computed, ref} from 'vue';
import type {ProcessInfo} from './useProcesses';
import type {AppSettings} from './useSettings';
import {invoke} from "@tauri-apps/api/core";

interface HighCpuProcess {
    process: ProcessInfo;
    startTime: number; // 开始时间戳
    lastTime: number; // 上次时间戳
    duration: number; // 持续时间（秒）
}

export function useHighCpuMonitor() {
    const highCpuProcesses = ref<Map<number, HighCpuProcess>>(new Map());
    const alertProcesses = ref<ProcessInfo[]>([]);

    // 检查是否需要显示警告弹窗
    const shouldShowAlert = computed(() => alertProcesses.value.length > 0);

    // 监控高CPU使用率进程
    function monitorHighCpu(processes: ProcessInfo[], settings: AppSettings) {
        const currentTimeSec = Math.floor(Date.now() / 1000);
        const thresholdMs = settings.highCpuDuration;

        // 检查当前高CPU进程
        const currentHighCpuPids = new Set<number>();

        for (const process of processes) {
            if (process.cpu_usage >= settings.highCpuThreshold) {
                currentHighCpuPids.add(process.pid);

                if (highCpuProcesses.value.has(process.pid)) {
                    const existing = highCpuProcesses.value.get(process.pid)!;
                    existing.process = process;
                    existing.duration += (currentTimeSec - existing.startTime);
                } else {
                    highCpuProcesses.value.set(process.pid, {
                        process,
                        startTime: currentTimeSec,
                        lastTime: currentTimeSec,
                        duration: 0
                    });
                }
            }
        }

        // 移除不再是高CPU的进程
        for (const [pid] of highCpuProcesses.value.entries()) {
            if (!currentHighCpuPids.has(pid)) {
                highCpuProcesses.value.delete(pid);
            }
        }

        // 检查哪些进程需要警告
        const newAlertProcesses: ProcessInfo[] = [];
        let i = 0;
        for (const [, highCpuProcess] of highCpuProcesses.value.entries()) {
            const durationMs = currentTimeSec - highCpuProcess.startTime;
            const p = highCpuProcess.process;
            console.log("high cpu: ", i++, p.name, p.cpu_usage, durationMs, thresholdMs);
            if (durationMs >= thresholdMs) {
                newAlertProcesses.push(highCpuProcess.process);
            }
        }

        function updateTrayTitleWithProcess(p: ProcessInfo) {
            let title = p.name;
            if (title.length > 12) {
                title = title.substring(0, 9);
                title += "...";
            }
            title += `:${Math.floor(p.cpu_usage)}%`;
            invoke('update_tray_title', {tooltip: "", title: title}).catch();
        }

        console.log("newAlertProcesses:", newAlertProcesses);
        switch (settings.trayDisplayMode) {
            case "always": {
                updateTrayTitleWithProcess(processes[0]);
            }
                break;
            case "warning-only": {
                if (newAlertProcesses.length > 0) {
                    updateTrayTitleWithProcess(newAlertProcesses[0]);

                } else {
                    invoke('update_tray_title', {tooltip: "", title: ""}).catch();
                }
            }
                break;
        }

        alertProcesses.value = newAlertProcesses;
        if (newAlertProcesses.length > 0 && settings.enableHighCpuPopup) {
            invoke("show_high_cpu_alert").catch();
        } else {
            invoke("hide_high_cpu_alert").catch();
        }
    }

    // 清除特定进程的警告
    function clearAlert(pid: number) {
        highCpuProcesses.value.delete(pid);
        alertProcesses.value = alertProcesses.value.filter(p => p.pid !== pid);
    }

    // 清除所有警告
    function clearAllAlerts() {
        highCpuProcesses.value.clear();
        alertProcesses.value = [];
    }

    // 获取进程的持续时间（秒）
    function getProcessDuration(pid: number): number {
        const highCpuProcess = highCpuProcesses.value.get(pid);
        if (!highCpuProcess) return 0;

        const currentTime = Date.now();
        return (currentTime - highCpuProcess.startTime) / 1000;
    }

    return {
        highCpuProcesses,
        alertProcesses,
        shouldShowAlert,
        monitorHighCpu,
        clearAlert,
        clearAllAlerts,
        getProcessDuration
    };
}