<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { Delete, Document, Folder, Monitor, Search } from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useLauncherStore } from '@/stores/launcherStore'
import type { LocalShortcut } from '@/types/launcher'

const store = useLauncherStore()
const editingItem = ref<LocalShortcut | null>(null)
const aliasInput = ref('')
const showAliasDialog = computed({
  get: () => editingItem.value !== null,
  set: (value: boolean) => {
    if (!value) editingItem.value = null
  },
})

onMounted(() => {
  if (!store.loaded) {
    store.load()
  }
})

async function openLauncherWindow() {
  await invoke('open_launcher_window')
}

async function addFile() {
  const selected = await openDialog({ multiple: true, directory: false })
  if (!selected) return

  const paths = Array.isArray(selected) ? selected : [selected]
  for (const path of paths) {
    await store.addShortcut(path, false)
  }
  ElMessage.success(`已添加 ${paths.length} 个文件`)
}

async function addFolder() {
  const selected = await openDialog({ multiple: false, directory: true })
  if (!selected) return

  await store.addShortcut(selected, true)
  ElMessage.success('已添加文件夹')
}

async function handleRemove(item: LocalShortcut) {
  if (item.source === 'desktop') return

  try {
    await ElMessageBox.confirm(`确定删除「${item.alias || item.name}」？`, '删除确认', {
      confirmButtonText: '删除',
      cancelButtonText: '取消',
      type: 'warning',
    })
    await store.removeShortcut(item.id)
    ElMessage.success('已删除')
  } catch {
    // user cancelled
  }
}

function handleLaunch(item: LocalShortcut) {
  store.launch(item)
}

function startEdit(item: LocalShortcut) {
  if (item.source === 'desktop') return

  editingItem.value = item
  aliasInput.value = item.alias || ''
}

async function saveAlias() {
  if (!editingItem.value) return

  await store.updateAlias(editingItem.value.id, aliasInput.value.trim())
  editingItem.value = null
  aliasInput.value = ''
}

function getIcon(type: LocalShortcut['type']) {
  if (type === 'folder') return Folder
  if (type === 'app') return Monitor
  return Document
}
</script>

<template>
  <div class="launcher-view">
    <div class="launcher-header">
      <el-button type="primary" @click="openLauncherWindow">打开启动器</el-button>
      <el-input
        v-model="store.searchQuery"
        placeholder="搜索快捷方式..."
        :prefix-icon="Search"
        clearable
        class="search-input"
      />
      <el-button-group class="action-buttons">
        <el-button type="primary" plain size="small" @click="addFile">+ 文件</el-button>
        <el-button type="primary" plain size="small" @click="addFolder">+ 文件夹</el-button>
      </el-button-group>
    </div>

    <div v-if="store.loaded" class="launcher-list">
      <div v-if="store.searchResults.length === 0" class="empty-state">
        <p>
          {{
            store.searchQuery.trim()
              ? '没有找到匹配的快捷方式'
              : '暂无快捷方式，点击上方按钮添加'
          }}
        </p>
      </div>

      <div
        v-for="item in store.searchResults"
        :key="item.id"
        class="shortcut-item"
        @click="handleLaunch(item)"
      >
        <el-icon class="item-icon" :size="20">
          <component :is="getIcon(item.type)" />
        </el-icon>
        <div class="item-info">
          <span class="item-name">
            {{ item.alias || item.name }}
            <span v-if="item.source === 'desktop'" class="item-source">桌面</span>
          </span>
          <span class="item-path">{{ item.path }}</span>
        </div>
        <div v-if="item.source !== 'desktop'" class="item-actions" @click.stop>
          <el-button size="small" text @click="startEdit(item)">别名</el-button>
          <el-button size="small" text type="danger" :icon="Delete" @click="handleRemove(item)" />
        </div>
      </div>
    </div>

    <el-dialog v-model="showAliasDialog" title="设置别名" width="400">
      <el-input
        v-model="aliasInput"
        placeholder="输入别名，留空使用原名"
        @keyup.enter="saveAlias"
      />
      <template #footer>
        <el-button @click="editingItem = null">取消</el-button>
        <el-button type="primary" @click="saveAlias">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.launcher-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  box-sizing: border-box;
}

.launcher-header {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.search-input {
  flex: 1;
}

.action-buttons {
  flex-shrink: 0;
}

.launcher-list {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.empty-state {
  text-align: center;
  color: #999;
  padding: 40px 0;
}

.shortcut-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.15s;
}

.shortcut-item:hover {
  background-color: var(--el-fill-color-light);
}

.item-icon {
  flex-shrink: 0;
  color: var(--el-color-primary);
}

.item-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.item-name {
  overflow: hidden;
  font-size: 14px;
  font-weight: 500;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-source {
  margin-left: 6px;
  padding: 1px 5px;
  border-radius: 4px;
  background-color: var(--el-fill-color-light);
  color: var(--el-text-color-secondary);
  font-size: 11px;
  font-weight: 400;
}

.item-path {
  overflow: hidden;
  color: #999;
  font-size: 12px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-actions {
  flex-shrink: 0;
  opacity: 0;
  transition: opacity 0.15s;
}

.shortcut-item:hover .item-actions {
  opacity: 1;
}
</style>
