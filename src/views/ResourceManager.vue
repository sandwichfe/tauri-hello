<script setup lang="ts">
import { ref, onMounted,onUnmounted,computed } from 'vue';
import { ElButton, ElTable, ElTableColumn, ElInput, ElMessage, ElIcon } from 'element-plus';
import { ArrowLeft } from '@element-plus/icons-vue';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile, writeTextFile, BaseDirectory, mkdir, exists } from '@tauri-apps/plugin-fs';
import VideoPreview from '../components/VideoPreview.vue';
import ImagePreview from '../components/ImagePreview.vue';
import { openLoading, closeLoading } from "../../src/utils/loadingUtil";
import { getCurrentWindow,  } from '@tauri-apps/api/window';

interface FileItem {
  is_dir: boolean;
  name: string;
  size: number;
  modified_time: string;
  path: string;
}

// 视频文件扩展名
const videoExtensions = ['.mp4', '.webm', '.ogg', '.mov', '.avi'];
// 音频文件扩展名
const audioExtensions = ['.mp3', '.wav', '.aac', '.flac', '.m4a'];
// 图片文件扩展名
const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp'];

const currentPath = ref('');
const fileList = ref<FileItem[]>([]);
const imageFiles = ref<Map<string, number>>(new Map());
const pathHistory = ref<string[]>([]);
const configPath = ref('pathConfig.json');

// 排序相关
const sortColumn = ref('');
const sortOrder = ref('');

// 预览相关
const showVideoPreview = ref(false);
const currentVideoUrl = ref('');
const showImagePreview = ref(false);
const currentImageUrl = ref<string[]>([]);
const currentIndex = ref(0);

// 导航栏高度
const tabsHeight = ref(42.8);
const toolbarHeight = ref(32);

// 系统设置的缩放率
const scaleFactor = ref(1);
const windowsHeight = ref(0);
// 计算高度（使用 computed 让高度保持响应式）
const scrollbarHeight = computed(() => `${(windowsHeight.value/scaleFactor.value)  - tabsHeight.value - toolbarHeight.value  - 20-0.1  }px`);

const scrollRef = ref<any | null>(null);



// 计算滚动区域高度
const calculateScrollbarHeight = async () => {
  windowsHeight.value = (await getCurrentWindow().innerSize()).height;
  scaleFactor.value = await getCurrentWindow().scaleFactor();

  if (scrollRef.value?.$el) {
      scrollRef.value.$el.style.height = scrollbarHeight.value;
      console.log('scrollbarHeight:', scrollbarHeight.value);
  }
  
};

// 监听窗口大小变化
const handleResize = () => {
  calculateScrollbarHeight();
};

// 检查是否可以返回上一级
const canGoBack = () => pathHistory.value.length > 0;

// 文件类型图标映射
const getFileIcon = (row: any) => {
  if (row.is_dir) return '📁';
  if (isVideoFile(row.name)) return '🎥';
  if (isAudioFile(row.name)) return '🔊';
  if (isImageFile(row.name)) return '🖼️';
  return '📄';
};

// 判断是否为视频文件
const isVideoFile = (filename: string) => {
  return videoExtensions.some(ext => filename.toLowerCase().endsWith(ext));
};

// 判断是否为音频文件
const isAudioFile = (filename: string) => {
  return audioExtensions.some(ext => filename.toLowerCase().endsWith(ext));
};

// 判断是否为图片文件
const isImageFile = (filename: string) => {
  return imageExtensions.some(ext => filename.toLowerCase().endsWith(ext));
};

// 预览视频
const previewVideo = async (path: string) => {
  try {
    openLoading();
    // 使用 Tauri 的 convertFileSrc 函数将文件路径转换为可用于视频预览的  URL (tauri 的protocol协议实现的  要在配置文件开权限)
    const assetUrl = await convertFileSrc(path);
    console.log('assetUrl：', assetUrl);
    currentVideoUrl.value = assetUrl;
    showVideoPreview.value = true;
    closeLoading();
  } catch (error) {
    ElMessage.error('视频加载失败');
    console.error(error);
  }
};

