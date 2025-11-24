<script setup lang="ts">
// 依赖与组件导入
import {computed, nextTick, onMounted, onUnmounted, ref} from 'vue';
import {ElMessage} from 'element-plus';
import {ArrowLeft, Document, Folder, Headset, Picture, VideoCamera} from '@element-plus/icons-vue';
import {convertFileSrc, invoke} from '@tauri-apps/api/core';
import {open} from '@tauri-apps/plugin-dialog';
import {BaseDirectory, exists, mkdir, readTextFile, writeTextFile} from '@tauri-apps/plugin-fs';
import VideoPreview from '../components/VideoPreview.vue';
import ImagePreview from '../components/ImagePreview.vue';
import {closeLoading, openLoading} from '../utils/loadingUtil';
import {getCurrentWindow, Window} from '@tauri-apps/api/window';
import {Webview} from '@tauri-apps/api/webview';

// 文件项结构定义
interface FileItem {
  is_dir: boolean;
  name: string;
  size: number;
  modified_time: string;
  path: string;
}

// 媒体文件扩展名集合
const videoExtensions = ['.mp4', '.webm', '.ogg', '.mov', '.avi'];
const audioExtensions = ['.mp3', '.wav', '.aac', '.flac', '.m4a'];
const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp'];

// 业务状态
const currentPath = ref('');
const fileList = ref<FileItem[]>([]);
const imageFiles = ref<Map<string, number>>(new Map());
const pathHistory = ref<string[]>([]);
const configPath = ref('pathConfig.json');

// 排序相关状态
const sortColumn = ref('');
const sortOrder = ref('');

// 预览相关状态
const showVideoPreview = ref(false);
const currentVideoUrl = ref('');
const showImagePreview = ref(false);
const currentImageUrl = ref<string[]>([]);
const currentIndex = ref(0);

// 导航栏与工具栏高度（用于高度计算）
const tabsHeight = ref(42.8);
const toolbarHeight = ref(32);

// 窗口尺寸与滚动区域高度计算
const scaleFactor = ref(1);
const windowsHeight = ref(0);
const scrollbarHeight = computed(() => {
  const height = (windowsHeight.value / scaleFactor.value) - tabsHeight.value - toolbarHeight.value - 20 - 0.1;
  return `${Math.max(0, height)}px`;
});

// 滚动组件引用与滚动位置存储
const scrollRef = ref<any | null>(null);
const scrollPositions = ref<Map<string, number>>(new Map());

// 获取当前滚动位置
const getCurrentScrollTop = (): number => {
  const inst = scrollRef.value as any;
  const wrap = inst?.wrapRef || inst?.$el?.querySelector?.('.el-scrollbar__wrap');
  return wrap ? wrap.scrollTop : 0;
};

// 记住当前路径的滚动位置
const rememberCurrentScroll = () => {
  if (currentPath.value) {
    scrollPositions.value.set(currentPath.value, getCurrentScrollTop());
  }
};

// 恢复指定路径的滚动位置
const restoreScrollForPath = (path: string) => {
  const pos = scrollPositions.value.get(path) ?? 0;
  const inst = scrollRef.value as any;
  if (!inst) return;
  if (typeof inst.setScrollTop === 'function') {
    inst.setScrollTop(pos);
    return;
  }
  const wrap = inst?.wrapRef || inst?.$el?.querySelector?.('.el-scrollbar__wrap');
  if (wrap) {
    wrap.scrollTop = pos;
  }
};

// 滚动到顶部（用于前进导航）
const scrollToTop = () => {
  const inst = scrollRef.value as any;
  if (!inst) return;
  if (typeof inst.setScrollTop === 'function') {
    inst.setScrollTop(0);
    return;
  }
  const wrap = inst?.wrapRef || inst?.$el?.querySelector?.('.el-scrollbar__wrap');
  if (wrap) wrap.scrollTop = 0;
};

// 滚动事件，持续记录当前位置
const onScroll = (e: any) => {
  const scrollTop = typeof e === 'object' && e ? (e.scrollTop ?? getCurrentScrollTop()) : getCurrentScrollTop();
  if (currentPath.value) {
    scrollPositions.value.set(currentPath.value, scrollTop);
  }
};


// 计算滚动区域高度（读取窗口逻辑尺寸与缩放）
const calculateScrollbarHeight = async () => {
  const size = await getCurrentWindow().innerSize();
  windowsHeight.value = size.height;
  scaleFactor.value = await getCurrentWindow().scaleFactor();
};

// 监听窗口大小变化并更新高度
const handleResize = () => {
  calculateScrollbarHeight();
};

// 检查是否可以返回上一级
const canGoBack = () => pathHistory.value.length > 0;

// 文件类型图标映射
const getFileIcon = (row: any) => {
  if (row.is_dir) return Folder;
  if (isVideoFile(row.name)) return VideoCamera;
  if (isAudioFile(row.name)) return Headset;
  if (isImageFile(row.name)) return Picture;
  return Document;
};

