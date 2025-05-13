<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { ElButton, ElDivider, ElMessage } from 'element-plus';
import {  Delete } from '@element-plus/icons-vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

interface FileItem {
  is_dir: boolean;
  name: string;
  size: number;
  modified_time: string;
  path: string;
}

// 分类列表
const categories = ref<Array<{id: number, name: string, files: FileItem[]}>>([
  { id: 1, name: '图片', files: [] },
  { id: 2, name: '视频', files: [] },
  { id: 3, name: '音频', files: [] },
  { id: 4, name: '文档', files: [] },
  { id: 5, name: '其他', files: [] }
]);

// 当前拖拽的文件
const draggedFile = ref<FileItem | null>(null);

// 处理拖拽进入分类区域
const handleDragOver = (event: DragEvent, categoryId: number) => {
  event.preventDefault();
  const target = event.currentTarget as HTMLElement;
  target.classList.add('drag-over');
};

// 处理拖拽离开分类区域
const handleDragLeave = (event: DragEvent) => {
  event.preventDefault();
  const target = event.currentTarget as HTMLElement;
  target.classList.remove('drag-over');
};

// 处理放置文件到分类区域
const handleDrop = (event: DragEvent, categoryId: number) => {
  console.log('drop', event, categoryId);
  event.preventDefault();
  const target = event.currentTarget as HTMLElement;
  target.classList.remove('drag-over');
  
  // 获取拖拽的数据
  const fileData = event.dataTransfer?.getData('application/json');
  if (fileData) {
    try {
      const file = JSON.parse(fileData) as FileItem;
      // 查找目标分类
      const category = categories.value.find(c => c.id === categoryId);
      if (category) {
        // 检查文件是否已存在于该分类
        const exists = category.files.some((f: FileItem) => f.path === file.path);
        if (!exists) {
          category.files.push(file);
          ElMessage.success(`添加成功`);
        } else {
          ElMessage.warning(`已存在${file.name}`);
        }
      }
    } catch (error) {
      console.error('解析拖拽数据失败:', error);
      ElMessage.error('添加文件到分类失败');
    }
  }
};

// 从分类中移除文件
const removeFile = (categoryId: number, filePath: string) => {
  const category = categories.value.find(c => c.id === categoryId);
  if (category) {
    category.files = category.files.filter((f: FileItem) => f.path !== filePath);
    ElMessage.success('已从分类中移除文件');
  }
};

// 格式化文件大小
const formatFileSize = (size: number) => {
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)} KB`;
  if (size < 1024 * 1024 * 1024) return `${(size / (1024 * 1024)).toFixed(2)} MB`;
  return `${(size / (1024 * 1024 * 1024)).toFixed(2)} GB`;
};

// 窗口大小相关
const windowHeight = ref(0);
const scaleFactor = ref(1);

// 计算窗口高度
const calculateWindowHeight = async () => {
  windowHeight.value = (await getCurrentWindow().innerSize()).height;
  scaleFactor.value = await getCurrentWindow().scaleFactor();
};

// 监听窗口大小变化
const handleResize = () => {
  calculateWindowHeight();
};

let unlistenDragDrop: UnlistenFn;

onMounted(async () => {
  // 初始计算高度
  await calculateWindowHeight();
  // 添加窗口大小变化监听
  window.addEventListener('resize', handleResize);
  

  // 监听来自Webview的拖拽文件事件
  unlistenDragDrop = await listen<FileItem>('tauri://drag-enter', (event) => {
    console.log(`Got error, payload: ${event.payload}`);

    const file = event.payload;
    // 将文件数据设置为可拖拽状态
    draggedFile.value = file;
  });
});

onUnmounted(() => {
  // 移除监听
  window.removeEventListener('resize', handleResize);
  
  // 移除拖拽文件事件监听
  if (unlistenDragDrop) {
    unlistenDragDrop();
  }
});
</script>

<template>
  <div class="file-classifier">
    <h2>文件分类</h2>
    <p class="tip">将文件从资源管理器拖拽到下方分类中</p>
    
    <div class="categories-container">
      <div 
        v-for="category in categories" 
        :key="category.id"
        class="category-card"
        @dragover="handleDragOver($event, category.id)"
        @dragleave="handleDragLeave($event)"
        @drop="handleDrop($event, category.id)"
      >
        <div class="category-header">
          <h3>{{ category.name }}</h3>
          <span class="file-count">{{ category.files.length }} 个文件</span>
        </div>
        
        <el-divider />
        
        <div class="file-list">
          <div v-if="category.files.length === 0" class="empty-tip">
            拖拽文件到此处
          </div>
          <div 
            v-for="(file, index) in category.files" 
            :key="index"
            class="file-item"
          >
            <div class="file-info">
              <span class="file-name">{{ file.name }}</span>
              <span class="file-size">{{ formatFileSize(file.size) }}</span>
            </div>
            <el-button 
              type="danger" 
              size="small" 
              circle
              @click="removeFile(category.id, file.path)"
            >
              <el-icon><Delete /></el-icon>
            </el-button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.file-classifier {
  height: 100%;
  overflow: auto;
}

.tip {
  color: #909399;
  margin-bottom: 20px;
}

.categories-container {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  justify-content: center;
}

.category-card {
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 15px;
  background-color: #fff;
  transition: all 0.3s;
  height: 300px;
  width: 100px;
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.category-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.file-count {
  font-size: 12px;
  color: #909399;
}

.file-list {
  flex: 1;
  overflow-y: auto;
  padding: 5px 0;
}

.empty-tip {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  color: #909399;
  font-style: italic;
}

.file-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  border-bottom: 1px solid #f0f0f0;
  flex-shrink: 0;
}

.file-item:last-child {
  border-bottom: none;
}

.file-info {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  flex-grow: 1;
  min-width: 0;
}

.file-name {
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 200px;
}

.file-size {
  font-size: 12px;
  color: #909399;
}

.drag-over {
  border: 2px dashed #409eff;
  background-color: rgba(64, 158, 255, 0.1);
}
</style>