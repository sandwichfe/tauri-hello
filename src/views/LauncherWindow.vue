<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window'
import { Document, Folder, Monitor, Search } from '@element-plus/icons-vue'
import { useLauncherStore } from '@/stores/launcherStore'
import type { LocalShortcut } from '@/types/launcher'

const store = useLauncherStore()
const inputRef = ref<HTMLInputElement | null>(null)
const resultListRef = ref<HTMLElement | null>(null)
const panelRef = ref<HTMLElement | null>(null)
const selectedIndex = ref(0)

const WINDOW_WIDTH = 720
const SEARCH_BOX_HEIGHT = 60
const ITEM_HEIGHT = 62
const RESULT_LIST_PADDING = 17
const RESULT_LIST_MAX_HEIGHT = 436

const visibleResults = computed(() => store.searchResults.slice(0, 24))
const hasQuery = computed(() => store.searchQuery.trim().length > 0)

async function resizeWindow() {
  const win = getCurrentWindow()
  if (!hasQuery.value || visibleResults.value.length === 0) {
    await win.setSize(new LogicalSize(WINDOW_WIDTH, SEARCH_BOX_HEIGHT))
    return
  }
  const contentHeight = RESULT_LIST_PADDING + visibleResults.value.length * ITEM_HEIGHT
  const resultHeight = Math.min(contentHeight, RESULT_LIST_MAX_HEIGHT)
  await win.setSize(new LogicalSize(WINDOW_WIDTH, SEARCH_BOX_HEIGHT + resultHeight))
}

onMounted(async () => {
  if (!store.loaded) {
    await store.load()
  }
  await nextTick()
  focusSearch()

  const win = getCurrentWindow()
  window.addEventListener('blur', () => {
    store.searchQuery = ''
    win.hide()
  })
})

watch(
  [hasQuery, visibleResults],
  async () => {
    selectedIndex.value = 0
    await nextTick()
    await resizeWindow()
  }
)

function focusSearch() {
  inputRef.value?.focus()
  inputRef.value?.select()
}

function startDrag(event: MouseEvent) {
  if (event.buttons === 1) {
    getCurrentWindow().startDragging()
  }
}

function getIcon(type: LocalShortcut['type']) {
  if (type === 'folder') return Folder
  if (type === 'app') return Monitor
  return Document
}

function moveSelection(step: number) {
  const count = visibleResults.value.length
  if (count === 0) return
  const next = selectedIndex.value + step
  if (next < 0 || next >= count) return
  selectedIndex.value = next
  nextTick(() => {
    const items = resultListRef.value?.querySelectorAll('.result-item')
    items?.[selectedIndex.value]?.scrollIntoView({ block: 'nearest' })
  })
}

async function launchItem(item: LocalShortcut) {
  await store.launch(item)
  store.searchQuery = ''
  await getCurrentWindow().hide()
}

async function launchSelected() {
  const item = visibleResults.value[selectedIndex.value]
  if (item) {
    await launchItem(item)
  }
}

async function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'ArrowDown') {
    event.preventDefault()
    moveSelection(1)
    return
  }

  if (event.key === 'ArrowUp') {
    event.preventDefault()
    moveSelection(-1)
    return
  }

  if (event.key === 'Enter') {
    event.preventDefault()
    await launchSelected()
    return
  }

  if (event.key === 'Escape') {
    if (store.searchQuery) {
      store.searchQuery = ''
      return
    }
    await getCurrentWindow().hide()
  }
}
</script>

<template>
  <main class="launcher-window" @click="focusSearch">
    <section ref="panelRef" class="launcher-panel">
      <div class="search-box" @mousedown="startDrag">
        <el-icon class="search-icon" :size="25">
          <Search />
        </el-icon>
        <input
          ref="inputRef"
          v-model="store.searchQuery"
          class="search-input"
          placeholder="搜索"
          spellcheck="false"
          @keydown="handleKeydown"
        />
      </div>

      <div v-if="hasQuery" ref="resultListRef" class="result-list">
        <button
          v-for="(item, index) in visibleResults"
          :key="item.id"
          class="result-item"
          :class="{ selected: index === selectedIndex }"
          type="button"
          @mouseenter="selectedIndex = index"
          @click.stop="launchItem(item)"
        >
          <span class="item-icon">
            <el-icon :size="26">
              <component :is="getIcon(item.type)" />
            </el-icon>
          </span>
          <span class="item-main">
            <span class="item-name">
              {{ item.alias || item.name }}
              <span v-if="item.source === 'desktop'" class="item-source">桌面</span>
            </span>
            <span class="item-path">{{ item.path }}</span>
          </span>
        </button>

        <div v-if="visibleResults.length === 0" class="empty-state">没有匹配的项目</div>
      </div>
    </section>
  </main>
