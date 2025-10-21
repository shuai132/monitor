<template>
  <div class="process-list">
    <div
        v-for="(process, index) in processes"
        :key="process.pid"
        class="process-item"
        :class="{
        pinned: isPinnedProcess(process),
        [getCpuUsageClass(process.cpu_usage)]: true
      }"
        @click="pinProcess(process, index)"
    >
      <!-- 第一行：进程名 + 内存 + CPU百分比和进度条 -->
      <div class="process-row-1">
        <div class="process-name-section">
          <div class="rank-badge">
            {{ getRealRank(process, index) }}
          </div>
          <div class="process-name" :title="process.name">
            {{ process.name }}
          </div>
        </div>

        <div class="memory-section">
          <div class="memory-usage">
            {{ formatMemoryUsage(process.memory_usage) }}
          </div>
        </div>

        <div class="cpu-section">
          <div class="cpu-percentage">
            {{ process.cpu_usage.toFixed(1) }}%
          </div>
          <div class="cpu-bar-container">
            <div
                class="cpu-bar"
                :style="{ width: Math.min(process.cpu_usage, 100) + '%' }"
            ></div>
          </div>
        </div>
      </div>

      <!-- 第二行：PID + 操作按钮 -->
      <div class="process-row-2">
        <div class="process-pid">
          PID: {{ process.pid }}
        </div>

        <div class="action-buttons">
          <button
              @click.stop="terminateProcess(process.pid)"
              class="action-btn terminate-btn"
              :title="`终止进程 ${process.name}`"
          >
            ⏹️
          </button>
          <button
              @click.stop="forceKillProcess(process.pid)"
              class="action-btn kill-btn"
              :title="`强制终止进程 ${process.name}`"
          >
            ❌
          </button>
          <button
              @click.stop="restartProcess(process.name)"
              class="action-btn restart-btn"
              :title="`重启进程 ${process.name}`"
          >
            🔄
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type {ProcessInfo} from '../composables/useProcesses';

interface Props {
  processes: ProcessInfo[];
  compact?: boolean;
  isPinnedProcess: (process: ProcessInfo) => boolean;
  getRealRank: (process: ProcessInfo, index: number) => number;
  getCpuUsageClass: (cpuUsage: number) => string;
  formatMemoryUsage: (bytes: number) => string;
  pinProcess: (process: ProcessInfo, index: number) => void;
  terminateProcess: (pid: number) => Promise<void>;
  forceKillProcess: (pid: number) => Promise<void>;
  restartProcess: (processName: string) => Promise<void>;
}

defineProps<Props>();
</script>

<style scoped>
.process-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.process-item {
  display: flex;
  flex-direction: column;
  padding: 8px 12px;
  background: #f8fafc;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
  transition: all 0.2s ease;
  gap: 6px;
}

.process-item:hover {
  background: #edf2f7;
  border-color: #cbd5e0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

/* 固定状态样式 */
.process-item.pinned {
  background: #e2e8f0;
  border-color: #94a3b8;
  cursor: pointer;
}

.process-item.pinned:hover {
  background: #cbd5e1;
  border-color: #64748b;
}

.process-item.pinned .rank-badge {
  color: #475569;
  font-weight: 700;
}

/* 第一行：进程名 + CPU */
.process-row-1 {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.process-name-section {
  display: flex;
  align-items: center;
  flex: 1;
  min-width: 0;
}

.rank-badge {
  font-size: 14px;
  font-weight: 600;
  margin-right: 12px;
  min-width: 24px;
  text-align: center;
  color: #718096;
  background: #e2e8f0;
  border-radius: 4px;
  padding: 2px 6px;
  flex-shrink: 0;
}

.process-name {
  font-size: 14px;
  font-weight: 600;
  color: #2d3748;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  min-width: 0;
}

.memory-section {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.memory-usage {
  font-size: 13px;
  font-weight: 500;
  color: #6b7280;
  min-width: 70px;
  text-align: right;
  margin-right: 16px;
}

.cpu-section {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.cpu-percentage {
  font-size: 14px;
  font-weight: 600;
  color: #4a5568;
  min-width: 50px;
  text-align: right;
}

.cpu-bar-container {
  width: 60px;
  height: 6px;
  background: #e2e8f0;
  border-radius: 3px;
  overflow: hidden;
}

.cpu-bar {
  height: 100%;
  border-radius: 3px;
  transition: width 0.3s ease;
}

.process-item.high-cpu .cpu-bar {
  background: #e53e3e;
}

.process-item.medium-cpu .cpu-bar {
  background: #dd6b20;
}

.process-item.low-cpu .cpu-bar {
  background: #38a169;
}

/* 第二行：PID + 操作按钮 */
.process-row-2 {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-left: 36px; /* 和第一行的进程名左对齐 */
}

.process-pid {
  font-size: 12px;
  color: #718096;
  font-weight: 500;
}

/* 进程操作按钮 */
.action-buttons {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.action-btn {
  padding: 4px 6px;
  border: none;
  border-radius: 4px;
  font-size: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 32px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
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

/* 紧凑模式样式 */
.process-item.compact {
  padding: 8px 12px;
  font-size: 12px;
  flex-direction: column;
  align-items: stretch;
  gap: 8px;
}

.process-item.compact .process-main-row {
  display: flex;
  align-items: center;
}

.process-item.compact .process-rank {
  font-size: 14px;
  margin-right: 12px;
  min-width: 20px;
}

.process-item.compact .process-name {
  font-size: 13px;
  font-weight: 500;
}

.process-item.compact .process-cpu {
  min-width: 80px;
  margin-right: 0;
}

.process-item.compact .cpu-percentage {
  font-size: 12px;
  margin-bottom: 4px;
}

.cpu-bar.compact {
  width: 60px;
  height: 4px;
}

.process-item.compact .process-actions {
  justify-content: flex-start;
  margin: 0;
}

.process-item.compact .action-btn {
  min-width: 45px;
  padding: 4px 8px;
  font-size: 10px;
}
</style>