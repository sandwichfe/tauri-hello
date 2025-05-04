<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage, ElTabs, ElTabPane } from 'element-plus';
import ResourceManager from './components/ResourceManager.vue';
// @ts-ignore
import { openLoading, closeLoading } from "../src/utils/loadingUtil";

const proxy_ip = ref("http://127.0.0.1:7897");

// Git代理相关功能
async function enableGitProxy() {
  openLoading();
  try {
    const proxyUrl = proxy_ip.value;  // 获取代理地址的值
    await invoke('my_custom_command', { command: `git config --global http.proxy ${proxyUrl}` });
    await invoke('my_custom_command', { command: `git config --global https.proxy ${proxyUrl}` });
    ElMessage.success("代理已设置");
  } catch (error) {
    console.error("Error configuring proxy:", error);
    ElMessage.error("Failed to configure proxy");
  }
  closeLoading();
}

async function closeGitProxy() {
  openLoading();
  try {
    await invoke('my_custom_command', { command: "git config --global --unset http.proxy" });
    await invoke('my_custom_command', { command: "git config --global --unset https.proxy" });
    ElMessage.success("代理已取消");
  } catch (error) {
    console.error("Error removing proxy:", error);
    ElMessage.error("操作失败");
  }
  closeLoading();
}

async function getGitProxyInfo() {
  openLoading();
  try {
    const result_http: string = await invoke('my_custom_command', { command: "git config --global --get http.proxy" });
    const result_https: string = await invoke('my_custom_command', { command: "git config --global --get https.proxy" });

    if (!result_http.trim() && !result_https.trim()) {
      ElMessage.success("未设置代理");
    } else {
      ElMessage.success({
        message: result_http + "<br>" + result_https,
        dangerouslyUseHTMLString: true
      });
    }
  } catch (error) {
    ElMessage.error("An error occurred while retrieving the proxy configuration.");
    console.error(error);
  }
  closeLoading();
}

// Git代理相关功能
async function openSetProxyDialog() {
  const newProxyUrl = prompt("请输入新的代理地址", proxy_ip.value);  // 提示框
  if (newProxyUrl) {
    proxy_ip.value = newProxyUrl;
    ElMessage.success(`代理地址已更新为: ${proxy_ip.value}`);
  } else {
    // ElMessage.error("请输入有效的代理地址");
  }
}


</script>

<template>
  <div class="container">
    <!-- Tabs组件 -->
    <el-tabs type="card" class="tabs">
      <!-- 代理管理 Tab -->
      <el-tab-pane label="Git代理管理">
        <div class="button-group">
          <button @click="enableGitProxy" class="action-button">开启本地代理</button>
          <button @click="closeGitProxy" class="action-button">关闭本地代理</button>
          <button @click="getGitProxyInfo" class="action-button">查看代理</button>
          <button @click="openSetProxyDialog" class="action-button">设置代理</button>
        </div>
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
/* 保持原样样式 */
</style>


<style scoped>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #333;
  background-color: #f6f6f6;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
  min-height: 60vh;
  gap: 20px;
  position: relative;
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

.el-tabs {
  width: 100%;
  max-width: 800px;
  overflow: hidden;  /* 防止Tab切换时页面往上跑 */
}

.el-tab-pane {
  padding: 20px;
  background-color: #f0f0f0;
  border-radius: 8px;
  height: 480px; /* 设置固定高度 */
  overflow: auto;  /* 防止溢出内容 */
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
