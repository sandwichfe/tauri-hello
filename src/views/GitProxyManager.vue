<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";
import { EditPen, QuestionFilled } from "@element-plus/icons-vue";
import { openLoading, closeLoading } from "../../src/utils/loadingUtil";
import { readTextFile, writeTextFile, BaseDirectory, mkdir, exists } from "@tauri-apps/plugin-fs";

const DEFAULT_PROXY_URL = "http://127.0.0.1:7897";
const CONFIG_PATH = "proxyConfig.json";

const labels = {
  useDefaultProxyConfig: "首次运行或配置文件不存在，使用默认代理地址。",
  unsetProxy: "未设置代理",
  unset: "未设置",
  getProxyInfoFailed: "获取 Git 代理信息失败",
  checkProxyStatusFailed: "检查代理状态失败",
  proxyClosed: "代理已关闭",
  proxyOpened: "代理已开启",
  proxyOperationFailed: "代理操作失败",
  proxyAddressUpdated: "代理地址已更新为",
  saveProxyConfigFailed: "保存代理配置失败",
  gitProxyAddress: "Git 代理地址",
  viewCurrentGitProxy: "查看当前 Git 代理",
  editProxyConfig: "修改代理配置",
  enabled: "已开启",
  disabled: "已关闭",
  editProxyAddress: "修改代理地址",
  enterProxyAddress: "请输入代理地址",
  cancel: "取消",
  confirm: "确认"
} as const;

const proxyIp = ref(DEFAULT_PROXY_URL);
const proxyEnabled = ref(false);
const proxyButtonStatus = ref(false);
const dialogVisible = ref(false);
const tempProxyUrl = ref("");

onMounted(async () => {
  try {
    const config = await readTextFile(CONFIG_PATH, {
      baseDir: BaseDirectory.AppConfig
    });

    const { config: savedConfig } = JSON.parse(config);
    proxyIp.value = savedConfig?.proxy_ip || DEFAULT_PROXY_URL;
  } catch (error) {
    console.log(labels.useDefaultProxyConfig);
  } finally {
    await checkProxyStatus();
  }
});

async function getGitProxyInfo() {
  openLoading();
  try {
    const resultHttp: string = await invoke("my_custom_command", { command: "git config --global --get http.proxy" });
    const resultHttps: string = await invoke("my_custom_command", { command: "git config --global --get https.proxy" });

    if (!resultHttp.trim() && !resultHttps.trim()) {
      ElMessage.success(labels.unsetProxy);
      return;
    }

    ElMessage.success(`HTTP: ${resultHttp || labels.unset}，HTTPS: ${resultHttps || labels.unset}`);
  } catch (error) {
    console.error(`${labels.getProxyInfoFailed}:`, error);
    ElMessage.error(labels.getProxyInfoFailed);
  } finally {
    closeLoading();
  }
}

async function checkProxyStatus() {
  try {
    const resultHttp: string = await invoke("my_custom_command", { command: "git config --global --get http.proxy" });
    const resultHttps: string = await invoke("my_custom_command", { command: "git config --global --get https.proxy" });
    proxyEnabled.value = !!(resultHttp.trim() || resultHttps.trim());
    proxyButtonStatus.value = proxyEnabled.value;
  } catch (error) {
    console.error(`${labels.checkProxyStatusFailed}:`, error);
    proxyEnabled.value = false;
    proxyButtonStatus.value = false;
  }
}

async function setGitProxy() {
  const proxyUrl = proxyIp.value;
  await invoke("my_custom_command", { command: `git config --global http.proxy ${proxyUrl}` });
  await invoke("my_custom_command", { command: `git config --global https.proxy ${proxyUrl}` });
}

async function disableProxy() {
  await invoke("my_custom_command", { command: "git config --global --unset http.proxy" });
  await invoke("my_custom_command", { command: "git config --global --unset https.proxy" });
  ElMessage.success(labels.proxyClosed);
  proxyEnabled.value = false;
}

async function enableProxy() {
  await setGitProxy();
  proxyEnabled.value = true;
  ElMessage.success(labels.proxyOpened);
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
    console.error(`${labels.proxyOperationFailed}:`, error);
    ElMessage.error(labels.proxyOperationFailed);
    proxyButtonStatus.value = proxyEnabled.value;
  } finally {
    closeLoading();
  }
}

async function openSetProxyDialog() {
  tempProxyUrl.value = proxyIp.value;
  dialogVisible.value = true;
}

async function confirmProxyChange() {
  if (tempProxyUrl.value) {
    proxyIp.value = tempProxyUrl.value;
    try {
      const dirExists = await exists("", { baseDir: BaseDirectory.AppConfig });
      if (!dirExists) {
        await mkdir("", {
          recursive: true,
          baseDir: BaseDirectory.AppConfig
        });
      }

      await writeTextFile(CONFIG_PATH, JSON.stringify({ config: { proxy_ip: proxyIp.value } }), {
        baseDir: BaseDirectory.AppConfig,
        create: true
      });
      ElMessage.success(`${labels.proxyAddressUpdated}: ${proxyIp.value}`);

      if (proxyEnabled.value) {
        await setGitProxy();
      }
    } catch (error) {
      console.error(`${labels.saveProxyConfigFailed}:`, error);
      ElMessage.error(`${labels.saveProxyConfigFailed}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  dialogVisible.value = false;
}

defineExpose({
  proxyIp,
  proxyEnabled,
  toggleProxy,
  openSetProxyDialog
});
</script>

<template>
  <div class="proxy-manager-container">
    <div class="current-proxy">
      <div class="proxy-info">
        <span class="proxy-label">{{ labels.gitProxyAddress }}</span>
        <span class="proxy-url" :title="proxyIp">{{ proxyIp }}</span>
        <el-tooltip :content="labels.viewCurrentGitProxy" placement="top">
          <el-button
            class="icon-action"
            :icon="QuestionFilled"
            circle
            text
            :aria-label="labels.viewCurrentGitProxy"
            @click="getGitProxyInfo"
          />
        </el-tooltip>
      </div>

      <div class="proxy-actions">
        <el-tooltip :content="labels.editProxyConfig" placement="top">
          <el-button
            class="icon-action"
            :icon="EditPen"
            circle
            :aria-label="labels.editProxyConfig"
            @click="openSetProxyDialog"
          />
        </el-tooltip>

        <el-switch
          v-model="proxyButtonStatus"
          @change="toggleProxy"
        />
      </div>
    </div>

    <el-dialog v-model="dialogVisible" :title="labels.editProxyAddress" width="min(480px, 92vw)">
      <el-input v-model="tempProxyUrl" :placeholder="labels.enterProxyAddress" />
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">{{ labels.cancel }}</el-button>
          <el-button type="primary" @click="confirmProxyChange">{{ labels.confirm }}</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.proxy-manager-container {
  height: 100%;
}

.current-proxy {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
  padding: 12px 16px;
  border-radius: 8px;
  background-color: #f7f7f7;
  color: #303133;
  font-size: 14px;
}

.proxy-info {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
  flex: 1;
}

.proxy-label {
  flex: none;
  color: #606266;
}

.proxy-url {
  min-width: 0;
  overflow: hidden;
  color: #409eff;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.proxy-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: none;
}

.icon-action {
  flex: none;
}

@media (max-width: 520px) {
  .current-proxy {
    align-items: stretch;
    flex-direction: column;
  }

  .proxy-actions {
    justify-content: space-between;
  }
}
</style>
