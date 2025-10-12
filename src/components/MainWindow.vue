<template>
  <div class="main-window">
    <!-- 控制按钮 -->
    <div class="controls">
      <button @click="getTopProcesses" :disabled="isLoading" class="refresh-btn">
        🔄 {{ isLoading ? '加载中...' : '手动刷新' }}
      </button>

      <button @click="toggleAutoRefresh" :class="{ active: isAutoRefresh }" class="auto-refresh-btn">
        {{ isAutoRefresh ? '⏸️ 停止自动刷新' : '▶️ 开启自动刷新' }}
      </button>
    </div>

    <!-- 消息提示 -->
    <div v-if="message" class="message-banner" :class="message.includes('失败') ? 'error' : 'success'">
      {{ message }}
    </div>

    <!-- 进程列表区域 -->
    <div class="processes-section">
      <h2>📊 CPU 占用率前10进程</h2>

      <div v-if="processes.length === 0" class="no-processes">
        <div class="loading-spinner" v-if="isLoading"></div>
        <p>{{ isLoading ? '加载中...' : '暂无数据' }}</p>
      </div>

      <ProcessList
        v-else
        :processes="processes"
        :compact="false"
        :isPinnedProcess="isPinnedProcess"
        :getRealRank="getRealRank"
        :getCpuUsageClass="getCpuUsageClass"
        :pinProcess="pinProcess"
        :terminateProcess="terminateProcess"
        :forceKillProcess="forceKillProcess"
        :restartProcess="restartProcess"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useProcesses } from '../composables/useProcesses';
import ProcessList from './ProcessList.vue';

const {
  processes,
  isAutoRefresh,
  isLoading,
  message,
  isPinnedProcess,
  getRealRank,
  getCpuUsageClass,
  pinProcess,
  terminateProcess,
  forceKillProcess,
  restartProcess,
  getTopProcesses,
  toggleAutoRefresh,
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
.main-window {
  margin: 0;
  padding: 24px;
  min-height: 100vh;
  background: #fafbfc;
  color: #1a202c;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
}

.controls {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin-bottom: 24px;
}

.refresh-btn, .auto-refresh-btn {
  background: #ffffff;
  border: 1px solid #e2e8f0;
  color: #4a5568;
  padding: 10px 20px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.refresh-btn:hover, .auto-refresh-btn:hover {
  background: #f7fafc;
  border-color: #cbd5e0;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background: #f7fafc;
}

.auto-refresh-btn.active {
  background: #38a169;
  color: white;
  border-color: #38a169;
}

.auto-refresh-btn.active:hover {
  background: #2f855a;
  border-color: #2f855a;
}

/* 消息横幅 */
.message-banner {
  padding: 12px 16px;
  margin: 16px 0;
  border-radius: 6px;
  text-align: center;
  font-weight: 500;
  font-size: 14px;
  animation: slideDown 0.3s ease-out;
}

.message-banner.success {
  background: #f0fff4;
  color: #22543d;
  border: 1px solid #9ae6b4;
}

.message-banner.error {
  background: #fff5f5;
  color: #742a2a;
  border: 1px solid #feb2b2;
}

@keyframes slideDown {
  from { opacity: 0; transform: translateY(-10px); }
  to { opacity: 1; transform: translateY(0); }
}

.processes-section {
  background: #ffffff;
  border-radius: 12px;
  padding: 24px;
  margin: 24px 0;
  border: 1px solid #e2e8f0;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.processes-section h2 {
  margin-top: 0;
  margin-bottom: 20px;
  font-size: 1.25rem;
  font-weight: 600;
  color: #2d3748;
}

.no-processes {
  text-align: center;
  padding: 48px 24px;
  font-size: 14px;
  color: #718096;
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 2px solid #e2e8f0;
  border-top: 2px solid #3182ce;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .main-window {
    padding: 16px;
  }

  .controls {
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }
}

@media (max-width: 480px) {
  .main-window {
    padding: 12px;
  }

  .processes-section {
    padding: 16px;
  }
}
</style>