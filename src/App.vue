<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface ProcessInfo {
  name: string;
  pid: number;
  cpu_usage: number;
}

const processes = ref<ProcessInfo[]>([]);
const isAutoRefresh = ref(false);
const isLoading = ref(false);
const message = ref("");
let refreshInterval: number | null = null;


async function getTopProcesses() {
  isLoading.value = true;
  try {
    const result = await invoke<ProcessInfo[]>("get_top_cpu_processes");
    processes.value = result;
  } catch (error) {
    console.error("获取进程信息失败:", error);
    processes.value = [];
  } finally {
    isLoading.value = false;
  }
}

async function terminateProcess(pid: number) {
  try {
    const result = await invoke<string>("terminate_process", { pid });
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
    const result = await invoke<string>("force_kill_process", { pid });
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
    const result = await invoke<string>("restart_process", { processName });
    message.value = result;
    setTimeout(() => message.value = "", 3000);
  } catch (error) {
    message.value = `重启进程失败: ${error}`;
    setTimeout(() => message.value = "", 3000);
  }
}

function toggleAutoRefresh() {
  isAutoRefresh.value = !isAutoRefresh.value;

  if (isAutoRefresh.value) {
    refreshInterval = setInterval(getTopProcesses, 2000);
  } else {
    if (refreshInterval) {
      clearInterval(refreshInterval);
      refreshInterval = null;
    }
  }
}

function getCpuUsageClass(cpuUsage: number) {
  if (cpuUsage > 50) return 'high-cpu';
  if (cpuUsage > 20) return 'medium-cpu';
  return 'low-cpu';
}

onMounted(() => {
  getTopProcesses();
});

onUnmounted(() => {
  if (refreshInterval) {
    clearInterval(refreshInterval);
  }
});
</script>

<template>
  <main class="container">
    <h1>🖥️ CPU 监控器</h1>

    <div class="row">
      <div class="controls">
        <button @click="getTopProcesses" :disabled="isLoading" class="refresh-btn">
          🔄 {{ isLoading ? '加载中...' : '手动刷新' }}
        </button>

        <button @click="toggleAutoRefresh" :class="{ active: isAutoRefresh }" class="auto-refresh-btn">
          {{ isAutoRefresh ? '⏸️ 停止自动刷新' : '▶️ 开启自动刷新' }}
        </button>
      </div>
    </div>

    <!-- 消息提示 -->
    <div v-if="message" class="message-banner" :class="message.includes('失败') ? 'error' : 'success'">
      {{ message }}
    </div>

    <div class="processes-section">
      <h2>📊 CPU 占用率前10进程</h2>

      <div v-if="processes.length === 0" class="no-processes">
        <div class="loading-spinner" v-if="isLoading"></div>
        <p>{{ isLoading ? '正在获取进程信息...' : '暂无进程数据' }}</p>
      </div>

      <div v-else class="process-list">
        <div
          v-for="(process, index) in processes"
          :key="process.pid"
          class="process-item"
          :class="getCpuUsageClass(process.cpu_usage)"
        >
          <div class="process-rank">{{ index + 1 }}</div>

          <div class="process-info">
            <div class="process-name">{{ process.name }}</div>
            <div class="process-pid">PID: {{ process.pid }}</div>
          </div>

          <div class="process-cpu">
            <div class="cpu-percentage">{{ process.cpu_usage.toFixed(1) }}%</div>
            <div class="cpu-bar">
              <div
                class="cpu-bar-fill"
                :style="{ width: Math.min(process.cpu_usage, 100) + '%' }"
              ></div>
            </div>
          </div>

          <div class="process-actions">
            <button
              @click="terminateProcess(process.pid)"
              class="action-btn terminate-btn"
              title="优雅终止进程"
            >
              🛑 终止
            </button>

            <button
              @click="forceKillProcess(process.pid)"
              class="action-btn kill-btn"
              title="强制终止进程"
            >
              💀 强杀
            </button>

            <button
              @click="restartProcess(process.name)"
              class="action-btn restart-btn"
              title="重启应用程序"
            >
              🔄 重启
            </button>
          </div>
        </div>
      </div>
    </div>

  </main>
