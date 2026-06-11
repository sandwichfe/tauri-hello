<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue';

const props = defineProps<{
  visible: boolean;
  imageUrls: string[];
  imageNames?: string[];
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

const currentImageName = computed(() => {
  if (!hasImages.value) return '';

  const propName = props.imageNames?.[currentIndex.value]?.trim();
  if (propName) return propName;

  const currentUrl = props.imageUrls[currentIndex.value] ?? '';
  const urlWithoutQuery = currentUrl.split(/[?#]/)[0] ?? '';
  const normalizedUrl = safeDecodeURIComponent(urlWithoutQuery);
  const name = normalizedUrl.split(/[\\/]/).pop() ?? '';

  return name || '未知文件';
});

const safeDecodeURIComponent = (value: string) => {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
};

const normalizeIndex = (index: number) => {
  if (!hasImages.value) return 0;
  return Math.min(Math.max(index, 0), props.imageUrls.length - 1);
};

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
  currentIndex.value = normalizeIndex(index);
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
      <div class="image-preview-info">
        <div class="image-preview-name" :title="currentImageName">{{ currentImageName }}</div>
      </div>
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

.image-preview-info {
  position: fixed;
  top: 24px;
  left: 50%;
  z-index: 2001;
  max-width: min(520px, calc(100vw - 96px));
  padding: 6px 12px;
  border-radius: 4px;
  color: #fff;
  background: rgb(0 0 0 / 48%);
  transform: translateX(-50%);
  pointer-events: none;
}

.image-preview-counter {
  position: fixed;
  bottom: 24px;
  left: 24px;
  z-index: 2001;
  padding: 6px 12px;
  border-radius: 4px;
  color: #fff;
  background: rgb(0 0 0 / 48%);
  pointer-events: none;
  font-size: 14px;
  line-height: 1;
}

.image-preview-name {
  overflow: hidden;
  width: 100%;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
  line-height: 1.2;
}
</style>