// 预览图片
const previewImage = async (path: string) => {
  try {
    openLoading();
    const assetUrl = await convertFileSrc(path);
    console.log('assetUrl：', assetUrl);
    currentImageUrl.value = await Promise.all(Array.from(imageFiles.value).map(async ([path, index]) => await convertFileSrc(path)));
    // 计算当前点击图片在imageFiles中的索引
    currentIndex.value = imageFiles.value.get(path) || 0;
    // 渲染图片预览
    showImagePreview.value = true;
    closeLoading();
  } catch (error) {
    ElMessage.error('图片加载失败');
    console.error(error);
  }
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
      // 保存路径配置
      await savePathConfig();
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
    const files = await invoke<FileItem[]>('read_directory', { path });
    fileList.value = files;
    imageFiles.value = new Map(files
  .filter(file => !file.is_dir && isImageFile(file.name))
  .map((file, index) => [file.path, index]));
    if (addToHistory && currentPath.value) {
      pathHistory.value.push(currentPath.value);
    }
    currentPath.value = path;
    
    // 应用之前的排序（如果有）
    if (sortColumn.value && sortOrder.value) {
      applySorting(sortColumn.value, sortOrder.value);
    }
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

// 处理行双击事件
const handleRowClick = (row: any) => {
  if (row.is_dir) {
    currentPath.value = row.path;
    openFolder(row.path, true);
  } else if (isVideoFile(row.name)) {
    previewVideo(row.path);
  } else if (isAudioFile(row.name)) {
    previewVideo(row.path);
  } else if (isImageFile(row.name)) {
    previewImage(row.path);
  }
};

// 处理排序变化
const handleSortChange = ({ column, prop, order }: any) => {
  sortColumn.value = prop;
  sortOrder.value = order;
  
  if (!order) {
    // 如果取消排序，不做任何操作
    return;
  }
  
  applySorting(prop, order);
};

// 应用排序
// 保存路径配置
const savePathConfig = async () => {
  try {
    // 确保目录存在
    const dirExists = await exists('', { baseDir: BaseDirectory.AppConfig });
    if (!dirExists) {
      await mkdir('', {
        recursive: true,
        baseDir: BaseDirectory.AppConfig
      });
    }
    // 写入配置文件
    await writeTextFile(configPath.value, JSON.stringify({ currentPath: currentPath.value }), {
      baseDir: BaseDirectory.AppConfig,
      create: true
    });
  } catch (error) {
    console.error('保存路径配置失败:', error);
  }
};

// 加载路径配置
onMounted(async () => {
  try {
    const config = await readTextFile(configPath.value, {
      baseDir: BaseDirectory.AppConfig
    });
    
    const { currentPath: savedPath } = JSON.parse(config);
    if (savedPath) {
      currentPath.value = savedPath;
      await openFolder(currentPath.value);
    }
    
    // 初始计算高度
    calculateScrollbarHeight();
    // 添加窗口大小变化监听
    window.addEventListener('resize', handleResize);
  } catch (error) {
    console.log('首次运行或配置文件不存在，使用默认值');
  }
});

onUnmounted(() => {
  // 移除监听
  window.removeEventListener('resize', handleResize);
});

const applySorting = (prop: string, order: string) => {
  const sortedList = [...fileList.value];
  
  // 首先按文件夹/文件类型排序（文件夹始终在前）
  sortedList.sort((a, b) => {
    // 如果正在按类型排序，则遵循排序方向
    if (prop === 'is_dir') {
      return order === 'ascending' 
        ? Number(a.is_dir) - Number(b.is_dir)
        : Number(b.is_dir) - Number(a.is_dir);
    }
    
    // 否则，文件夹始终在前（除非正在按其他属性排序）
    if (a.is_dir !== b.is_dir) {
      return a.is_dir ? -1 : 1;
    }
    
    // 然后按指定属性排序
    if (prop === 'name') {
      return order === 'ascending'
        ? a.name.localeCompare(b.name)
        : b.name.localeCompare(a.name);
    } else if (prop === 'size') {
      // 文件夹大小显示为 '-'，所以我们给它们一个默认值 -1
      const sizeA = a.is_dir ? -1 : a.size;
      const sizeB = b.is_dir ? -1 : b.size;
      return order === 'ascending' ? sizeA - sizeB : sizeB - sizeA;
    } else if (prop === 'modified_time') {
      return order === 'ascending'
        ? a.modified_time.localeCompare(b.modified_time)
        : b.modified_time.localeCompare(a.modified_time);
    }
    
    return 0;
  });
  
  fileList.value = sortedList;
};

</script>

<template>
  <div class="resource-manager">
    <div class="toolbar">
      <el-button
        @click="goBack"
        :disabled="!canGoBack()"
        type="info"
        class="back-button"
      >
        <el-icon><arrow-left /></el-icon>
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
    
    <el-scrollbar  :native="true"  ref="scrollRef">
    <el-table 
      :data="fileList" 
      style="width: 100%" 
      @row-dblclick="handleRowClick"
      @sort-change="handleSortChange"
    >
      <el-table-column 
        label="类型" 
        width="80"
        prop="is_dir"
        sortable="custom"
      >
        <template #default="{ row }">
          <span>{{ getFileIcon(row) }}</span>
        </template>
      </el-table-column>
      <el-table-column 
        prop="name" 
        label="名称"
        sortable="custom" 
      />
      <el-table-column 
        label="大小" 
        width="120"
        prop="size"
        sortable="custom"
      >
        <template #default="{ row }">
          <span>{{ row.is_dir ? '-' : formatFileSize(row.size) }}</span>
        </template>
      </el-table-column>
      <el-table-column 
        prop="modified_time" 
        label="修改时间" 
        width="180"
        sortable="custom" 
      />
    </el-table>
  </el-scrollbar>
  <div class="file-count">
      共 {{ fileList.length }} 个文件
    </div>

    <VideoPreview
      v-model:visible="showVideoPreview"
      :video-url="currentVideoUrl"
    />
    <ImagePreview
      v-model:visible="showImagePreview"
      :image-urls="currentImageUrl"
      :initial-index="currentIndex"
    />
  </div>
</template>

<style scoped>
.resource-manager {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.toolbar {
  display: flex;
  gap: 10px;
  align-items: center;
   /* 避免工具栏被压缩 */
  flex-shrink: 0;   
}

.path-input {
  flex: 1;
}

.back-button {
  min-width: 40px;
}


.el-scrollbar {
  /* height: 100%; */
  /* height: v-bind(scrollbarHeight);/ */
}

.el-table {
  flex: 1;
  overflow-y: auto;
  margin-top: 10px;
}

.file-count {
  line-height: 20px;
  padding-right: 5px;
  font-size: 13px;
  border-top: 0.1px solid #ebeef5;
  text-align: right;
  color: #909399;
}

</style>