<template>
  <div class="process-list">
    <div
      v-for="(process, index) in processes"
      :key="process.pid"
      class="process-item"
      :class="[
        getCpuUsageClass(process.cpu_usage),
        { 'pinned': isPinnedProcess(process), 'compact': compact }
      ]"
      @click="pinProcess(process, index)"
    >
      <!-- 紧凑模式：两行布局 -->
      <template v-if="compact">
        <!-- 第一行：序号、进程名、PID、百分比、进度条 -->
        <div class="process-main-row">
          <div class="process-rank">
            {{ isPinnedProcess(process) ? getRealRank(process, index) : index + 1 }}
          </div>

          <div class="process-info">
            <div class="process-name">{{ process.name }} ({{ process.pid }})</div>
          </div>

          <div class="process-cpu">
            <div class="cpu-percentage">{{ process.cpu_usage.toFixed(1) }}%</div>
            <div class="cpu-bar compact">
              <div
                class="cpu-bar-fill"
                :style="{ width: Math.min(process.cpu_usage, 100) + '%' }"
              ></div>
            </div>
          </div>
        </div>

        <!-- 第二行：操作按钮 -->
        <div class="process-actions" @click.stop>
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
      </template>

      <!-- 普通模式：一行布局 -->
      <template v-else>
        <div class="process-rank">
          {{ isPinnedProcess(process) ? getRealRank(process, index) : index + 1 }}
        </div>

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

        <div class="process-actions" @click.stop>
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
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProcessInfo } from '../composables/useProcesses';

interface Props {
  processes: ProcessInfo[];
  compact?: boolean;
  isPinnedProcess: (process: ProcessInfo) => boolean;
  getRealRank: (process: ProcessInfo, index: number) => number;
  getCpuUsageClass: (cpuUsage: number) => string;
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

.process-item.pinned .process-rank {
  color: #475569;
  font-weight: 700;
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