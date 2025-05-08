import { createApp } from "vue";
import App from "./App.vue";

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

// 导入预加载功能
import { preloadResources, openLoading, closeLoading } from './utils/loadingUtil';

// 创建Vue应用实例
const app = createApp(App);

// 使用Element Plus
app.use(ElementPlus);

// 在挂载应用前先预加载资源
async function initApp() {
  // 显示加载状态
  openLoading();
  
  try {
    // 等待资源预加载完成
    await preloadResources();
    
    // 预加载完成后挂载应用
    app.mount('#app');
    
    // 短暂延迟后关闭加载状态，确保UI渲染完成
    setTimeout(() => {
      closeLoading();
    }, 300);
  } catch (error) {
    console.error('应用初始化失败:', error);
    // 即使出错也要关闭加载状态并尝试挂载应用
    closeLoading();
    app.mount('#app');
  }
}

// 启动应用
initApp();
