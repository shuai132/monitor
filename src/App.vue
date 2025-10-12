<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import MainWindow from './components/MainWindow.vue';
import TrayPopup from './components/TrayPopup.vue';
import HighCpuTrayPopup from './components/HighCpuTrayPopup.vue';

const isTrayPopup = ref(false);
const isHighCpuAlert = ref(false);

onMounted(async () => {
  // 检测当前窗口类型
  const currentWindow = getCurrentWindow();
  const label = await currentWindow.label;
  isTrayPopup.value = label === 'tray-popup';
  isHighCpuAlert.value = label === 'high-cpu-alert';
});
</script>

<template>
  <!-- 根据窗口类型渲染不同的组件 -->
  <TrayPopup v-if="isTrayPopup" />
  <HighCpuTrayPopup v-else-if="isHighCpuAlert" />
  <MainWindow v-else />
</template>

<style>
/* 全局样式重置 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
