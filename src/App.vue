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
  padding: 20px;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

h1 {
  text-align: center;
  margin-bottom: 30px;
  font-size: 2.5rem;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
}

.row {
  display: flex;
  justify-content: center;
  margin-bottom: 20px;
}

.controls {
  display: flex;
  gap: 15px;
}

.refresh-btn, .auto-refresh-btn {
  background: rgba(255, 255, 255, 0.2);
  border: 2px solid rgba(255, 255, 255, 0.3);
  color: white;
  padding: 12px 24px;
  border-radius: 25px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
}

.refresh-btn:hover, .auto-refresh-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  border-color: rgba(255, 255, 255, 0.5);
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

.auto-refresh-btn.active {
  background: rgba(76, 175, 80, 0.8);
  border-color: rgba(76, 175, 80, 1);
}

/* 消息横幅 */
.message-banner {
  padding: 12px 20px;
  margin: 15px 0;
  border-radius: 10px;
  text-align: center;
  font-weight: 500;
  animation: slideDown 0.3s ease-out;
}

.message-banner.success {
  background: rgba(76, 175, 80, 0.9);
  border: 1px solid rgba(76, 175, 80, 1);
}

.message-banner.error {
  background: rgba(244, 67, 54, 0.9);
  border: 1px solid rgba(244, 67, 54, 1);
}

@keyframes slideDown {
  from { opacity: 0; transform: translateY(-10px); }
  to { opacity: 1; transform: translateY(0); }
}

.processes-section {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(15px);
  border-radius: 20px;
  padding: 25px;
  margin: 20px 0;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.processes-section h2 {
  margin-top: 0;
  margin-bottom: 20px;
  font-size: 1.5rem;
}

.no-processes {
  text-align: center;
  padding: 40px;
  font-size: 1.1rem;
  opacity: 0.8;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(255, 255, 255, 0.3);
  border-top: 3px solid white;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.process-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.process-item {
  display: flex;
  align-items: center;
  padding: 15px 20px;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 15px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
}

.process-item:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: translateX(5px);
}

.process-item.high-cpu {
  border-left: 4px solid #ff4757;
  background: rgba(255, 71, 87, 0.1);
}

.process-item.medium-cpu {
  border-left: 4px solid #ffa726;
  background: rgba(255, 167, 38, 0.1);
}

.process-item.low-cpu {
  border-left: 4px solid #4caf50;
  background: rgba(76, 175, 80, 0.1);
}

.process-rank {
  font-size: 1.5rem;
  font-weight: bold;
  margin-right: 20px;
  min-width: 30px;
  text-align: center;
  color: rgba(255, 255, 255, 0.8);
}

.process-info {
  flex: 1;
  margin-right: 20px;
}

.process-name {
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 5px;
}

.process-pid {
  font-size: 0.9rem;
  opacity: 0.7;
}

.process-cpu {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  min-width: 120px;
  margin-right: 20px;
}

.cpu-percentage {
  font-size: 1.2rem;
  font-weight: bold;
  margin-bottom: 5px;
}

.cpu-bar {
  width: 100px;
  height: 8px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  overflow: hidden;
}

.cpu-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, #4caf50, #ffa726, #ff4757);
  border-radius: 4px;
  transition: width 0.3s ease;
}

/* 进程操作按钮 */
.process-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.action-btn {
  padding: 8px 12px;
  border: none;
  border-radius: 8px;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
}

.terminate-btn {
  background: rgba(255, 152, 0, 0.8);
  color: white;
  border: 1px solid rgba(255, 152, 0, 1);
}

.terminate-btn:hover {
  background: rgba(255, 152, 0, 1);
  transform: translateY(-1px);
  box-shadow: 0 3px 10px rgba(255, 152, 0, 0.3);
}

.kill-btn {
  background: rgba(244, 67, 54, 0.8);
  color: white;
  border: 1px solid rgba(244, 67, 54, 1);
}

.kill-btn:hover {
  background: rgba(244, 67, 54, 1);
  transform: translateY(-1px);
  box-shadow: 0 3px 10px rgba(244, 67, 54, 0.3);
}

.restart-btn {
  background: rgba(33, 150, 243, 0.8);
  color: white;
  border: 1px solid rgba(33, 150, 243, 1);
}

.restart-btn:hover {
  background: rgba(33, 150, 243, 1);
  transform: translateY(-1px);
  box-shadow: 0 3px 10px rgba(33, 150, 243, 0.3);
}


/* 响应式设计 */
@media (max-width: 768px) {
  .container {
    padding: 15px;
  }

  .process-item {
    flex-direction: column;
    align-items: stretch;
    text-align: center;
    gap: 15px;
    padding: 20px;
  }

  .process-rank {
    margin-right: 0;
    margin-bottom: 10px;
  }

  .process-info {
    margin-right: 0;
    margin-bottom: 10px;
  }

  .process-cpu {
    align-items: center;
    margin-right: 0;
    margin-bottom: 10px;
  }

  .process-actions {
    justify-content: center;
    flex-wrap: wrap;
  }

  .controls {
    flex-direction: column;
    align-items: center;
  }

  .action-btn {
    min-width: 80px;
    padding: 10px 15px;
    font-size: 13px;
  }
}

@media (max-width: 480px) {
  h1 {
    font-size: 2rem;
  }

  .process-actions {
    gap: 5px;
  }

  .action-btn {
    min-width: 70px;
    padding: 8px 10px;
    font-size: 11px;
  }
}
</style>