<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from 'element-plus';
import { openLoading, closeLoading } from "../../src/utils/loadingUtil";
import { readTextFile, writeTextFile,BaseDirectory,mkdir,exists  } from '@tauri-apps/plugin-fs'

const proxy_ip = ref("http://127.0.0.1:7897");
const configPath = ref("");

onMounted(async () => {
  try {
    configPath.value = 'proxyConfig.json';
    const config = await readTextFile(configPath.value, {
      baseDir: BaseDirectory.AppConfig
    });
    
    const { config: savedConfig } = JSON.parse(config);
    proxy_ip.value = savedConfig?.proxy_ip || "http://127.0.0.1:7897";
  } catch (error) {
    console.log('首次运行或配置文件不存在，使用默认值');
  }
});

async function enableGitProxy() {
  openLoading();
  try {
    const proxyUrl = proxy_ip.value;
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
    } catch (error) {
      console.error('保存代理配置失败:', error);
      ElMessage.error(`保存代理配置失败: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
}

defineExpose({
  proxy_ip,
  enableGitProxy,
  closeGitProxy,
  getGitProxyInfo,
  openSetProxyDialog
});
</script>

<template>
  <div class="button-group">
    <button @click="enableGitProxy" class="action-button">开启本地代理</button>
    <button @click="closeGitProxy" class="action-button">关闭本地代理</button>
    <button @click="getGitProxyInfo" class="action-button">查看代理</button>
    <button @click="openSetProxyDialog" class="action-button">设置代理</button>
  </div>
</template>

<style scoped>
.button-group {
  display: flex;
  flex-direction: column;
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
  background-color: #909399;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  flex: 1;
  min-width: 160px;
  text-align: center;
}

button:hover {
  background-color: #3d4748;
  color: #fff;
}

button:active {
  background-color: #9ea6a7;
}
</style>