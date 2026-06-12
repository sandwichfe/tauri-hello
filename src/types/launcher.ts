export interface LocalShortcut {
  id: string
  name: string
  alias?: string
  path: string
  type: 'file' | 'folder' | 'app'
  source?: 'manual' | 'desktop'
  pinyin?: string
  pinyinAbbr?: string
  addedAt: number
}
