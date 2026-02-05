<script setup lang="ts">
import { ref, computed } from 'vue';
import { ElMessage } from 'element-plus';
import { Setting } from '@element-plus/icons-vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

type ConvertType = 'avi-to-mp4' | 'm3u8-to-mp4';
type Quality = 'high' | 'medium' | 'low';

const inputPath = ref('');
const outputDir = ref('');

const inputName = computed(() => {
  if (!inputPath.value) {
    return '';
  }
  const parts = inputPath.value.split(/[\\/]/);
  return parts[parts.length - 1];
});

const convertType = ref<ConvertType>('avi-to-mp4');
const quality = ref<Quality>('high');
const loadingText = ref('');
const converting = ref(false);
const settingsVisible = ref(false);
const logDialogVisible = ref(false);
const logContent = ref('');

const outputDirLabel = computed(() => {
  if (outputDir.value) {
    return outputDir.value;
  }
  if (inputPath.value) {
    return '未选择目录，将使用输入文件所在目录';
  }
  return '未选择目录';
});

const qualityLabel = computed(() => {
  if (quality.value === 'high') {
    return '高画质';
  }
  if (quality.value === 'low') {
    return '低画质';
  }
  return '中等画质';
});

const detectTypeFromPath = () => {
  if (!inputPath.value) {
    return;
  }
  const lower = inputPath.value.toLowerCase();
  if (lower.endsWith('.m3u8')) {
    convertType.value = 'm3u8-to-mp4';
  } else if (lower.endsWith('.avi')) {
    convertType.value = 'avi-to-mp4';
  }
};

const chooseFile = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: '视频文件',
          extensions: ['avi', 'm3u8']
        }
      ]
    });
    if (!selected || Array.isArray(selected)) {
      return;
    }
    inputPath.value = selected;
    detectTypeFromPath();
  } catch (error) {
    console.error(error);
    ElMessage.error('选择文件失败');
  }
};

const chooseOutputDir = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false
    });
    if (!selected || Array.isArray(selected)) {
      return;
    }
    outputDir.value = selected;
  } catch (error) {
    console.error(error);
    ElMessage.error('选择输出目录失败');
  }
};

const getOutputName = () => {
  if (!inputName.value) {
    return 'output.mp4';
  }
  const name = inputName.value;
  const index = name.lastIndexOf('.');
  if (index === -1) {
    return `${name}.mp4`;
  }
  return `${name.slice(0, index)}.mp4`;
};

const getInputDir = () => {
  if (!inputPath.value) {
    return '';
  }
  const path = inputPath.value;
  const lastSlash = path.lastIndexOf('/');
  const lastBackslash = path.lastIndexOf('\\');
  const index = Math.max(lastSlash, lastBackslash);
  if (index === -1) {
    return '';
  }
  return path.slice(0, index);
};

const joinPath = (dir: string, name: string) => {
  if (!dir) {
    return name;
  }
  const normalizedDir = dir.replace(/[\\/]+$/, '');
  const sep = normalizedDir.includes('\\') ? '\\' : '/';
  return `${normalizedDir}${sep}${name}`;
};

const convert = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请先选择要转换的视频文件');
    return;
  }
  converting.value = true;
  loadingText.value = '正在转换视频，请稍候...';
  try {
    const baseDir = outputDir.value || getInputDir();
    if (!baseDir) {
      ElMessage.error('无法确定输出目录，请先选择输出目录或有效输入文件');
      converting.value = false;
      loadingText.value = '';
      return;
    }
    const outputName = getOutputName();
    const outputPath = joinPath(baseDir, outputName);
    const result = await invoke<string>('convert_video_ffmpeg', {
      input: inputPath.value,
      output: outputPath,
      convertType: convertType.value,
      quality: quality.value
    });
    logContent.value = result || '';
    loadingText.value = '转换成功';
    ElMessage.success('转换完成');
  } catch (error) {
    console.error(error);
    logContent.value =
      (error as { message?: string }).message ||
      String(error);
    loadingText.value = '转换失败';
    ElMessage.error('视频转换失败');
  } finally {
    converting.value = false;
  }
};
</script>

