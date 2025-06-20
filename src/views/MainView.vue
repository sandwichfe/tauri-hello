<script setup lang="ts">
import { ElTabs, ElTabPane } from 'element-plus';
import type { TabsInstance } from 'element-plus'
import ResourceManager from './ResourceManager.vue';
import GitProxyManager from './GitProxyManager.vue';
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

// 系统设置的缩放率
const scaleFactor = ref(1);
const windowsHeight = ref(0);

const tabsHeight = ref(42.8);
// 计算高度（使用 computed 让高度保持响应式）
const tabsContentHeight = computed(() => `${(windowsHeight.value/scaleFactor.value)  - tabsHeight.value}px`);

// 使用 TabsInstance 类型声明 ref
const tabsRef = ref<TabsInstance | null>(null);

// 更新 Tab Content 高度的函数（添加防抖）
const updateTabContentHeight = async () => {
  windowsHeight.value = (await getCurrentWindow().innerSize()).height;
  scaleFactor.value = await getCurrentWindow().scaleFactor();
  console.log('windowsHeight updated:', windowsHeight.value/scaleFactor.value);
  // 确保 tabsRef 已经挂载
  if (tabsRef.value?.$el) {
    const contentEl = tabsRef.value.$el.querySelector('.el-tabs__content');
    if (contentEl) {
      contentEl.style.height = tabsContentHeight.value;
      console.log('tabsContentHeight updated:', tabsContentHeight.value);
    }
  }
};

// 防抖函数
// const debounce = (fn: Function, delay: number) => {
//   let timer: number | null = null;
//   return function(this: any, ...args: any[]) {
//     if (timer) {
//       clearTimeout(timer);
//     }
//     timer = setTimeout(() => {
//       fn.apply(this, args);
//       timer = null;
//     }, delay);
//   };
// };

// 创建防抖版本的高度更新函数
// const debouncedUpdateTabContentHeight = debounce(updateTabContentHeight, 100);


// 组件挂载时设置初始高度并添加 resize 监听
onMounted(async () => {
  // 初始设置
  updateTabContentHeight();
  // 使用Tauri的onResized方法监听窗口大小变化
  getCurrentWindow().onResized(async ({ payload: size }) => {
    updateTabContentHeight();
    windowsHeight.value = size.height;
    console.log('Window resized', size);
  })

});

// 组件卸载时移除监听（重要！避免内存泄漏）
onUnmounted(() => {

});
</script>


<template>
    <div class="container">
        <!-- Tabs组件 -->
        <el-tabs type="card" class="tabs" ref="tabsRef" style="height: 100%;">
            <!-- 代理管理 Tab -->
            <el-tab-pane label="Git代理管理">
                <GitProxyManager />
            </el-tab-pane>

            <!-- 资源管理 Tab -->
            <el-tab-pane label="资源管理">
                <ResourceManager />
            </el-tab-pane>

            <!-- 设置 Tab -->
            <el-tab-pane label="设置">
                <div style="display: flex;  flex-direction: column;">
                    <h2>设置</h2>
                    <p>这里是设置页面的内容。</p>
                </div>
            </el-tab-pane>
        </el-tabs>

    </div>
</template>

<style scoped>

.container {
  height: 100%;
  width: 100%;
  margin: 0 auto;
  padding: 0;
  user-select: none;  /* 禁止文字选中 */
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}

.tabs :deep(.el-tabs__header) {
  height: 42px;
  /* background-color: #9d5555; */
  margin: 0;
}

.tabs :deep(.el-tabs__content) {
  /* background-color: #7441b0; */
  overflow: auto;
  min-height: 200px;  /* 关键修复：防止高度塌陷 */
}


.button-group {
  display: flex;
   flex-direction: column;  /* 改为纵向排列 */
  justify-content: center;
  gap: 20px;
  width: 100%;
}

button {
  border-radius: 8px;
  border: 1px solid #bbb;
  padding: 0.8em 1.4em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #333;
  background-color: #909399; /* 淡蓝色按钮背景 */
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  flex: 1;
  min-width: 160px;
  text-align: center;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

button:hover {
  background-color: #3d4748; /* 悬浮时更深的淡蓝色 */
  color: #fff;
}

button:active {
  background-color: #9ea6a7; /* 点击时的蓝色 */
}

h2 {
  font-size: 1.5rem;
  color: #444;
  margin-bottom: 10px;
}

p {
  font-size: 1.2rem;
  color: #666;
}

.container .el-tab-pane {
      height: 100%!important;
      color: #010101;
  }

</style>