// 判断各类型媒体文件
const isVideoFile = (filename: string) => videoExtensions.some(ext => filename.toLowerCase().endsWith(ext));
const isAudioFile = (filename: string) => audioExtensions.some(ext => filename.toLowerCase().endsWith(ext));
const isImageFile = (filename: string) => imageExtensions.some(ext => filename.toLowerCase().endsWith(ext));

// 将本地文件路径转换为可预览的资源 URL
const getAssetUrl = async (path: string) => {
  try {
    return convertFileSrc(path);
  } catch (e) {
    throw e;
  }
};

// 预览视频文件
const previewVideo = async (path: string) => {
  try {
    openLoading();
    currentVideoUrl.value = await getAssetUrl(path);
    showVideoPreview.value = true;
    closeLoading();
  } catch (error) {
    ElMessage.error('视频加载失败');
    console.error(error);
  }
};

// 预览图片文件
const previewImage = async (path: string) => {
  try {
    openLoading();
    currentImageUrl.value = await Promise.all(
        Array.from(imageFiles.value).map(async ([p]) => await getAssetUrl(p))
    );
    currentIndex.value = imageFiles.value.get(path) || 0;
    showImagePreview.value = true;
    closeLoading();
  } catch (error) {
    ElMessage.error('图片加载失败');
    console.error(error);
  }
};

