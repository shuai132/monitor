<template>
  <div class="high-cpu-alert">
    <div class="alert-header">
      <h4>⚠️ 高CPU使用率警告</h4>
      <button @click="clearAllAlerts" class="close-btn" title="关闭所有警告">✕</button>
    </div>

    <div class="alert-content">

      <div class="alert-description">
        <div class="duration-info">
          检测到 {{ alertProcesses.length }} 个高CPU进程
        </div>
        <span style="flex: 1;"></span>
        <button @click="disablePopup" class="dismiss-btn">
          🔇 关闭弹窗
        </button>
      </div>

      <ProcessList
          :processes="arrangedAlertProcesses"
          :compact="true"
          :isPinnedProcess="isPinnedAlertProcess"
          :getRealRank="getRealAlertRank"
          :getCpuUsageClass="getCpuUsageClass"
          :pinProcess="pinAlertProcess"
          :terminateProcess="handleTerminateProcess"
          :forceKillProcess="handleForceKillProcess"
          :restartProcess="restartProcess"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue';
import type {ProcessInfo} from '../composables/useProcesses';
import ProcessList from './ProcessList.vue';

interface Props {
  alertProcesses: ProcessInfo[];
  getProcessDuration: (pid: number) => number;
  terminateProcess: (pid: number) => Promise<void>;
  forceKillProcess: (pid: number) => Promise<void>;
  restartProcess: (processName: string) => Promise<void>;
  getCpuUsageClass: (cpuUsage: number) => string;
}

interface Emits {
  (e: 'clearAlert', pid: number): void;

  (e: 'clearAllAlerts'): void;

  (e: 'disablePopup'): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// 固定进程相关状态
const pinnedAlertProcess = ref<ProcessInfo | null>(null);
const pinnedAlertPosition = ref<number>(-1);

// 计算排列后的警告进程列表
const arrangedAlertProcesses = computed(() => {
  if (pinnedAlertProcess.value && pinnedAlertPosition.value >= 0) {
    // 查找固定进程的最新信息
    const updatedPinnedProcess = props.alertProcesses.find(p => p.pid === pinnedAlertProcess.value!.pid);

    if (updatedPinnedProcess) {
      // 从新列表中移除固定的进程
      const filteredProcesses = props.alertProcesses.filter(p => p.pid !== pinnedAlertProcess.value!.pid);

      // 在指定位置插入固定的进程
      const result = [...filteredProcesses];
      result.splice(pinnedAlertPosition.value, 0, updatedPinnedProcess);

      return result;
    } else {
      // 如果固定的进程不存在了，清除固定状态
      clearPinnedAlertProcess();
      return props.alertProcesses;
    }
  }

  return props.alertProcesses;
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
    const rank = props.alertProcesses.findIndex(p => p.pid === process.pid);
    return rank >= 0 ? rank + 1 : index + 1;
  }
  return index + 1;
}

async function handleTerminateProcess(pid: number) {
  await props.terminateProcess(pid);

  // 如果终止的是固定进程，清除固定状态
  if (pinnedAlertProcess.value?.pid === pid) {
    clearPinnedAlertProcess();
  }

  emit('clearAlert', pid);
}

async function handleForceKillProcess(pid: number) {
  await props.forceKillProcess(pid);

  // 如果终止的是固定进程，清除固定状态
  if (pinnedAlertProcess.value?.pid === pid) {
    clearPinnedAlertProcess();
  }

  emit('clearAlert', pid);
}

function clearAllAlerts() {
  // 清除所有警告时，也清除固定状态
  clearPinnedAlertProcess();
  emit('clearAllAlerts');
}

function disablePopup() {
  // 关闭高CPU警告弹窗
  clearPinnedAlertProcess();
  emit('disablePopup');
}
</script>

<style scoped>
.high-cpu-alert {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-radius: 8px;
  border: 1px solid rgba(239, 68, 68, 0.3);
  box-shadow: 0 8px 32px rgba(239, 68, 68, 0.2);
  padding: 12px;
  width: 100%;
  max-width: none;
  max-height: 600px;
  overflow: hidden;
}

.alert-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 12px;
  border-bottom: 1px solid rgba(239, 68, 68, 0.2);
  margin-bottom: 6px;
}

.alert-header h4 {
  margin: 0;
  font-size: 1.0rem;
  font-weight: 600;
  color: #2d3748;
}

.close-btn {
  background: none;
  border: none;
  font-size: 16px;
  color: #dc2626;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #b91c1c;
}

.alert-content {
  max-height: 400px;
  overflow-y: auto;
}

.alert-description {
  display: flex;
  margin: 0 0 6px 0;
  font-size: 13px;
  border-radius: 6px;
}

.duration-info {
  display: flex;
  align-items: center;
  font-size: 12px;
}

.dismiss-btn {
  background: #f8fafc;
  color: #dc2626;
  border: 1px solid rgba(239, 68, 68, 0.3);
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.dismiss-btn:hover {
  background: rgba(239, 68, 68, 0.05);
  border-color: #dc2626;
}
</style>