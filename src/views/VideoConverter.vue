<script setup lang="ts">
import { ref, computed } from 'vue';
import { ElMessage } from 'element-plus';
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
const quality = ref<Quality>('medium');
const loadingText = ref('');
const converting = ref(false);

const outputDirLabel = computed(() => {
  if (outputDir.value) {
    return outputDir.value;
  }
  if (inputPath.value) {
    return '未选择目录，将使用输入文件所在目录';
  }
  return '未选择目录';
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
      return;
    }
    const outputName = getOutputName();
    const outputPath = joinPath(baseDir, outputName);
    await invoke('convert_video_ffmpeg', {
      input: inputPath.value,
      output: outputPath,
      convertType: convertType.value,
      quality: quality.value
    });
    ElMessage.success('转换完成');
  } catch (error) {
    console.error(error);
    ElMessage.error('视频转换失败');
  } finally {
    converting.value = false;
    loadingText.value = '';
  }
};
</script>

<template>
  <div class="video-converter">
    <div class="panel">
      <h2 class="title">
        视频格式转换
      </h2>
      <p class="subtitle">
        支持 AVI 转 MP4 和 M3U8 转 MP4
      </p>

      <div class="form-item">
        <span class="label">
          选择输入文件
        </span>
        <div class="file-row">
          <el-button type="primary" @click="chooseFile" :disabled="converting">
            选择文件
          </el-button>
          <span class="file-name">
            {{ inputName || '未选择文件' }}
          </span>
        </div>
      </div>

      <div class="form-item">
        <span class="label">
          输出目录
        </span>
        <div class="file-row">
          <el-button type="primary" @click="chooseOutputDir" :disabled="converting">
            选择输出目录
          </el-button>
          <span class="file-name">
            {{ outputDirLabel }}
          </span>
        </div>
      </div>

      <div class="form-item">
        <span class="label">
          转换类型
        </span>
        <el-radio-group v-model="convertType" :disabled="converting">
          <el-radio-button label="avi-to-mp4">
            AVI 转 MP4
          </el-radio-button>
          <el-radio-button label="m3u8-to-mp4">
            M3U8 转 MP4
          </el-radio-button>
        </el-radio-group>
      </div>

      <div class="form-item">
        <span class="label">
          导出画质
        </span>
        <el-radio-group v-model="quality" :disabled="converting">
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

      <div class="form-item">
        <span class="label">
          输出文件名
        </span>
        <span class="file-name">
          {{ getOutputName() }}
        </span>
      </div>

      <div class="actions">
        <el-button type="success" :loading="converting" @click="convert">
          开始转换
        </el-button>
      </div>

      <div v-if="converting || loadingText" class="status">
        <div class="status-text">
          {{ loadingText || '正在转换视频，请稍候...' }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.video-converter {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 24px;
  box-sizing: border-box;
}

.panel {
  width: 100%;
  max-width: 720px;
  background-color: #ffffff;
  border-radius: 12px;
  padding: 24px 32px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

.title {
  font-size: 20px;
  margin-bottom: 4px;
  color: #303133;
}

.subtitle {
  margin: 0;
  margin-bottom: 24px;
  color: #909399;
  font-size: 13px;
}

.form-item {
  margin-bottom: 18px;
}

.label {
  display: inline-block;
  margin-bottom: 8px;
  color: #606266;
  font-size: 13px;
}

.file-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.file-name {
  color: #606266;
  font-size: 13px;
  word-break: break-all;
}

.actions {
  margin-top: 8px;
}

.status {
  margin-top: 24px;
}

.status-text {
  margin-top: 8px;
  font-size: 13px;
  color: #606266;
}
</style>
