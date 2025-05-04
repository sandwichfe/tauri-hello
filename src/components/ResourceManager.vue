<script setup lang="ts">
import { ref } from 'vue';
import { ElButton, ElTable, ElTableColumn, ElInput, ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

const currentPath = ref('');
const fileList = ref([]);

// 文件类型图标映射
const getFileIcon = (isDir: boolean) => {
  return isDir ? '📁' : '📄';
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
      await openFolder(currentPath.value);
    }
  } catch (error) {
    ElMessage.error('选择文件夹失败');
    console.error(error);
  }
};

// 打开文件夹
const openFolder = async (path: string) => {
  try {
    const files = await invoke('read_directory', { path });
    fileList.value = files;
  } catch (error) {
    ElMessage.error('读取文件夹失败');
    console.error(error);
  }
};

// 处理行点击事件
const handleRowClick = (row: any) => {
  if (row.is_dir) {
    currentPath.value = row.path;
    openFolder(row.path);
  }
};

</script>

<template>
  <div class="resource-manager">
    <div class="toolbar">
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
          <span>{{ getFileIcon(row.is_dir) }}</span>
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
}

.path-input {
  flex: 1;
}

.el-table {
  flex: 1;
  overflow: auto;
}
</style>