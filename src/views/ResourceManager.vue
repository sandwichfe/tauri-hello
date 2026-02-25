<script setup lang="ts">
// 依赖与组件导入
import {computed, nextTick, onMounted, onUnmounted, ref} from 'vue';
import {ElMessage, ElMessageBox} from 'element-plus';
import {ArrowLeft, Document, Folder, Headset, MoreFilled, Picture, VideoCamera} from '@element-plus/icons-vue';
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

const searchKeyword = ref('');

// 预览相关状态
const showVideoPreview = ref(false);
const currentVideoUrl = ref('');
const showImagePreview = ref(false);
const currentImageUrl = ref<string[]>([]);
const currentIndex = ref(0);

const showFileDetail = ref(false);
const detailFile = ref<FileItem | null>(null);

const actionMenuVisible = ref(false);
const actionMenuPosition = ref({x: 0, y: 0});
const actionRow = ref<FileItem | null>(null);

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

// 获取表格滚动元素
const getScrollableEl = () => {
  return scrollRef.value;
}

// 获取当前滚动位置
const getCurrentScrollTop = (): number => {
  return getScrollableEl()?.scrollTop ?? 0;
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
  // 使用 nextTick 确保在 DOM 更新后执行滚动恢复
  nextTick(() => {
    const el = getScrollableEl();
    if (el) {
      el.scrollTop = pos;
    }
  });
};

