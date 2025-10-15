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
          <label class="setting-label">状态栏标题内容</label>
          <select v-model="settings.trayDisplayMode" class="select-input">
            <option value="warning-only">持续高CPU占用进程</option>
            <option value="always">最高CPU占用进程</option>
          </select>
        </div>
        <div>
          <div class="setting-item">
            <label class="setting-label">CPU阈值（%）</label>
            <input
                type="number"
                v-model.number="settings.highCpuThreshold"
                class="number-input"
            />
          </div>

          <div class="setting-item">
            <label class="setting-label">持续时间（秒）</label>
            <input
                type="number"
                v-model.number="settings.highCpuDuration"
                class="number-input"
            />
          </div>

          <div v-if="false" class="setting-item">
            <label class="setting-label">
              <input
                  type="checkbox"
                  v-model="settings.enableHighCpuPopup"
              />
              启用高CPU警告弹窗
            </label>
          </div>
        </div>
      </div>
    </div>

    <div class="settings-footer">
      <button @click="resetSettings" class="reset-btn">
        🔄 重置默认
      </button>
      <button @click="$emit('close')" class="save-btn">
        ✅ 保存
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {type Ref} from 'vue';
import {type AppSettings, useSettings} from '../composables/useSettings';

interface Emits {
  (e: 'close'): void;

  (e: 'autoRefreshChange', enabled: boolean, interval: number): void;
}

const emit = defineEmits<Emits>();

const {settings, resetSettings} = useSettings();

// 显式类型声明来帮助TypeScript推断
const typedSettings: Ref<AppSettings> = settings;

function onAutoRefreshChange() {
  emit('autoRefreshChange', typedSettings.value.autoRefresh, typedSettings.value.refreshInterval);
}
</script>

<style scoped>
.settings-panel {
  background: #ffffff;
  border-radius: 12px;
  border: 1px solid #e2e8f0;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
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

.select-input {
  padding: 6px 10px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 14px;
  background: white;
  cursor: pointer;
  min-width: 200px;
  transition: border-color 0.2s ease;
}

.select-input:focus {
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

</style>