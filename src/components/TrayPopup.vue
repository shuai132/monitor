<template>
  <div class="tray-popup">
    <div v-if="processes.length === 0" class="no-processes">
      <div class="loading-spinner" v-if="isLoading"></div>
      <p>{{ isLoading ? '加载中...' : '暂无数据' }}</p>
    </div>

    <ProcessList
      v-else
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

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useProcesses } from '../composables/useProcesses';
import ProcessList from './ProcessList.vue';

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
} = useProcesses();

onMounted(() => {
  startAutoRefresh();
});

onUnmounted(() => {
  stopAutoRefresh();
});
</script>

<style scoped>
.tray-popup {
  padding: 12px;
  min-height: auto;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.no-processes {
  text-align: center;
  padding: 24px 16px;
  font-size: 12px;
  color: #718096;
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
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>