<template>
  <div class="tray-popup">
    <!-- 高CPU警告进程（如有） -->
    <template v-if="shouldShowAlert">
      <div class="section-title alert-title">⚠️ 高CPU使用率警告</div>
      <div class="process-list-container">
        <ProcessList
            :processes="arrangedAlertProcesses"
            :compact="true"
            :isPinnedProcess="isPinnedAlertProcess"
            :getRealRank="getRealAlertRank"
            :getCpuUsageClass="getCpuUsageClass"
            :pinProcess="pinAlertProcess"
            :terminateProcess="handleAlertTerminate"
            :forceKillProcess="handleAlertForceKill"
            :restartProcess="restartProcess"
        />
      </div>

      <!-- 分隔线 -->
      <div v-if="processes.length > 0" class="separator"></div>
    </template>

    <!-- 进程列表 -->
    <div v-if="processes.length === 0 && !shouldShowAlert" class="no-processes">
      <div class="loading-spinner" v-if="isLoading"></div>
      <p>{{ isLoading ? '加载中...' : '暂无数据' }}</p>
    </div>

    <template v-if="processes.length > 0">
      <div class="section-title">📊 CPU占用前10进程</div>
      <div class="process-list-container">
        <ProcessList
            :processes="processes"
            :compact="true"
            :isPinnedProcess="isPinnedProcess"
            :getRealRank="getRealRank"
            :getCpuUsageClass="getCpuUsageClass"
            :pinProcess="pinProcess"
            :terminateProcess="terminateProcess"
            :forceKillProcess="forceKillProcess"
            :restartProcess="restartProcess"
        />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref, watch} from 'vue';
import {type ProcessInfo, useProcesses} from '../composables/useProcesses';
import {useSettings} from '../composables/useSettings';
import {useHighCpuMonitor} from '../composables/useHighCpuMonitor';
import ProcessList from './ProcessList.vue';
import {usePageVisibility} from '../composables/usePageVisibility';

// 设置管理
const {settings} = useSettings();

const {pageVisible} = usePageVisibility();
watch(pageVisible, (v) => {
  if (v) {
    startAutoRefresh();
  } else {
    stopAutoRefresh();
  }
});

onMounted(() => {
  startAutoRefresh();
});

// 进程管理
const {
  processes,
  isLoading,
  isPinnedProcess,
  getRealRank,
  getCpuUsageClass,
  pinProcess,
  terminateProcess,
  forceKillProcess,
  restartProcess,
  startAutoRefresh,
  stopAutoRefresh
} = useProcesses(settings);

// 高CPU监控
const {
  alertProcesses,
  shouldShowAlert,
  clearAlert
} = useHighCpuMonitor();

// 高CPU警告进程的固定功能
const pinnedAlertProcess = ref<ProcessInfo | null>(null);
const pinnedAlertPosition = ref<number>(-1);

// 计算排列后的警告进程列表
const arrangedAlertProcesses = computed(() => {
  if (pinnedAlertProcess.value && pinnedAlertPosition.value >= 0) {
    // 查找固定进程的最新信息
    const updatedPinnedProcess = alertProcesses.value.find(p => p.pid === pinnedAlertProcess.value!.pid);

    if (updatedPinnedProcess) {
      // 从新列表中移除固定的进程
      const filteredProcesses = alertProcesses.value.filter(p => p.pid !== pinnedAlertProcess.value!.pid);

      // 在指定位置插入固定的进程
      const result = [...filteredProcesses];
      result.splice(pinnedAlertPosition.value, 0, updatedPinnedProcess);

      return result;
    } else {
      // 如果固定的进程不存在了，清除固定状态
      clearPinnedAlertProcess();
      return alertProcesses.value;
    }
  }

  return alertProcesses.value;
});

// 固定进程操作
function pinAlertProcess(process: ProcessInfo, index: number) {
  if (isPinnedAlertProcess(process)) {
    // 取消固定
    clearPinnedAlertProcess();
  } else {
    // 固定进程
    pinnedAlertProcess.value = process;
    pinnedAlertPosition.value = index;
  }
}

function clearPinnedAlertProcess() {
  pinnedAlertProcess.value = null;
  pinnedAlertPosition.value = -1;
}

function isPinnedAlertProcess(process: ProcessInfo): boolean {
  return pinnedAlertProcess.value?.pid === process.pid;
}

function getRealAlertRank(process: ProcessInfo, index: number): number {
  if (isPinnedAlertProcess(process)) {
    // 从原始警告进程列表中查找真实排名
    const rank = alertProcesses.value.findIndex(p => p.pid === process.pid);
    return rank >= 0 ? rank + 1 : index + 1;
  }
  return index + 1;
}

async function handleAlertTerminate(pid: number) {
  await terminateProcess(pid);

  // 如果终止的是固定进程，清除固定状态
  if (pinnedAlertProcess.value?.pid === pid) {
    clearPinnedAlertProcess();
  }

  clearAlert(pid);
}

async function handleAlertForceKill(pid: number) {
  await forceKillProcess(pid);

  // 如果终止的是固定进程，清除固定状态
  if (pinnedAlertProcess.value?.pid === pid) {
    clearPinnedAlertProcess();
  }

  clearAlert(pid);
}

</script>

<style scoped>
.tray-popup {
  padding: 12px;
  width: 100%;
  height: 100%;
  min-height: 100%;
  max-height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 1;
}

.high-cpu-section {
  margin-bottom: 16px;
}

.separator {
  height: 1px;
  background: linear-gradient(to right, transparent, #e2e8f0 50%, transparent);
  margin: 16px 0;
}

.process-section {
  margin-top: 8px;
}

.section-title {
  font-size: 13px;
  font-weight: 600;
  color: #4a5568;
  margin-bottom: 12px;
  padding-bottom: 6px;
  border-bottom: 1px solid #e2e8f0;
}

.alert-title {
  color: #dc2626;
  background: rgba(239, 68, 68, 0.05);
  padding: 6px 8px;
  border-radius: 4px;
  border-bottom: 1px solid rgba(239, 68, 68, 0.2);
}

.process-list-container {
  flex: 1;
  min-height: 0;
  position: relative;
  z-index: 2;
}

.no-processes {
  text-align: center;
  padding: 24px 16px;
  font-size: 12px;
  color: #718096;
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.loading-spinner {
  width: 24px;
  height: 24px;
  border: 2px solid #e2e8f0;
  border-top: 2px solid #3182ce;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 12px;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>