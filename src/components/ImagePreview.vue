<script setup lang="ts">
import { ElMessage, ElImage } from 'element-plus';
import { ElIcon } from 'element-plus';
import {
  Back,
  DArrowRight,
  Download,
  Refresh,
  RefreshLeft,
  RefreshRight,
  Right,
  ZoomIn,
  ZoomOut,
} from '@element-plus/icons-vue';

const props = defineProps<{
  visible: boolean;
  imageUrl: string;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();


const handleClose = () => {
  emit('update:visible', false);
};

const handleImageError = (e: Event) => {
  console.error('图片加载错误详情:', {
    eventType: e.type,
    target: e.target,
    timeStamp: e.timeStamp
  });
  ElMessage.error('图片加载失败，请检查文件格式或内容');
};

const handleImageLoaded = () => {
  console.log('图片已成功加载');
};

const download = (index: number) => {
  const url = props.imageUrl;
  const suffix = url.slice(url.lastIndexOf('.'));
  const filename = Date.now() + suffix;

  fetch(url)
    .then((response) => response.blob())
    .then((blob) => {
      const blobUrl = URL.createObjectURL(new Blob([blob]));
      const link = document.createElement('a');
      link.href = blobUrl;
      link.download = filename;
      document.body.appendChild(link);
      link.click();
      URL.revokeObjectURL(blobUrl);
      link.remove();
    });
};
</script>

<template>
  <el-dialog
    :model-value="visible"
    @update:model-value="emit('update:visible', $event)"
    width="80%"
    height="80%"
    @close="handleClose"
    destroy-on-close
    :show-close="false"
  >
    <el-image
      :src="imageUrl"
      :preview-src-list="[imageUrl]"
      fit="contain"
      @error="handleImageError"
      @load="handleImageLoaded"
      show-progress
      preview-teleported
    >
      <template
        #toolbar="{ actions, prev, next, reset, activeIndex, setActiveItem }"
      >
        <el-icon @click="prev"><Back /></el-icon>
        <el-icon @click="next"><Right /></el-icon>
        <el-icon @click="setActiveItem(0)">
          <DArrowRight />
        </el-icon>
        <el-icon @click="actions('zoomOut')"><ZoomOut /></el-icon>
        <el-icon
          @click="actions('zoomIn', { enableTransition: false, zoomRate: 2 })"
        >
          <ZoomIn />
        </el-icon>
        <el-icon
          @click="
            actions('clockwise', { rotateDeg: 180, enableTransition: false })"
        >
          <RefreshRight />
        </el-icon>
        <el-icon @click="actions('anticlockwise')"><RefreshLeft /></el-icon>
        <el-icon @click="reset"><Refresh /></el-icon>
        <el-icon @click="download(0)"><Download /></el-icon>
      </template>
    </el-image>
  </el-dialog>
</template>

<style scoped>
.image-preview {
  width: 100%;
  max-height: 70vh;
  object-fit: contain;
}
</style>