// 文件大小格式化显示
const formatFileSize = (size: number) => {
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)} KB`;
  if (size < 1024 * 1024 * 1024) return `${(size / (1024 * 1024)).toFixed(2)} MB`;
  return `${(size / (1024 * 1024 * 1024)).toFixed(2)} GB`;
};

// 选择并打开文件夹
const selectFolder = async () => {
  try {
    const selected = await open({directory: true, multiple: false});
    if (selected) {
      pathHistory.value = [];
      await openFolder(selected as string, false);
      await savePathConfig();
    }
  } catch (error) {
    ElMessage.error('选择文件夹失败');
    console.error(error);
  }
};

// 从后端读取目录文件
const readDirectory = async (path: string) => {
  return await invoke<FileItem[]>('read_directory', {path});
};

// 更新图片文件映射（用于图片预览列表）
const updateImageFilesMap = (files: FileItem[]) => {
  imageFiles.value = new Map(
      files
      .filter(file => !file.is_dir && isImageFile(file.name))
      .map((file, index) => [file.path, index])
  );
};

// 打开指定文件夹并维护历史
const openFolder = async (nextPath: string, pushHistory = true) => {
  try {
    if (pushHistory && currentPath.value && currentPath.value !== nextPath) {
      rememberCurrentScroll();
      pathHistory.value.push(currentPath.value);
    }

    const files = await readDirectory(nextPath);
    fileList.value = files;
    updateImageFilesMap(files);
    currentPath.value = nextPath;

    if (sortColumn.value && sortOrder.value) {
      applySorting(sortColumn.value, sortOrder.value);
    }

    // 等待dom更新后 再滚动到顶部
    await nextTick();
    if (pushHistory) {
      scrollToTop();
    }
  } catch (error) {
    ElMessage.error('读取文件夹失败');
    console.error(error);
  }
};

// 返回上一级目录（基于栈的历史）
const goBack = async () => {
  if (!canGoBack()) return;
  rememberCurrentScroll();
  const previous = pathHistory.value.pop();
  if (previous) {
    await openFolder(previous, false);
    await nextTick();
    restoreScrollForPath(previous);
  }
};

// 行双击事件：打开文件夹或预览媒体
const handleRowClick = (row: FileItem) => {
  if (row.is_dir) {
    openFolder(row.path, true);
    return;
  }
  if (isVideoFile(row.name) || isAudioFile(row.name)) {
    previewVideo(row.path);
    return;
  }
  if (isImageFile(row.name)) {
    previewImage(row.path);
  }
};

// 开始拖拽：设置拖拽数据（JSON）与拖拽效果
const handleDragStart = (row: FileItem, event: DragEvent) => {
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/json', JSON.stringify(row));
    event.dataTransfer.effectAllowed = 'copy';
  }
};

// 打开文件分类窗口并绑定拖拽事件
const openClassifierWindow = async () => {
  try {
    const appWindow = new Window('file-classifier');
    const webview = new Webview(appWindow, 'file-classifier-webview', {
      url: '/#/file-classifier',
      width: 800,
      height: 600,
      x: 100,
      y: 100,
      dragDropEnabled: true,
      acceptFirstMouse: true,
    });

    webview.onDragDropEvent((event) => {
      if (event.payload.type === 'drop' || event.payload.type === 'enter') {
        const paths = event.payload.paths;
        if (paths && paths.length > 0) {
          const path = paths[0];
          const name = path.split(/[\\/]/).pop() || path;
          const fileData = {is_dir: false, name, size: 0, modified_time: '', path};
          webview.emit('file-dropped', fileData);
        }
      }
    });

    webview.once('tauri://created', () => {
    });
    webview.once('tauri://error', (e) => {
      console.error('文件分类窗口创建失败:', e);
      ElMessage.error('打开文件分类窗口失败');
    });
  } catch (error) {
    console.error('打开文件分类窗口失败:', error);
    ElMessage.error('打开文件分类窗口失败');
  }
};

// 处理排序变化并应用排序
const handleSortChange = ({prop, order}: any) => {
  sortColumn.value = prop;
  sortOrder.value = order;
  if (!order) return;
  applySorting(prop, order);
};

// 保存路径配置到 AppConfig 目录
const savePathConfig = async () => {
  try {
    const dirExists = await exists('', {baseDir: BaseDirectory.AppConfig});
    if (!dirExists) {
      await mkdir('', {recursive: true, baseDir: BaseDirectory.AppConfig});
    }
    await writeTextFile(
        configPath.value,
        JSON.stringify({currentPath: currentPath.value}),
        {baseDir: BaseDirectory.AppConfig, create: true}
    );
  } catch (error) {
    console.error('保存路径配置失败:', error);
  }
};

// 载入路径配置并返回已保存路径
const loadPathConfig = async (): Promise<string | null> => {
  try {
    const config = await readTextFile(configPath.value, {baseDir: BaseDirectory.AppConfig});
    const {currentPath: savedPath} = JSON.parse(config);
    return savedPath || null;
  } catch {
    return null;
  }
};

// 组件挂载：加载配置与尺寸计算
onMounted(async () => {
  const saved = await loadPathConfig();
  if (saved) {
    await openFolder(saved, false);
  }
  await calculateScrollbarHeight();
  window.addEventListener('resize', handleResize);
});

// 组件卸载：移除事件监听
onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
});

// 应用排序：文件夹优先，其次按列与排序方向
const applySorting = (prop: string, order: string) => {
  const sortedList = [...fileList.value];
  sortedList.sort((a, b) => {
    if (prop === 'is_dir') {
      return order === 'ascending' ? Number(a.is_dir) - Number(b.is_dir) : Number(b.is_dir) - Number(a.is_dir);
    }
    if (a.is_dir !== b.is_dir) return a.is_dir ? -1 : 1;
    if (prop === 'name') {
      return order === 'ascending' ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name);
    }
    if (prop === 'size') {
      const sizeA = a.is_dir ? -1 : a.size;
      const sizeB = b.is_dir ? -1 : b.size;
      return order === 'ascending' ? sizeA - sizeB : sizeB - sizeA;
    }
    if (prop === 'modified_time') {
      return order === 'ascending' ? a.modified_time.localeCompare(b.modified_time) : b.modified_time.localeCompare(a.modified_time);
    }
    return 0;
  });
  fileList.value = sortedList;
};

</script>

<template>
  <div class="resource-manager">
    <div class="toolbar">
      <el-button @click="goBack" :disabled="!canGoBack()" type="info" class="back-button">
        <el-icon>
          <arrow-left/>
        </el-icon>
      </el-button>
      <el-input v-model="currentPath" placeholder="当前路径" readonly class="path-input">
        <template #append>
          <el-button @click="selectFolder">选择文件夹</el-button>
        </template>
      </el-input>
      <el-button type="primary" @click="openClassifierWindow" class="classifier-button">
        打开分类窗口
      </el-button>
    </div>

    <el-scrollbar :native="true" :style="{ height: scrollbarHeight }" ref="scrollRef" @scroll="onScroll">
      <el-table :data="fileList" style="width: 100%" @row-dblclick="handleRowClick" @sort-change="handleSortChange"
                row-key="path">
        <el-table-column label="类型" width="80" prop="is_dir" sortable="custom">
          <template #default="{ row }">
            <el-icon>
              <component :is="getFileIcon(row)"/>
            </el-icon>
          </template>
        </el-table-column>
        <el-table-column prop="name" label="名称" sortable="custom">
          <template #default="{ row }">
            <div class="draggable-cell" draggable="true" @dragstart="handleDragStart(row, $event)">
              {{ row.name }}
            </div>
          </template>
        </el-table-column>
        <el-table-column label="大小" width="120" prop="size" sortable="custom">
          <template #default="{ row }">
            <span>{{ row.is_dir ? '-' : formatFileSize(row.size) }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="modified_time" label="修改时间" width="180" sortable="custom"/>
      </el-table>
    </el-scrollbar>
    <div class="file-count">
      共 {{ fileList.length }} 个文件
    </div>

    <VideoPreview v-model:visible="showVideoPreview" :video-url="currentVideoUrl"/>
    <ImagePreview v-model:visible="showImagePreview" :image-urls="currentImageUrl" :initial-index="currentIndex"/>
  </div>
</template>

<style scoped>
.resource-manager .el-icon {
  color: #a08dba;
  font-size: 22px;
}

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
  background-color: #fff;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
}

.back-button:hover {
  background-color: #f5f7fa;
  border: 1px solid #dcdfe6;
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

.classifier-button {
  margin-left: 10px;
}

.draggable-cell {
  padding: 5px 0;
  width: 100%;
}

.draggable-cell:hover {
  background-color: #f5f7fa;
  border-radius: 4px;
}
</style>
