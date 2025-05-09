import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

export const useWindowStore = defineStore('window', () => {
  // 系统设置的缩放率
  const scaleFactor = ref(1)
  const windowsHeight = ref(0)
  
  // 计算高度（使用 computed 让高度保持响应式）
  const tabsContentHeight = computed(() => (windowsHeight.value/scaleFactor.value) - 65)

  // 更新窗口高度和缩放因子的函数
  const updateWindowSize = async () => {
    windowsHeight.value = (await getCurrentWindow().innerSize()).height
    scaleFactor.value = await getCurrentWindow().scaleFactor()
  }

  return {
    scaleFactor,
    windowsHeight,
    tabsContentHeight,
    updateWindowSize
  }
})