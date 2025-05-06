<script setup lang="ts">
import { ElTabs, ElTabPane } from 'element-plus';
import type { TabsInstance } from 'element-plus'
import ResourceManager from './components/ResourceManager.vue';
import GitProxyManager from './components/GitProxyManager.vue';
import { computed, onMounted, onUnmounted, ref } from 'vue';

// 计算高度（使用 computed 让高度保持响应式）
const tabsContentHeight = computed(() => `${document.documentElement.clientHeight - 65}px`);

// 使用 TabsInstance 类型声明 ref
const tabsRef = ref<TabsInstance | null>(null);

// 更新 Tab Content 高度的函数
const updateTabContentHeight = () => {
  if (tabsRef.value?.$el) {
    const contentEl = tabsRef.value.$el.querySelector('.el-tabs__content');
    if (contentEl) {
      contentEl.style.height = tabsContentHeight.value;
    }
  }
};

// 组件挂载时设置初始高度并添加 resize 监听
onMounted(() => {
  updateTabContentHeight(); // 初始设置
  window.addEventListener('resize', updateTabContentHeight);
});

// 组件卸载时移除监听（重要！避免内存泄漏）
onUnmounted(() => {
  window.removeEventListener('resize', updateTabContentHeight);
});
</script>

<template>
  <div class="container">
    <!-- Tabs组件 -->
    <el-tabs type="card" class="tabs"  ref="tabsRef">
      <!-- 代理管理 Tab -->
      <el-tab-pane label="Git代理管理">
        <GitProxyManager />
      </el-tab-pane>

      <!-- 其他模块 Tab -->
      <el-tab-pane label="其他模块">
        <div>
          <h2>其他功能模块</h2>
          <p>在这里添加其他模块内容。</p>
        </div>
      </el-tab-pane>

      <!-- 资源管理 Tab -->
      <el-tab-pane label="资源管理">
        <ResourceManager />
      </el-tab-pane>

      <!-- 设置 Tab -->
      <el-tab-pane label="设置">
        <div>
          <h2>设置模块</h2>
          <p>在这里可以放置设置相关的功能。</p>
        </div>
      </el-tab-pane>
    </el-tabs>

  </div>
</template>


<style scoped>

.container {
  width: 100%;
  margin: 0 auto;
  padding: 0;
  user-select: none;  /* 禁止文字选中 */
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}

.tabs >>> .el-tabs__header {
  height: 42px;
  /* background-color: #9d5555; */
  margin: 0;
}

.tabs >>> .el-tabs__content {
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

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  button {
    color: #ffffff;
    background-color: #0f0f0f;
    border-color: #444;
  }

  button:hover {
    background-color: #4dd0e1;
  }

  button:active {
    background-color: #26c6da;
  }

  .el-tabs {
    background-color: #333;
  }

  .el-tab-pane {
    background-color: #444;
    color: #fff;
  }
}
</style>
