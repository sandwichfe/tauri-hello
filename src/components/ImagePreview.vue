<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue';

const props = defineProps<{
  visible: boolean;
  imageUrls: string[];
  initialIndex: number;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();

const currentIndex = ref(props.initialIndex);
let previousBodyOverflow = '';
let previousHtmlOverflow = '';
let isScrollLocked = false;

const hasImages = computed(() => props.imageUrls.length > 0);

const safeInitialIndex = computed(() => {
  if (!hasImages.value) return 0;
  return Math.min(Math.max(props.initialIndex, 0), props.imageUrls.length - 1);
});

const counterText = computed(() => {
  if (!hasImages.value) return '';
  return `${currentIndex.value + 1} / ${props.imageUrls.length}`;
});

const lockPageScroll = () => {
  if (isScrollLocked) return;
  previousBodyOverflow = document.body.style.overflow;
  previousHtmlOverflow = document.documentElement.style.overflow;
  document.body.style.overflow = 'hidden';
  document.documentElement.style.overflow = 'hidden';
  isScrollLocked = true;
};

const unlockPageScroll = () => {
  if (!isScrollLocked) return;
  document.body.style.overflow = previousBodyOverflow;
  document.documentElement.style.overflow = previousHtmlOverflow;
  isScrollLocked = false;
};

const handleClose = () => {
  emit('update:visible', false);
};

const handleSwitch = (index: number) => {
  currentIndex.value = index;
};

watch(
  () => props.visible && hasImages.value,
  (shouldShowViewer) => {
    if (shouldShowViewer) {
      currentIndex.value = safeInitialIndex.value;
      lockPageScroll();
      return;
    }

    unlockPageScroll();
  },
  { immediate: true }
);

watch(safeInitialIndex, (index) => {
  if (props.visible) {
    currentIndex.value = index;
  }
});

onBeforeUnmount(() => {
  unlockPageScroll();
});
</script>

<template>
  <div v-if="visible && hasImages" class="image-preview-viewer">
    <el-image-viewer
      :url-list="imageUrls"
      :initial-index="safeInitialIndex"
      hide-on-click-modal
      @close="handleClose"
      @switch="handleSwitch"
    >
      <div class="image-preview-counter">{{ counterText }}</div>
    </el-image-viewer>
  </div>
</template>

<style scoped>
.image-preview-viewer {
  position: fixed;
  inset: 0;
  z-index: 2000;
}

.image-preview-counter {
  position: fixed;
  top: 24px;
  left: 50%;
  z-index: 2001;
  padding: 6px 12px;
  border-radius: 4px;
  color: #fff;
  font-size: 14px;
  line-height: 1;
  background: rgb(0 0 0 / 48%);
  transform: translateX(-50%);
  pointer-events: none;
}
</style>
