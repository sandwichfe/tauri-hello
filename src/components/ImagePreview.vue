<script setup lang="ts">
import { ElMessage, ElImage } from 'element-plus';


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
      @close="handleClose"
      show-progress
      preview-teleported
      hide-on-click-modal
    >
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