// 滚动到顶部（用于前进导航）
const scrollToTop = () => {
  const el = getScrollableEl();
  if (el) {
    el.scrollTop = 0;
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
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const exponent = Math.min(Math.floor(Math.log(size) / Math.log(1024)), units.length - 1);
  const formattedSize = (size / Math.pow(1024, exponent)).toFixed(2);
  return `${formattedSize} ${units[exponent]}`;
};



const filteredFileList = computed(() => {
  const keyword = searchKeyword.value.trim().toLowerCase();
  if (!keyword) {
    return fileList.value;
  }
  return fileList.value.filter((item) =>
    item.name.toLowerCase().includes(keyword)
  );
});

// 选择并打开文件夹
const selectFolder = async () => {
  try {
    const selected = await open({directory: true, multiple: false});
    if (selected) {
      // 选择新文件夹前，先清除之前的操作数据
      clearOperateData();
      await openFolder(selected as string, false);
      await savePathConfig();
    }
  } catch (error) {
    ElMessage.error('选择文件夹失败');
    console.error(error);
  }
};

// 清除操作数据（包括路径历史和滚动位置）
const clearOperateData = () => {
  pathHistory.value = [];
  scrollPositions.value.clear();
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

    searchKeyword.value = '';

    if (sortColumn.value && sortOrder.value) {
      applySorting(sortColumn.value, sortOrder.value);
    }

    // 等待dom更新后 再滚动到顶部
    await nextTick();

    // 如果没有记录滚动位置，说明是第一次打开文件夹，滚动到顶部
    if (scrollPositions.value.size == 0) {
      scrollToTop();
    }
    // 新打开文件夹 滚动到顶部
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

const openActionMenu = (event: MouseEvent, row: FileItem) => {
  const {clientX, clientY} = event;
  actionRow.value = row;
  actionMenuPosition.value = {x: clientX, y: clientY};
  actionMenuVisible.value = true;
};

const closeActionMenu = () => {
  actionMenuVisible.value = false;
  actionRow.value = null;
};

const handleActionDetail = () => {
  if (!actionRow.value) {
    return;
  }
  handleViewDetail(actionRow.value);
  closeActionMenu();
};

const handleActionDelete = () => {
  if (!actionRow.value) {
    return;
  }
  handleMoveToRecycleBin(actionRow.value);
  closeActionMenu();
};

const handleViewDetail = (row: FileItem) => {
  detailFile.value = row;
  showFileDetail.value = true;
};

const handleMoveToRecycleBin = async (row: FileItem) => {
  try {
    await ElMessageBox.confirm(
      `确定将“${row.name}”移动到回收站吗？`,
      '提示',
      {
        type: 'warning',
        confirmButtonText: '确定',
        cancelButtonText: '取消'
      }
    );
  } catch {
    return;
  }

  try {
    await invoke('move_to_recycle_bin', {path: row.path});
    ElMessage.success('已移动到回收站');
    if (currentPath.value) {
      await openFolder(currentPath.value, false);
    }
  } catch (error) {
    ElMessage.error('移动到回收站失败');
    console.error(error);
  }
};

const handleCopyPath = async () => {
  if (!detailFile.value) {
    return;
  }
  try {
    await navigator.clipboard.writeText(detailFile.value.path);
    ElMessage.success('路径已复制到剪贴板');
  } catch (error) {
    console.error(error);
    ElMessage.error('复制路径失败');
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
const handleSortChange = ({prop}: any) => {
  if (sortColumn.value === prop) {
    sortOrder.value = sortOrder.value === 'ascending' ? 'descending' : 'ascending';
  } else {
    sortColumn.value = prop;
    sortOrder.value = 'ascending';
  }
  applySorting(sortColumn.value, sortOrder.value);
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
      <el-input
        v-model="searchKeyword"
        placeholder="搜索当前目录"
        clearable
        class="search-input"
      />
      <el-button type="primary" @click="openClassifierWindow" class="classifier-button" v-if="false">
        打开分类窗口
      </el-button>
    </div>

    <div class="custom-table-container" :style="{ height: scrollbarHeight }" ref="scrollRef">
      <div class="custom-table-header">
        <div class="header-cell type-cell sortable" @click="handleSortChange({ prop: 'is_dir' })">
          类型
          <span :class="['sort-caret', sortOrder]" :style="{ visibility: sortColumn === 'is_dir' ? 'visible' : 'hidden' }"></span>
        </div>
        <div class="header-cell name-cell sortable" @click="handleSortChange({ prop: 'name' })">
          名称
          <span :class="['sort-caret', sortOrder]" :style="{ visibility: sortColumn === 'name' ? 'visible' : 'hidden' }"></span>
        </div>
        <div class="header-cell size-cell sortable" @click="handleSortChange({ prop: 'size' })">
          大小
          <span :class="['sort-caret', sortOrder]" :style="{ visibility: sortColumn === 'size' ? 'visible' : 'hidden' }"></span>
        </div>
        <div class="header-cell modified-cell sortable" @click="handleSortChange({ prop: 'modified_time' })">
          修改时间
          <span :class="['sort-caret', sortOrder]" :style="{ visibility: sortColumn === 'modified_time' ? 'visible' : 'hidden' }"></span>
        </div>
        <div class="header-cell action-cell">
          操作
        </div>
      </div>
      <div class="custom-table-body">
        <div v-for="row in filteredFileList" :key="row.path" class="custom-table-row" @dblclick="handleRowClick(row)" draggable="true" @dragstart="handleDragStart(row, $event)">
          <div class="body-cell type-cell">
            <el-icon>
              <component :is="getFileIcon(row)"/>
            </el-icon>
          </div>
          <div class="body-cell name-cell">{{ row.name }}</div>
          <div class="body-cell size-cell">{{ row.is_dir ? '-' : formatFileSize(row.size) }}</div>
          <div class="body-cell modified-cell">{{ row.modified_time }}</div>
          <div class="body-cell action-cell">
            <span class="more-button" @click.stop="openActionMenu($event, row)">
              <el-icon class="more-icon">
                <more-filled/>
              </el-icon>
            </span>
          </div>
        </div>
      </div>
    </div>

    <div class="file-count">
      共 {{ filteredFileList.length }} 个文件
    </div>

    <VideoPreview v-model:visible="showVideoPreview" :video-url="currentVideoUrl"/>
    <ImagePreview v-model:visible="showImagePreview" :image-urls="currentImageUrl" :initial-index="currentIndex"/>
    <div
      v-if="actionMenuVisible"
      class="action-menu-overlay"
      @click="closeActionMenu"
    >
      <div
        class="action-menu"
        :style="{ top: actionMenuPosition.y + 'px', left: actionMenuPosition.x + 'px' }"
        @click.stop
      >
        <div class="action-menu-item" @click="handleActionDetail">
          详情
        </div>
        <div class="action-menu-item" @click="handleActionDelete">
          删除
        </div>
      </div>
    </div>
    <el-dialog
      v-model="showFileDetail"
      title="文件详情"
      width="480px"
    >
      <div v-if="detailFile">
        <p>名称：{{ detailFile.name }}</p>
        <p>类型：{{ detailFile.is_dir ? '文件夹' : '文件' }}</p>
        <p>大小：{{ detailFile.is_dir ? '-' : formatFileSize(detailFile.size) }}</p>
        <p>修改时间：{{ detailFile.modified_time }}</p>
        <div class="detail-path-row">
          <span class="detail-path-text">路径：{{ detailFile.path }}</span>
          <el-button
            link
            type="primary"
            size="small"
            @click="handleCopyPath"
          >复制
          </el-button>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<style scoped>
.custom-table-container {
  overflow-y: auto;
  border: 1px solid #ebeef5;
  border-radius: 4px;
  margin-top: 10px;
}

.custom-table-header, .custom-table-row {
  display: flex;
  align-items: center;
}

.custom-table-row:hover {
  background-color: #f5f7fa;
}

.custom-table-row:last-child {
  border-bottom: none;
}

.custom-table-header {
  position: sticky;
  top: 0;
  background-color: #fff;
  z-index: 1;
  border-bottom: 1px solid #ebeef5;
}

.header-cell, .body-cell {
  padding: 12px 10px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 14px;
}

.body-cell {
  color: #303133;
  text-align: left;
}

.header-cell {
  font-weight: normal;
  color: #606266;
  position: relative;
  display: inline-flex;
  align-items: center;
}

.type-cell {
  width: 60px;
}

.name-cell {
  flex: 1;
}

.size-cell {
  width: 100px;
}

.modified-cell {
  width: 160px;
}

.action-cell {
  width: 120px;
  text-align: right;
}

.more-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 14px;
  cursor: pointer;
  transition: background-color 0.15s ease, transform 0.1s ease;
}

.more-button:hover {
  background-color: rgba(0, 0, 0, 0.04);
}

.more-button:active {
  background-color: rgba(0, 0, 0, 0.08);
  transform: scale(0.95);
}

.more-icon {
  font-size: 18px;
  color: #909399;
}

.dropdown-clickable {
  display: inline-flex;
  align-items: center;
  justify-content: flex-start;
  width: 100%;
  transition: transform 0.1s ease, color 0.1s ease;
}

.dropdown-clickable:active {
  transform: scale(0.98);
  color: #409EFF;
}

.action-menu-overlay {
  position: fixed;
  inset: 0;
  z-index: 2000;
}

.action-menu {
  position: fixed;
  min-width: 120px;
  background-color: #fff;
  border: 1px solid #ebeef5;
  border-radius: 4px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}

.action-menu-item {
  padding: 8px 16px;
  cursor: pointer;
}

.action-menu-item:hover {
  background-color: #f5f7fa;
}

.sortable {
  cursor: pointer;
}

.sort-caret {
  margin-left: 4px;
  display: inline-flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 12px;
  height: 14px;
}

.sort-caret::before,
.sort-caret::after {
  content: '';
  width: 0;
  height: 0;
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
}

.sort-caret::before {
  border-bottom: 4px solid #c0c4cc;
  margin-bottom: 2px;
}

.sort-caret::after {
  border-top: 4px solid #c0c4cc;
}

.sort-caret.ascending::before {
  border-bottom-color: #409eff;
}

.sort-caret.ascending::after {
  border-top-color: #c0c4cc;
}

.sort-caret.descending::before {
  border-bottom-color: #c0c4cc;
}

.sort-caret.descending::after {
  border-top-color: #409eff;
}

.resource-manager .el-icon {
  color: #a08dba;
  font-size: 22px;
  vertical-align: middle;
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
  flex-shrink: 0;
}

.path-input {
  flex: 1;
}

.search-input {
  width: 260px;
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

.file-count {
  line-height: 20px;
  padding-right: 5px;
  font-size: 13px;
  border-top: 0.1px solid #ebeef5;
  text-align: right;
  color: #909399;
  background-color: #fcfcfc;
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

.detail-path-row {
  margin-top: 4px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.detail-path-text {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
