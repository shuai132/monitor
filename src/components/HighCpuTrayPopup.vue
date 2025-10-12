<template>
  <div class="high-cpu-tray-popup">
    <HighCpuAlert
      :alertProcesses="alertProcesses"
      :getProcessDuration="getProcessDuration"
      :terminateProcess="terminateProcess"
      :forceKillProcess="forceKillProcess"
      :restartProcess="restartProcess"
      :getCpuUsageClass="getCpuUsageClass"
      @clearAlert="clearAlert"
      @clearAllAlerts="clearAllAlerts"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue';
import { useProcesses } from '../composables/useProcesses';
import { useSettings } from '../composables/useSettings';
import { useHighCpuMonitor } from '../composables/useHighCpuMonitor';
import HighCpuAlert from './HighCpuAlert.vue';

// 设置管理
const { settings } = useSettings();

// 进程管理
const {
  processes,
  getCpuUsageClass,
  terminateProcess,
  forceKillProcess,
  restartProcess,
  startAutoRefresh,
  stopAutoRefresh
} = useProcesses(settings);

// 高CPU监控
const {
  alertProcesses,
  monitorHighCpu,
  clearAlert,
  clearAllAlerts,
  getProcessDuration
} = useHighCpuMonitor();

// 监控进程变化，检查高CPU使用率
watch(processes, (newProcesses) => {
  if (newProcesses.length > 0) {
    monitorHighCpu(newProcesses, settings.value);
  }
}, { deep: true });

onMounted(() => {
  startAutoRefresh();
});

onUnmounted(() => {
  stopAutoRefresh();
});
</script>

<style scoped>
.high-cpu-tray-popup {
  padding: 0;
  background: transparent;
  width: 100%;
  max-width: 420px;
  box-sizing: border-box;
}
</style>
