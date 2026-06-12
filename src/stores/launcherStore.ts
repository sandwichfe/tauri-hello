import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { readTextFile, writeTextFile, exists, BaseDirectory } from '@tauri-apps/plugin-fs'
import { open as shellOpen } from '@tauri-apps/plugin-shell'
import { invoke } from '@tauri-apps/api/core'
import { pinyin } from 'pinyin-pro'
import Fuse from 'fuse.js'
import type { LocalShortcut } from '@/types/launcher'

const DATA_FILE = 'launcher-shortcuts.json'
const DESKTOP_ITEM_BASE_TIME = 0

interface DesktopLaunchItem {
  name: string
  path: string
  isDirectory: boolean
}

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2)
}

function generatePinyin(name: string) {
  const full = pinyin(name, { toneType: 'none', type: 'array' }).join('')
  const abbr = pinyin(name, { pattern: 'first', toneType: 'none' }).split(' ').join('')
  return { pinyin: full, pinyinAbbr: abbr }
}

function detectType(filePath: string, isDirectory: boolean): LocalShortcut['type'] {
  if (isDirectory) return 'folder'
  const ext = filePath.split('.').pop()?.toLowerCase()
  if (['exe', 'lnk', 'bat', 'cmd', 'url', 'appref-ms'].includes(ext || '')) return 'app'
  return 'file'
}

function extractName(filePath: string): string {
  const segments = filePath.replace(/\\/g, '/').split('/')
  return segments[segments.length - 1] || filePath
}

function normalizePath(filePath: string): string {
  return filePath.replace(/\\/g, '/').toLowerCase()
}

function createShortcut(options: {
  filePath: string
  isDirectory: boolean
  id: string
  source: LocalShortcut['source']
  addedAt: number
  name?: string
}): LocalShortcut {
  const name = options.name || extractName(options.filePath)
  const type = detectType(options.filePath, options.isDirectory)
  const { pinyin: py, pinyinAbbr } = generatePinyin(name)

  return {
    id: options.id,
    name,
    path: options.filePath,
    type,
    source: options.source,
    pinyin: py,
    pinyinAbbr,
    addedAt: options.addedAt,
  }
}

export const useLauncherStore = defineStore('launcher', () => {
  const shortcuts = ref<LocalShortcut[]>([])
  const desktopShortcuts = ref<LocalShortcut[]>([])
  const searchQuery = ref('')
  const loaded = ref(false)

  let fuse: Fuse<LocalShortcut> | null = null

  const searchableShortcuts = computed<LocalShortcut[]>(() => {
    const manualPaths = new Set(shortcuts.value.map(item => normalizePath(item.path)))
    const discovered = desktopShortcuts.value.filter(item => !manualPaths.has(normalizePath(item.path)))
    return [...shortcuts.value, ...discovered]
  })

  function rebuildIndex() {
    fuse = new Fuse(searchableShortcuts.value, {
      keys: [
        { name: 'name', weight: 2 },
        { name: 'alias', weight: 2.5 },
        { name: 'path', weight: 0.5 },
        { name: 'pinyin', weight: 1.5 },
        { name: 'pinyinAbbr', weight: 1 },
      ],
      threshold: 0.35,
      ignoreLocation: true,
      includeScore: true,
    })
  }

  const searchResults = computed<LocalShortcut[]>(() => {
    const query = searchQuery.value.trim()
    if (!query) {
      return [...searchableShortcuts.value].sort((a, b) => b.addedAt - a.addedAt)
    }
    if (!fuse) rebuildIndex()
    const results = fuse!.search(query)
    return results.map(r => r.item)
  })

  async function load() {
    try {
      const fileExists = await exists(DATA_FILE, { baseDir: BaseDirectory.AppData })
      if (fileExists) {
        const content = await readTextFile(DATA_FILE, { baseDir: BaseDirectory.AppData })
        shortcuts.value = (JSON.parse(content) as LocalShortcut[]).map(item => ({
          ...item,
          source: item.source || 'manual',
        }))
      }
    } catch (e) {
      console.error('Failed to load shortcuts:', e)
      shortcuts.value = []
    }
    await loadDesktopShortcuts()
    rebuildIndex()
    loaded.value = true
  }

  async function loadDesktopShortcuts() {
    try {
      const items = await invoke<DesktopLaunchItem[]>('list_desktop_launch_items')
      desktopShortcuts.value = items.map((item, index) => createShortcut({
        filePath: item.path,
        isDirectory: item.isDirectory,
        name: item.name,
        id: `desktop:${normalizePath(item.path)}`,
        source: 'desktop',
        addedAt: DESKTOP_ITEM_BASE_TIME - index,
      }))
    } catch (e) {
      console.error('Failed to load desktop shortcuts:', e)
      desktopShortcuts.value = []
    }
  }

  async function save() {
    try {
      await writeTextFile(DATA_FILE, JSON.stringify(shortcuts.value, null, 2), {
        baseDir: BaseDirectory.AppData,
      })
    } catch (e) {
      console.error('Failed to save shortcuts:', e)
    }
  }

  async function addShortcut(filePath: string, isDirectory: boolean) {
    const item = createShortcut({
      filePath,
      isDirectory,
      id: generateId(),
      source: 'manual',
      addedAt: Date.now(),
    })

    const existingIndex = shortcuts.value.findIndex(s => normalizePath(s.path) === normalizePath(item.path))
    if (existingIndex >= 0) {
      shortcuts.value[existingIndex] = {
        ...shortcuts.value[existingIndex],
        ...item,
        alias: shortcuts.value[existingIndex].alias,
      }
    } else {
      shortcuts.value.push(item)
    }

    rebuildIndex()
    await save()
    return item
  }

  async function removeShortcut(id: string) {
    shortcuts.value = shortcuts.value.filter(s => s.id !== id)
    rebuildIndex()
    await save()
  }

  async function updateAlias(id: string, alias: string) {
    const item = shortcuts.value.find(s => s.id === id)
    if (!item) return
    item.alias = alias || undefined
    const displayName = alias || item.name
    const { pinyin: py, pinyinAbbr } = generatePinyin(displayName)
    item.pinyin = py
    item.pinyinAbbr = pinyinAbbr
    rebuildIndex()
    await save()
  }

  async function launch(shortcut: LocalShortcut) {
    await shellOpen(shortcut.path)
  }

  return {
    shortcuts,
    desktopShortcuts,
    searchQuery,
    searchResults,
    loaded,
    load,
    addShortcut,
    removeShortcut,
    updateAlias,
    launch,
  }
})
