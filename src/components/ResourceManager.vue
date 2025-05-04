<script setup lang="ts">
import { ref } from 'vue';
import { ElButton, ElTable, ElTableColumn, ElInput, ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { dirname } from '@tauri-apps/api/path';
import VideoPreview from './VideoPreview.vue';

// 视频文件扩展名
const videoExtensions = ['.mp4', '.webm', '.ogg', '.mov', '.avi'];

const currentPath = ref('');
const fileList = ref([]);
const pathHistory = ref<string[]>([]);

// 视频预览相关
const showVideoPreview = ref(false);
const currentVideoUrl = ref('');

// 检查是否可以返回上一级
const canGoBack = () => pathHistory.value.length > 0;

// 文件类型图标映射
const getFileIcon = (row: any) => {
  if (row.is_dir) return '📁';
  if (isVideoFile(row.name)) return '🎥';
  return '📄';
};

// 判断是否为视频文件
const isVideoFile = (filename: string) => {
  return videoExtensions.some(ext => filename.toLowerCase().endsWith(ext));
};

// 预览视频
const previewVideo = async (path: string) => {
  const fileContent = await invoke('read_file', { path });
  const blob = new Blob([fileContent]);
  const url = URL.createObjectURL(blob);
  currentVideoUrl.value = url;
  showVideoPreview.value = true;
};

// 格式化文件大小
const formatFileSize = (size: number) => {
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)} KB`;
  if (size < 1024 * 1024 * 1024) return `${(size / (1024 * 1024)).toFixed(2)} MB`;
  return `${(size / (1024 * 1024 * 1024)).toFixed(2)} GB`;
};

// 选择文件夹
const selectFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    
    if (selected) {
      currentPath.value = selected as string;
    // 清空历史记录  
    pathHistory.value = [];
      await openFolder(currentPath.value);
    }
  } catch (error) {
    ElMessage.error('选择文件夹失败');
    console.error(error);
  }
};

// 打开文件夹
const openFolder = async (path: string, addToHistory = true) => {
  try {
    const files = await invoke('read_directory', { path });
    fileList.value = files;
    if (addToHistory && currentPath.value) {
      pathHistory.value.push(currentPath.value);
    }
    currentPath.value = path;
  } catch (error) {
    ElMessage.error('读取文件夹失败');
    console.error(error);
  }
};

// 返回上一级目录
const goBack = async () => {
  if (canGoBack()) {
    const previousPath = pathHistory.value[pathHistory.value.length - 2];
    if (previousPath) {
      // 移除当前路径之后的所有历史记录
      pathHistory.value = pathHistory.value.slice(0, -1);
      await openFolder(previousPath, false);
    }
  }
};

// 处理行点击事件
const handleRowClick = (row: any) => {
  if (row.is_dir) {
    currentPath.value = row.path;
    openFolder(row.path, true);
  } else if (isVideoFile(row.name)) {
    previewVideo(row.path);
  }
};

</script>

<template>
  <div class="resource-manager">
    <div class="toolbar">
      <el-button
        @click="goBack"
        :disabled="!canGoBack()"
        type="primary"
        class="back-button"
      >
        返回上一级
      </el-button>
      <el-input
        v-model="currentPath"
        placeholder="当前路径"
        readonly
        class="path-input"
      >
        <template #append>
          <el-button @click="selectFolder">选择文件夹</el-button>
        </template>
      </el-input>
    </div>
    
    <el-table :data="fileList" style="width: 100%" @row-click="handleRowClick">
      <el-table-column label="类型" width="50">
        <template #default="{ row }">
          <span>{{ getFileIcon(row) }}</span>
        </template>
      </el-table-column>
      <el-table-column prop="name" label="名称" />
      <el-table-column label="大小" width="120">
        <template #default="{ row }">
          <span>{{ row.is_dir ? '-' : formatFileSize(row.size) }}</span>
        </template>
      </el-table-column>
      <el-table-column prop="modified_time" label="修改时间" width="180" />
    </el-table>

    <VideoPreview
      v-model:visible="showVideoPreview"
      :video-url="currentVideoUrl"
    />
  </div>
</template>

<style scoped>
.resource-manager {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.toolbar {
  display: flex;
  gap: 10px;
  align-items: center;
}

.path-input {
  flex: 1;
}

.back-button {
  min-width: 100px;
}

.el-table {
  flex: 1;
  overflow: auto;
}
</style>