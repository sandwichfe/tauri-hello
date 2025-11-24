<script setup lang="ts">
import { ElImageViewer  } from 'element-plus';
import {  ref } from 'vue';

// @ts-ignore-next-line
defineProps<{
  visible: boolean;
  imageUrls: string[];
  initialIndex: number;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();


const handleClose = () => {
  emit('update:visible', false);
};

const scrollContainer = ref(null);

// 解决当图片预览 超出窗口 会触发父级滚动条的问题
const handleWheel = (e: WheelEvent) => {
  const container = scrollContainer.value;
  if (!container) return; // 确保容器存在

  const { scrollTop, scrollHeight, clientHeight } = container;
  const isScrollingDown = e.deltaY > 0;
  const isScrollingUp = e.deltaY < 0;

  // 检查是否到达边界
  const atTop = scrollTop <= 0 && isScrollingUp;
  const atBottom = scrollTop + clientHeight >= scrollHeight && isScrollingDown;

  // 阻止默认行为（禁止父级滚动）
  if (atTop || atBottom) {
    e.preventDefault();
  }
};



</script>

<template>
  <!-- <el-dialog :model-value="visible" @update:model-value="emit('update:visible', $event)" width="80%" height="80%"
    @close="handleClose" destroy-on-close :show-close="false"> -->

  <!-- <el-image
      :src="imageUrl"
      :preview-src-list="[imageUrl]"
      :preview-teleported="true"
      fit="contain"
      @error="handleImageError"
      @load="handleImageLoaded"
      @close="handleClose"
      show-progress
      hide-on-click-modal
    >
    </el-image> -->
  <!-- </el-dialog> -->

  <div 
    ref="scrollContainer"
    @wheel="handleWheel"
    style="overflow: auto;"
  >
    <el-image-viewer v-if="visible" :url-list="imageUrls" :initial-index="initialIndex" hide-on-click-modal
      @close="handleClose">
    </el-image-viewer>
  </div>


</template>

<style scoped>
.image-preview {
  width: 100%;
  object-fit: contain;
}

</style>