</template>

<style scoped>
.launcher-window {
  width: 100%;
  height: 100%;
  padding: 0;
  box-sizing: border-box;
  overflow: hidden;
  background: transparent;
  color: var(--launcher-text);
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

.launcher-panel {
  width: 100%;
  overflow: hidden;
  border: 1px solid var(--launcher-border);
  border-radius: 10px;
  background: var(--launcher-bg);
  box-shadow: 0 18px 55px rgba(15, 23, 42, 0.22), 0 4px 18px rgba(15, 23, 42, 0.12);
  backdrop-filter: blur(22px);
}

.search-box {
  height: 58px;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 5px 14px;
  box-sizing: border-box;
  user-select: none;
}

.search-icon {
  flex: none;
  color: var(--launcher-muted);
}

.search-input {
  flex: 1;
  min-width: 0;
  height: 48px;
  border: none;
  outline: none;
  background: transparent;
  color: var(--launcher-text);
  font-size: 25px;
  font-weight: 300;
  line-height: 1.3;
  letter-spacing: 0;
}

.search-input::placeholder {
  color: var(--launcher-placeholder);
}

.result-list {
  max-height: 436px;
  overflow-y: auto;
  padding: 6px 10px 10px;
  border-top: 1px solid var(--launcher-divider);
  box-sizing: border-box;
}

.result-item {
  width: 100%;
  min-height: 62px;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 10px;
  border: 0;
  border-radius: 8px;
  background: transparent;
  color: inherit;
  text-align: left;
  cursor: pointer;
  box-sizing: border-box;
}

.result-item:hover,
.result-item.selected {
  background: var(--launcher-active);
}

.item-icon {
  width: 38px;
  height: 38px;
  flex: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  color: var(--launcher-primary);
  background: var(--launcher-icon-bg);
}

.item-main {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.item-name {
  display: block;
  overflow: hidden;
  color: var(--launcher-text);
  font-size: 14px;
  font-weight: 600;
  line-height: 20px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-source {
  margin-left: 7px;
  padding: 1px 6px;
  border-radius: 999px;
  background: var(--launcher-chip);
  color: var(--launcher-muted);
  font-size: 11px;
  font-weight: 500;
}

.item-path {
  overflow: hidden;
  color: var(--launcher-muted);
  font-size: 12px;
  line-height: 16px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-state {
  padding: 36px 0 38px;
  color: var(--launcher-muted);
  font-size: 14px;
  text-align: center;
}

:global(html),
:global(body),
:global(#app) {
  margin: 0;
  padding: 0;
  overflow: hidden;
  background: transparent;
}

:global(:root) {
  --launcher-bg: rgba(244, 244, 244, 0.96);
  --launcher-text: #333333;
  --launcher-muted: #6b7280;
  --launcher-placeholder: #7a7a7a;
  --launcher-border: rgba(0, 0, 0, 0.08);
  --launcher-divider: rgba(0, 0, 0, 0.1);
  --launcher-active: #f0f9ff;
  --launcher-primary: #0284c7;
  --launcher-icon-bg: rgba(2, 132, 199, 0.1);
  --launcher-chip: rgba(0, 0, 0, 0.045);
}

@media (prefers-color-scheme: dark) {
  :global(:root) {
    --launcher-bg: rgba(48, 49, 51, 0.96);
    --launcher-text: #f3f4f6;
    --launcher-muted: #bfc0c3;
    --launcher-placeholder: #aaaaaa;
    --launcher-border: rgba(255, 255, 255, 0.1);
    --launcher-divider: rgba(255, 255, 255, 0.1);
    --launcher-active: rgba(255, 255, 255, 0.08);
    --launcher-primary: #38bdf8;
    --launcher-icon-bg: rgba(56, 189, 248, 0.12);
    --launcher-chip: rgba(255, 255, 255, 0.08);
  }
}
</style>