<template>
  <div class="video-converter">
    <div class="task-bar">
      <div class="file-info">
        <div class="file-row-main">
          <span class="file-name-primary">
            {{ inputName || '请选择要转换的视频文件' }}
          </span>
          <el-tag v-if="convertType === 'avi-to-mp4'" size="small" type="info">
            AVI → MP4
          </el-tag>
          <el-tag v-else size="small" type="info">
            M3U8 → MP4
          </el-tag>
          <el-button
            circle
            text
            class="settings-btn"
            @click="settingsVisible = true"
          >
            <el-icon>
              <Setting />
            </el-icon>
          </el-button>
        </div>
        <div class="file-row-sub">
          <span class="sub-text">
            输出到：{{ outputDirLabel }}
          </span>
          <span class="sub-text">
            画质：{{ qualityLabel }}
          </span>
          <span class="sub-text">
            输出文件名：{{ getOutputName() }}
          </span>
        </div>
      </div>

      <div class="task-actions">
        <el-button type="primary" @click="chooseFile" :disabled="converting">
          选择文件
        </el-button>
        <el-button @click="chooseOutputDir" :disabled="converting">
          输出目录
        </el-button>
        <el-button
          type="danger"
          class="convert-btn"
          :loading="converting"
          @click="convert"
        >
          转换
        </el-button>
      </div>
    </div>

    <div v-if="converting || loadingText || logContent" class="status">
      <el-progress
        v-if="converting || logContent"
        :percentage="100"
        :indeterminate="converting"
        stroke-width="3"
        :status="converting ? undefined : 'success'"
      />
      <div class="status-text">
        {{ loadingText }}
        <el-button
          v-if="logContent"
          link
          type="primary"
          class="log-link"
          @click="logDialogVisible = true"
        >
          查看详细日志
        </el-button>
      </div>
    </div>

    <el-dialog
      v-model="settingsVisible"
      title="设置"
      width="520px"
      class="settings-dialog"
    >
      <el-tabs model-value="video" class="settings-tabs">
        <el-tab-pane label="视频" name="video">
          <div class="settings-form">
            <div class="settings-item">
              <span class="settings-label">
                转换类型
              </span>
              <el-radio-group
                v-model="convertType"
                size="small"
                :disabled="converting"
              >
                <el-radio-button label="avi-to-mp4">
                  AVI 转 MP4
                </el-radio-button>
                <el-radio-button label="m3u8-to-mp4">
                  M3U8 转 MP4
                </el-radio-button>
              </el-radio-group>
            </div>

            <div class="settings-item">
              <span class="settings-label">
                导出画质
              </span>
              <el-radio-group
                v-model="quality"
                size="small"
                :disabled="converting"
              >
                <el-radio-button label="high">
                  高画质
                </el-radio-button>
                <el-radio-button label="medium">
                  中等画质
                </el-radio-button>
                <el-radio-button label="low">
                  低画质
                </el-radio-button>
              </el-radio-group>
            </div>
          </div>
        </el-tab-pane>
        <el-tab-pane label="音频" name="audio">
          <div class="placeholder-tip">
            暂未提供音频参数设置
          </div>
        </el-tab-pane>
      </el-tabs>

      <template #footer>
        <span class="dialog-footer">
          <el-button @click="settingsVisible = false">
            取消
          </el-button>
          <el-button type="primary" @click="settingsVisible = false">
            确定
          </el-button>
        </span>
      </template>
    </el-dialog>

    <el-dialog
      v-model="logDialogVisible"
      title="转换日志"
      width="640px"
    >
      <pre class="log-content">
{{ logContent }}
      </pre>
      <template #footer>
        <span class="dialog-footer">
          <el-button type="primary" @click="logDialogVisible = false">
            关闭
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.video-converter {
  height: 100%;
  padding: 20px 24px;
  box-sizing: border-box;
}

.task-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #ffffff;
  border-radius: 12px;
  padding: 16px 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-row-main {
  display: flex;
  align-items: center;
  gap: 10px;
}

.file-row-sub {
  margin-top: 6px;
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.file-name {
  color: #606266;
  font-size: 13px;
  word-break: break-all;
}

.file-name-primary {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  max-width: 360px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sub-text {
  font-size: 12px;
  color: #909399;
}

.settings-btn {
  margin-left: 4px;
}

.task-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-left: 24px;
}

.convert-btn {
  padding: 10px 26px;
  font-size: 15px;
  font-weight: 600;
}

.status {
  margin-top: 12px;
}

.status-text {
  margin-top: 8px;
  font-size: 13px;
  color: #606266;
}

.log-link {
  margin-left: 8px;
}

.settings-dialog :deep(.el-dialog__body) {
  background-color: #202124;
  color: #e5e5e5;
  padding-top: 10px;
}

.settings-dialog :deep(.el-dialog__header) {
  background-color: #202124;
  border-bottom: none;
  color: #e5e5e5;
}

.settings-dialog :deep(.el-dialog__title) {
  color: #e5e5e5;
}

.settings-tabs {
  margin-top: 4px;
}

.settings-tabs :deep(.el-tabs__header) {
  margin: 0 0 12px;
}

.settings-tabs :deep(.el-tabs__nav-wrap)::after {
  background-color: transparent;
}

.settings-form {
  padding: 4px 4px 8px;
}

.settings-item {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
}

.settings-label {
  width: 90px;
  font-size: 13px;
  color: #e5e5e5;
}

.settings-item :deep(.el-radio-button__inner) {
  background-color: #303134;
  border-color: #424346;
  color: #e5e5e5;
}

.settings-item :deep(.el-radio-button__original-radio:checked + .el-radio-button__inner) {
  background-color: #409eff;
  border-color: #409eff;
  color: #ffffff;
}

.placeholder-tip {
  padding: 12px 4px;
  font-size: 13px;
  color: #b0b0b0;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.log-content {
  max-height: 360px;
  overflow: auto;
  font-size: 12px;
  line-height: 1.5;
  background-color: #1e1e1e;
  color: #e5e5e5;
  padding: 12px;
  border-radius: 4px;
}
</style>
