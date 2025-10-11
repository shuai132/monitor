<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface ProcessInfo {
  name: string;
  pid: number;
  cpu_usage: number;
}

const processes = ref<ProcessInfo[]>([]);
const loading = ref(false);
const error = ref("");
const autoRefresh = ref(true);
let refreshInterval: number | null = null;

async function fetchTopCPUProcesses() {
  try {
    loading.value = true;
    error.value = "";
    const result = await invoke<ProcessInfo[]>("get_top_cpu_processes");
    processes.value = result;
  } catch (err) {
    error.value = `获取进程信息失败: ${err}`;
    console.error("获取CPU进程信息失败:", err);
  } finally {
    loading.value = false;
  }
}

function startAutoRefresh() {
  if (refreshInterval) return;

  refreshInterval = setInterval(() => {
    if (autoRefresh.value) {
      fetchTopCPUProcesses();
    }
  }, 2000); // 每2秒刷新一次
}

function stopAutoRefresh() {
  if (refreshInterval) {
    clearInterval(refreshInterval);
    refreshInterval = null;
  }
}

function toggleAutoRefresh() {
  autoRefresh.value = !autoRefresh.value;
  if (autoRefresh.value) {
    startAutoRefresh();
  } else {
    stopAutoRefresh();
  }
}

onMounted(() => {
  fetchTopCPUProcesses();
  startAutoRefresh();
});

onUnmounted(() => {
  stopAutoRefresh();
});
</script>

<template>
  <main class="container">
    <h1>🖥️ CPU 监控器</h1>

    <div class="controls">
      <button @click="fetchTopCPUProcesses" :disabled="loading" class="refresh-btn">
        {{ loading ? "刷新中..." : "🔄 手动刷新" }}
      </button>
      <button @click="toggleAutoRefresh" class="toggle-btn" :class="{ active: autoRefresh }">
        {{ autoRefresh ? "⏸️ 停止自动刷新" : "▶️ 开启自动刷新" }}
      </button>
    </div>

    <div v-if="error" class="error">
      {{ error }}
    </div>

    <div class="processes-container">
      <h2>CPU 占用率最高的前10个进程</h2>

      <div v-if="loading && processes.length === 0" class="loading">
        加载中...
      </div>

      <div v-else-if="processes.length > 0" class="processes-list">
        <div class="process-header">
          <span class="col-name">进程名称</span>
          <span class="col-pid">进程ID</span>
          <span class="col-cpu">CPU 使用率</span>
        </div>

        <div
          v-for="(process, index) in processes"
          :key="process.pid"
          class="process-item"
          :class="{ 'high-usage': process.cpu_usage > 50 }"
        >
          <div class="process-rank">{{ index + 1 }}</div>
          <div class="process-info">
            <span class="process-name" :title="process.name">{{ process.name }}</span>
            <span class="process-pid">PID: {{ process.pid }}</span>
            <div class="cpu-usage">
              <div class="cpu-bar">
                <div
                  class="cpu-fill"
                  :style="{ width: Math.min(process.cpu_usage, 100) + '%' }"
                ></div>
              </div>
              <span class="cpu-text">{{ process.cpu_usage.toFixed(2) }}%</span>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="no-data">
        暂无进程数据
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  padding: 20px;
  max-width: 1000px;
  margin: 0 auto;
}

h1 {
  color: #2c3e50;
  margin-bottom: 30px;
  font-size: 2.5em;
}

.controls {
  display: flex;
  gap: 15px;
  justify-content: center;
  margin-bottom: 30px;
}

.refresh-btn, .toggle-btn {
  padding: 12px 24px;
  border-radius: 8px;
  border: none;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 14px;
}

