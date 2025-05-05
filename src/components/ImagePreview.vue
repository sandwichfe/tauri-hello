<script setup lang="ts">
import { ref } from 'vue';
import { ElMessage } from 'element-plus';
import Viewer from 'v-viewer';
import 'viewerjs/dist/viewer.css';

const props = defineProps<{
  visible: boolean;
  imageUrl: string;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();

const previewRef = ref();

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
  const viewer = new Viewer(document.querySelector('.image-preview'), {
    url: 'src'
  });
};
</script>

<template>
<div>
  3213123
  <el-dialog
    :model-value="visible"
    @update:model-value="emit('update:visible', $event)"
    title="图片预览"
    width="80%"
    @close="handleClose"
    destroy-on-close
  >
    <img
      class="image-preview"
      :src="imageUrl"
      @error="handleImageError"
      @load="handleImageLoaded"
      data-original="imageUrl"
    />
  </el-dialog>
</div>
</template>

<style scoped>
.image-preview {
  width: 100%;
  max-height: 70vh;
  object-fit: contain;
}
</style>