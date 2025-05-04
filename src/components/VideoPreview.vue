<script setup lang="ts">
import { ref } from 'vue';
import { ElDialog } from 'element-plus';

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
      class="video-player"
      :src="videoUrl"
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