.refresh-btn {
  background: linear-gradient(45deg, #3498db, #2980b9);
  color: white;
}

.refresh-btn:hover {
  background: linear-gradient(45deg, #2980b9, #1f5f8b);
  transform: translateY(-2px);
}

.refresh-btn:disabled {
  background: #95a5a6;
  cursor: not-allowed;
  transform: none;
}

.toggle-btn {
  background: linear-gradient(45deg, #27ae60, #229954);
  color: white;
}

.toggle-btn:hover {
  background: linear-gradient(45deg, #229954, #1e8449);
  transform: translateY(-2px);
}

.toggle-btn:not(.active) {
  background: linear-gradient(45deg, #e74c3c, #c0392b);
}

.toggle-btn:not(.active):hover {
  background: linear-gradient(45deg, #c0392b, #a93226);
}

.error {
  background: #ffe6e6;
  color: #c0392b;
  padding: 15px;
  border-radius: 8px;
  margin-bottom: 20px;
  border-left: 4px solid #e74c3c;
}

.processes-container {
  background: white;
  border-radius: 12px;
  padding: 25px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  border: 1px solid #ecf0f1;
}

.processes-container h2 {
  color: #2c3e50;
  margin-bottom: 20px;
  font-size: 1.5em;
}

.loading {
  text-align: center;
  padding: 40px;
  font-size: 1.2em;
  color: #7f8c8d;
}

.no-data {
  text-align: center;
  padding: 40px;
  color: #7f8c8d;
  font-style: italic;
}

.process-header {
  display: grid;
  grid-template-columns: 150px 100px 1fr;
  gap: 20px;
  padding: 15px 20px;
  background: linear-gradient(45deg, #34495e, #2c3e50);
  color: white;
  border-radius: 8px;
  margin-bottom: 15px;
  font-weight: 600;
}

.processes-list {
  border-radius: 8px;
  overflow: hidden;
}

.process-item {
  display: flex;
  align-items: center;
  padding: 15px 20px;
  border-bottom: 1px solid #ecf0f1;
  transition: all 0.3s ease;
  background: white;
}

.process-item:hover {
  background: #f8f9fa;
  transform: translateX(5px);
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.process-item.high-usage {
  border-left: 4px solid #e74c3c;
  background: linear-gradient(90deg, #ffe6e6, white);
}

.process-rank {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background: linear-gradient(45deg, #3498db, #2980b9);
  color: white;
  border-radius: 50%;
  font-weight: bold;
  margin-right: 20px;
  font-size: 16px;
}

.process-info {
  display: grid;
  grid-template-columns: 150px 100px 1fr;
  gap: 20px;
  flex: 1;
  align-items: center;
}

.process-name {
  font-weight: 600;
  color: #2c3e50;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.process-pid {
  color: #7f8c8d;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 13px;
}

.cpu-usage {
  display: flex;
  align-items: center;
  gap: 15px;
}

.cpu-bar {
  flex: 1;
  height: 20px;
  background: #ecf0f1;
  border-radius: 10px;
  overflow: hidden;
  position: relative;
}

.cpu-fill {
  height: 100%;
  background: linear-gradient(45deg, #27ae60, #2ecc71);
  border-radius: 10px;
  transition: width 0.5s ease;
  position: relative;
}

.process-item.high-usage .cpu-fill {
  background: linear-gradient(45deg, #e74c3c, #ec7063);
}

.cpu-text {
  min-width: 60px;
  text-align: right;
  font-weight: 600;
  color: #2c3e50;
  font-family: 'Monaco', 'Courier New', monospace;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .container {
    background-color: #1a1a1a;
  }

  h1 {
    color: #ecf0f1;
  }

  .processes-container {
    background: #2c3e50;
    border-color: #34495e;
  }

  .processes-container h2 {
    color: #ecf0f1;
  }

  .process-item {
    background: #34495e;
    border-color: #3c5a78;
  }

  .process-item:hover {
    background: #3c5a78;
  }

  .process-name {
    color: #ecf0f1;
  }

  .cpu-text {
    color: #ecf0f1;
  }

  .error {
    background: #4a2c2c;
    color: #e74c3c;
    border-color: #e74c3c;
  }
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>