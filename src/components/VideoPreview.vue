<script setup lang="ts">
import { ref } from 'vue';
import { ElMessage,ElDialog } from 'element-plus';

const props = defineProps<{
  visible: boolean;
  videoUrl: string;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();

const handleClose = () => {
  emit('update:visible', false);
};

const handleVideoError = (e: Event) => {
  console.error('视频加载错误详情:', {
    eventType: e.type,
    target: e.target,
    timeStamp: e.timeStamp,
    error: (e.target as HTMLVideoElement)?.error
  });
  ElMessage.error('视频加载失败，请检查文件格式或内容');
  emit('update:visible', false);
};

const handleVideoLoaded = () => {
  console.log('视频已成功加载');
};
</script>

<template>
  <el-dialog
    :model-value="visible"
    @update:model-value="emit('update:visible', $event)"
    title="视频预览"
    width="80%"
    @close="handleClose"
    destroy-on-close
  >
    <video
      controls
      autoplay
      muted
      class="video-player"
      :src="videoUrl"
      @error="handleVideoError"
      @loadeddata="handleVideoLoaded"
    ></video>
  </el-dialog>
</template>

<style scoped>
.video-player {
  width: 100%;
  max-height: 70vh;
  object-fit: contain;
}
</style>