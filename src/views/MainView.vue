<script setup lang="ts">
import GitProxyManager from './GitProxyManager.vue';
import ResourceManager from './ResourceManager.vue';
import VideoConverter from './VideoConverter.vue';
</script>

<template>
  <main class="main-view">
    <el-tabs type="card" tab-position="top" class="main-tabs">
      <el-tab-pane label="Git 代理管理">
        <GitProxyManager />
      </el-tab-pane>

      <el-tab-pane label="资源管理">
        <ResourceManager />
      </el-tab-pane>

      <el-tab-pane label="视频转换">
        <VideoConverter />
      </el-tab-pane>
    </el-tabs>
  </main>
</template>

<style scoped>
.main-view {
  width: 100%;
  height: 100%;
  min-height: 0;
  overflow: hidden;
}

.main-tabs {
  height: 100%;
  display: flex;
  flex-direction: column-reverse;
}

.main-tabs :deep(.el-tabs__header) {
  flex: none;
  margin: 0;
  user-select: none;
}

.main-tabs :deep(.el-tabs__content) {
  flex: 1;
  min-height: 0;
  overflow: auto;
}

.main-tabs :deep(.el-tab-pane) {
  height: 100%;
  min-height: 0;
  color: #010101;
}
</style>
