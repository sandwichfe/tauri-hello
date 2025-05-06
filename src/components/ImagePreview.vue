<script setup lang="ts">
import { ElMessage, ElImage,ElImageViewer  } from 'element-plus';


const props = defineProps<{
  visible: boolean;
  imageUrls: string[];
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();


const handleClose = () => {
  emit('update:visible', false);
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


    <el-image-viewer v-if="visible" 
    :url-list="imageUrls" 
    :initial-index= 0     
    hide-on-click-modal
    @close="handleClose"  
    destroy-on-close
    :show-close="false"
    class="image-preview"
    >
    </el-image-viewer>


</template>

<style scoped>
.image-preview {
  width: 100%;
  max-height: 70vh;
  object-fit: contain;
}

/* 大图预览的关闭图标 改为大一点的白色 */
.el-image-viewer__btn.el-image-viewer__close {
    color: #FFF;
    opacity: unset;
    top: 50px;
    right: 50px;
    width: 50px;
    height: 50px;
    font-size: 50px;
}

</style>