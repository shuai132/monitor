<template>
  <div class="main-window">
    <!-- 控制按钮 -->
    <div class="controls">
      <button @click="showSettings = !showSettings" class="settings-btn">
        ⚙️ 设置
      </button>

      <button @click="exitApp" class="exit-btn">
        ⏻ 退出
      </button>

      <button @click="getTopProcesses" :disabled="isLoading" class="refresh-btn">
        🔄 {{ isLoading ? '加载中...' : '手动刷新' }}
      </button>
    </div>

    <!-- 设置面板 -->
    <SettingsPanel
        v-if="showSettings"
        @close="showSettings = false"
        @autoRefreshChange="handleAutoRefreshChange"
    />

    <!-- 高CPU警告弹窗 -->
    <HighCpuAlert
        v-if="shouldShowAlert && !showSettings && settings.enableHighCpuPopup"
        :alertProcesses="alertProcesses"
        :getProcessDuration="getProcessDuration"
        :terminateProcess="terminateProcess"
        :forceKillProcess="forceKillProcess"
        :restartProcess="restartProcess"
        :getCpuUsageClass="getCpuUsageClass"
        @clearAlert="clearAlert"
        @clearAllAlerts="clearAllAlerts"
        @disablePopup="disablePopup"
    />

    <!-- 消息提示 -->
    <div v-if="message" class="message-banner" :class="message.includes('失败') ? 'error' : 'success'">
      {{ message }}
    </div>

    <!-- 进程列表区域 -->
    <div class="processes-section">
      <h4>📊 CPU 占用率前10进程</h4>

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
import {onMounted, onUnmounted, ref, watch} from 'vue';
import {useProcesses} from '../composables/useProcesses';
import {useSettings} from '../composables/useSettings';
import {useHighCpuMonitor} from '../composables/useHighCpuMonitor';
import {useTrayUpdater} from '../composables/useTrayUpdater';
import ProcessList from './ProcessList.vue';
import SettingsPanel from './SettingsPanel.vue';
import HighCpuAlert from './HighCpuAlert.vue';

const showSettings = ref(false);

// 设置管理
const {settings} = useSettings();

// 进程管理
const {
  processes,
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
  startAutoRefresh,
  stopAutoRefresh,
  updateAutoRefresh
} = useProcesses(settings);

// 高CPU监控
const {
  alertProcesses,
  shouldShowAlert,
  monitorHighCpu,
  clearAlert,
  clearAllAlerts,
  getProcessDuration
} = useHighCpuMonitor();

// 托盘更新器
const {updateTrayDisplay} = useTrayUpdater();

// 处理设置变化
function handleAutoRefreshChange(enabled: boolean, interval: number) {
  updateAutoRefresh(enabled, interval);
}

// 监控进程变化，检查高CPU使用率
watch(processes, (newProcesses) => {
  if (newProcesses.length > 0) {
    monitorHighCpu(newProcesses, settings.value);
  }
}, {deep: true});

// 监听设置变化，立即更新托盘显示
watch(settings, async (newSettings) => {
  console.log("settings: ", newSettings)
  try {
    await updateTrayDisplay(newSettings);
  } catch (error) {
    console.error('更新托盘显示失败:', error);
  }
}, {deep: true});

// 关闭高CPU警告弹窗
function disablePopup() {
  settings.value.enableHighCpuPopup = false;
  // 同时清除所有当前警告
  clearAllAlerts();
}

// 退出应用程序
async function exitApp() {
  try {
    // 调用后端的退出命令
    const {invoke} = await import('@tauri-apps/api/core');
    await invoke('exit_app');
  } catch (error) {
    console.error('退出应用程序失败:', error);
    // 备用方法：尝试关闭当前窗口
    try {
      const {getCurrentWindow} = await import('@tauri-apps/api/window');
      const currentWindow = getCurrentWindow();
      await currentWindow.close();
    } catch (closeError) {
      console.error('关闭窗口失败:', closeError);
    }
  }
}

// 初始化时同步设置到后端
async function initializeBackendSettings() {
  try {
    await updateTrayDisplay(settings.value);
    console.log('已将前端设置同步到后端:', settings.value);
  } catch (error) {
    console.error('初始化后端设置失败:', error);
  }
}

onMounted(() => {
  startAutoRefresh();
  // 延迟一点时间确保后端已经准备好
  setTimeout(initializeBackendSettings, 1000);
});

onUnmounted(() => {
  stopAutoRefresh();
});
</script>

<style scoped>
.main-window {
  margin: 0;
  padding: 12px;
  min-height: 100vh;
  background: #fafbfc;
  color: #1a202c;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
}

.controls {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin-bottom: 12px;
}

.refresh-btn, .auto-refresh-btn, .settings-btn, .exit-btn {
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

.refresh-btn:hover, .auto-refresh-btn:hover, .settings-btn:hover {
  background: #f7fafc;
  border-color: #cbd5e0;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

/* 退出按钮特殊样式 */
.exit-btn {
  background: #fff5f5;
  border-color: #fed7d7;
  color: #e53e3e;
}

.exit-btn:hover {
  background: #feb2b2;
  border-color: #fc8181;
  color: #c53030;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.15);
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
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.processes-section {
  background: #ffffff;
  border-radius: 12px;
  padding: 12px;
  margin: 24px 0;
  border: 1px solid #e2e8f0;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.processes-section h4 {
  margin-top: 0;
  margin-bottom: 16px;
  font-size: 1.0rem;
  font-weight: 600;
  color: #2d3748;
  padding-bottom: 12px;
  border-bottom: 1px solid #e2e8f0;
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
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
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