<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage, ElButton } from 'element-plus';
import { EditPen,QuestionFilled } from '@element-plus/icons-vue'
import { openLoading, closeLoading } from "../../src/utils/loadingUtil";
import { readTextFile, writeTextFile, BaseDirectory, mkdir, exists } from '@tauri-apps/plugin-fs'

const proxy_ip = ref("http://127.0.0.1:7897");
const configPath = ref("");
const proxyEnabled = ref(false);
const proxyButtonStatus = ref(false);

onMounted(async () => {
  try {
    configPath.value = 'proxyConfig.json';
    const config = await readTextFile(configPath.value, {
      baseDir: BaseDirectory.AppConfig
    });
    
    const { config: savedConfig } = JSON.parse(config);
    proxy_ip.value = savedConfig?.proxy_ip || "http://127.0.0.1:7897";
    
    // 检查代理是否已启用
    await checkProxyStatus();
  } catch (error) {
    console.log('首次运行或配置文件不存在，使用默认值');
  }
});

async function getGitProxyInfo() {
  openLoading();
  try {
    const result_http: string = await invoke('my_custom_command', { command: "git config --global --get http.proxy" });
    const result_https: string = await invoke('my_custom_command', { command: "git config --global --get https.proxy" });
    console.info("result_http: " + result_http);
    if (!result_http.trim() && !result_https.trim()) {
      ElMessage.success("未设置代理");
    } else {
      ElMessage.success({
        message: result_http + "<br>" + result_https,
        dangerouslyUseHTMLString: true
      });
    }
  } catch (error) {
    console.error(error);
  }
  closeLoading();
}

// 检查代理状态
async function checkProxyStatus() {
  try {
    const result_http: string = await invoke('my_custom_command', { command: "git config --global --get http.proxy" });
    const result_https: string = await invoke('my_custom_command', { command: "git config --global --get https.proxy" });
    proxyEnabled.value = !!(result_http.trim() || result_https.trim());
    proxyButtonStatus.value = proxyEnabled.value;
  } catch (error) {
    console.error("检查代理状态失败:", error);
    proxyEnabled.value = false;
  }
}

async function setGitProxy() {
  const proxyUrl = proxy_ip.value;
  await invoke('my_custom_command', { command: `git config --global http.proxy ${proxyUrl}` });
  await invoke('my_custom_command', { command: `git config --global https.proxy ${proxyUrl}` });
}

async function disableProxy() {
  await invoke('my_custom_command', { command: "git config --global --unset http.proxy" });
  await invoke('my_custom_command', { command: "git config --global --unset https.proxy" });
  ElMessage.success("代理已关闭");
  proxyEnabled.value = false;
}

async function enableProxy() {
  await setGitProxy();
  proxyEnabled.value = true;
  ElMessage.success("代理已开启");
}

async function toggleProxy() {
  openLoading();
  try {
    if (proxyEnabled.value) {
      await disableProxy();
    } else {
      await enableProxy();
    }
  } catch (error) {
    console.error("代理操作失败:", error);
    ElMessage.error("代理操作失败");
    proxyButtonStatus.value = proxyEnabled.value;
  }
  closeLoading();
}

async function openSetProxyDialog() {
  const newProxyUrl = prompt("请输入新的代理地址", proxy_ip.value);
  if (newProxyUrl) {
    proxy_ip.value = newProxyUrl;
    try {
      // 确保目录存在
      const dirExists = await exists('', { baseDir: BaseDirectory.AppConfig });
      if (!dirExists) {
        await mkdir('', {
          recursive: true,
          baseDir: BaseDirectory.AppConfig
        });
      }
      // 写入配置文件
      await writeTextFile(configPath.value, JSON.stringify({ config: { proxy_ip: proxy_ip.value } }), {
        baseDir: BaseDirectory.AppConfig,
        create: true
      });
      ElMessage.success(`代理地址已更新为: ${proxy_ip.value}`);

      // 如果代理正在启用，更新下代理地址
      if (proxyEnabled.value) {
        await setGitProxy();
      }
      
    } catch (error) {
      console.error('保存代理配置失败:', error);
      ElMessage.error(`保存代理配置失败: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
}

defineExpose({
  proxy_ip,
  proxyEnabled,
  toggleProxy,
  openSetProxyDialog
});
</script>

<template>
  <div class="proxy-manager-container">


    <div class="current-proxy">

      <div>git代理地址: {{ proxy_ip }}</div>

      <el-icon @click="getGitProxyInfo" class="view-btn"><QuestionFilled /></el-icon>

      <el-tooltip content="修改代理配置" placement="QuestionFilled">
        <el-button type="warning" @click="openSetProxyDialog" :icon="EditPen" circle class="setting-proxy-btn" />
      </el-tooltip>

      <el-switch v-model="proxyButtonStatus" @change="toggleProxy" class="proxy-switch-btn" />

    </div>

  </div>
</template>

<style scoped>
.proxy-manager-container {
  height: 100%;
}


h2 {
  color: #303133;
  margin-bottom: 16px;
  font-size: 24px;
  font-weight: 600;
}

.current-proxy {
  background-color: #f7f7f7;;
  color: #409eff;
  padding: 12px 16px; /* 增加内边距 */
  border-radius: 8px;
  font-size: 14px;
  display: flex;          /* 新增：启用flex布局 */
  align-items: center;    /* 垂直居中 */
  gap: 12px;              /* 元素间距 */
  position: relative;     /* 为悬浮按钮定位 */
}

.proxy-status {
  margin-top: 0; /* 取消原来的上边距 */
}

/* 新增：设置代理按钮样式 */
.setting-proxy-btn {
  transition: all 0.3s;
}

.proxy-switch-btn {
margin-right: 2px;
margin-left: auto;
}

.view-btn {
  cursor: pointer;
}

.toggle-button {
  height: 32px;
  font-size: 14px;
  font-weight: 500;
  border-radius: 8px;
  transition: all 0.3s;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

/* 移动端适配调整 */
@media (max-width: 600px) {
  .current-proxy {
    flex-wrap: wrap;
  }
  .setting-proxy-btn {
    margin-left: 0;
    margin-top: 8px;
  }
}
</style>