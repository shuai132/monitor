<template>
  <div class="settings-panel">
    <div class="settings-header">
      <h3>⚙️ 应用设置</h3>
      <button @click="$emit('close')" class="close-btn">✕</button>
    </div>

    <div class="settings-content">
      <!-- 刷新设置 -->
      <div class="setting-group">
        <h4>🔄 刷新设置</h4>
        
        <div class="setting-item">
          <label class="setting-label">
            <input 
              type="checkbox" 
              v-model="settings.autoRefresh"
              @change="onAutoRefreshChange"
            />
            启用自动刷新
          </label>
        </div>

        <div class="setting-item" v-if="settings.autoRefresh">
          <label class="setting-label">刷新间隔（秒）</label>
          <input 
            type="number" 
            v-model.number="settings.refreshInterval"
            min="1" 
            max="60" 
            class="number-input"
          />
        </div>
      </div>

      <!-- 托盘显示设置 -->
      <div class="setting-group">
        <h4>📋 托盘显示</h4>
        
        <div class="setting-item">
          <label class="setting-label">
            <input 
              type="checkbox" 
              v-model="settings.trayShowProcess"
            />
            显示进程名称
          </label>
        </div>

        <div class="setting-item">
          <label class="setting-label">
            <input 
              type="checkbox" 
              v-model="settings.trayShowPercentage"
            />
            显示CPU百分比
          </label>
        </div>
      </div>

      <!-- 高CPU警告设置 -->
      <div class="setting-group">
        <h4>⚠️ 高CPU使用率警告</h4>
        
        <div class="setting-item">
          <label class="setting-label">
            <input 
              type="checkbox" 
              v-model="settings.highCpuAlert"
            />
            启用高CPU警告
          </label>
        </div>

        <div v-if="settings.highCpuAlert" class="sub-settings">
          <div class="setting-item">
            <label class="setting-label">CPU阈值（%）</label>
            <input 
              type="number" 
              v-model.number="settings.highCpuThreshold"
              min="50" 
              max="200" 
              class="number-input"
            />
          </div>

          <div class="setting-item">
            <label class="setting-label">持续时间（分钟）</label>
            <input 
              type="number" 
              v-model.number="settings.highCpuDuration"
              min="1" 
              max="60" 
              class="number-input"
            />
          </div>
        </div>
      </div>
    </div>

    <div class="settings-footer">
      <button @click="resetSettings" class="reset-btn">
        🔄 重置为默认
      </button>
      <button @click="$emit('close')" class="save-btn">
        ✅ 保存并关闭
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSettings } from '../composables/useSettings';

interface Emits {
  (e: 'close'): void;
  (e: 'autoRefreshChange', enabled: boolean, interval: number): void;
}

const emit = defineEmits<Emits>();

const { settings, resetSettings } = useSettings();

function onAutoRefreshChange() {
  emit('autoRefreshChange', settings.value.autoRefresh, settings.value.refreshInterval);
}
</script>

<style scoped>
.settings-panel {
  background: #ffffff;
  border-radius: 12px;
  border: 1px solid #e2e8f0;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  max-width: 480px;
  margin: 24px auto;
  overflow: hidden;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  background: #f8fafc;
  border-bottom: 1px solid #e2e8f0;
}

.settings-header h4 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: #2d3748;
}

.close-btn {
  background: none;
  border: none;
  font-size: 18px;
  color: #718096;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: #e2e8f0;
  color: #4a5568;
}

.settings-content {
  padding: 24px;
  max-height: 500px;
  overflow-y: auto;
}

.setting-group {
  margin-bottom: 32px;
}

.setting-group:last-child {
  margin-bottom: 0;
}

.setting-group h4 {
  margin: 0 0 16px 0;
  font-size: 1rem;
  font-weight: 600;
  color: #2d3748;
  padding-bottom: 8px;
  border-bottom: 1px solid #e2e8f0;
}

.setting-item {
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-label {
  display: flex;
  align-items: center;
  font-size: 14px;
  font-weight: 500;
  color: #4a5568;
  cursor: pointer;
  gap: 8px;
}

.setting-label input[type="checkbox"] {
  margin: 0;
  cursor: pointer;
}

.number-input {
  width: 80px;
  padding: 6px 10px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 14px;
  text-align: center;
  transition: border-color 0.2s ease;
}

.number-input:focus {
  outline: none;
  border-color: #3182ce;
  box-shadow: 0 0 0 3px rgba(49, 130, 206, 0.1);
}

.sub-settings {
  margin-left: 24px;
  padding-top: 12px;
  border-left: 2px solid #e2e8f0;
  padding-left: 16px;
}

.sub-settings .setting-item {
  margin-bottom: 12px;
}

.settings-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  background: #f8fafc;
  border-top: 1px solid #e2e8f0;
}

.reset-btn, .save-btn {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.reset-btn {
  background: #f7fafc;
  color: #718096;
  border-color: #e2e8f0;
}

.reset-btn:hover {
  background: #edf2f7;
  color: #4a5568;
  border-color: #cbd5e0;
}

.save-btn {
  background: #38a169;
  color: white;
  border-color: #38a169;
}

.save-btn:hover {
  background: #2f855a;
  border-color: #2f855a;
}

/* 响应式设计 */
@media (max-width: 480px) {
  .settings-panel {
    margin: 16px;
    max-width: none;
  }

  .settings-header, .settings-content, .settings-footer {
    padding: 16px;
  }

  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .number-input {
    width: 100%;
    max-width: 120px;
  }

  .settings-footer {
    flex-direction: column;
    gap: 12px;
  }

  .reset-btn, .save-btn {
    width: 100%;
  }
}
</style>