</template>

<style scoped>
.container {
  margin: 0;
  padding: 24px;
  min-height: 100vh;
  background: #fafbfc;
  color: #1a202c;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
}

h1 {
  text-align: center;
  margin-bottom: 32px;
  font-size: 2.25rem;
  font-weight: 700;
  color: #2d3748;
  letter-spacing: -0.025em;
}

.row {
  display: flex;
  justify-content: center;
  margin-bottom: 24px;
}

.controls {
  display: flex;
  gap: 12px;
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

.process-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.process-item {
  display: flex;
  align-items: center;
  padding: 16px;
  background: #f8fafc;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
  transition: all 0.2s ease;
}

.process-item:hover {
  background: #edf2f7;
  border-color: #cbd5e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}


.process-rank {
  font-size: 1.125rem;
  font-weight: 600;
  margin-right: 16px;
  min-width: 24px;
  text-align: center;
  color: #718096;
}

.process-info {
  flex: 1;
  margin-right: 16px;
}

.process-name {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 4px;
  color: #2d3748;
}

.process-pid {
  font-size: 12px;
  color: #718096;
}

.process-cpu {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  min-width: 100px;
  margin-right: 16px;
}

.cpu-percentage {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 6px;
  color: #4a5568;
}

.cpu-bar {
  width: 80px;
  height: 6px;
  background: #e2e8f0;
  border-radius: 3px;
  overflow: hidden;
}

.cpu-bar-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.3s ease;
}

.process-item.high-cpu .cpu-bar-fill {
  background: #e53e3e;
}

.process-item.medium-cpu .cpu-bar-fill {
  background: #dd6b20;
}

.process-item.low-cpu .cpu-bar-fill {
  background: #38a169;
}

/* 进程操作按钮 */
.process-actions {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
}

.action-btn {
  padding: 6px 10px;
  border: none;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 50px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 2px;
}

.terminate-btn {
  background: #f8fafc;
  color: #64748b;
  border: 1px solid #e2e8f0;
}

.terminate-btn:hover {
  background: #f1f5f9;
  color: #475569;
  border-color: #cbd5e1;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.kill-btn {
  background: #f8fafc;
  color: #64748b;
  border: 1px solid #e2e8f0;
}

.kill-btn:hover {
  background: #fef2f2;
  color: #dc2626;
  border-color: #fecaca;
  box-shadow: 0 1px 2px rgba(220, 38, 38, 0.1);
}

.restart-btn {
  background: #f8fafc;
  color: #64748b;
  border: 1px solid #e2e8f0;
}

.restart-btn:hover {
  background: #f0f9ff;
  color: #0369a1;
  border-color: #bae6fd;
  box-shadow: 0 1px 2px rgba(3, 105, 161, 0.1);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .container {
    padding: 16px;
  }

  h1 {
    font-size: 1.875rem;
  }

  .process-item {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
    padding: 16px;
  }

  .process-rank {
    margin-right: 0;
    text-align: left;
  }

  .process-info {
    margin-right: 0;
  }

  .process-cpu {
    align-items: flex-start;
    margin-right: 0;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }

  .cpu-bar {
    width: 60px;
  }

  .process-actions {
    justify-content: flex-start;
    flex-wrap: wrap;
  }

  .controls {
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .action-btn {
    min-width: 60px;
    padding: 8px 12px;
    font-size: 12px;
  }
}

@media (max-width: 480px) {
  .container {
    padding: 12px;
  }

  h1 {
    font-size: 1.5rem;
  }

  .processes-section {
    padding: 16px;
  }

  .process-actions {
    gap: 4px;
  }

  .action-btn {
    min-width: 50px;
    padding: 6px 8px;
    font-size: 10px;
  }
}
</style>
