import { AppDispatch } from '@/app/store'
import store from '@/app/store'
import {
  toggleNoDistractionsMode,
  togglePreviewOnly,
  toggleSidebar,
  toggleSplitMode,
} from '@/features/appSlice'
import { NebulaModal } from '@/features/modalSlice'
import { updatePage } from '@/utils/notebook'
import { invoke } from '@tauri-apps/api/tauri'

export const isInEditor = () => window.location.pathname.startsWith('/editor')

export const shortcuts: Record<string, string> = {
  'ctrl+.': 'view:toggle-sidebar',
  'ctrl+shift+d': 'view:toggle-no-distractions',
  'ctrl+/': 'view:toggle-preview-only',
  'ctrl+shift+?': 'view:toggle-split-mode',
  'ctrl+n': 'core:add-page',
  'ctrl+shift+n': 'core:add-sub-page',
  'ctrl+s': 'core:save-current-notebook',
}
export const handlers: Record<string, (dispatch: AppDispatch) => void> = {
  'core:add-page': (dispatch) => {
    if (isInEditor()) {
      const currentPage = store.getState().editor.currentPage
      dispatch(
        NebulaModal.showModal({
          type: 'page/create',
          id: 'Create Page Keyboard',
          parentId: currentPage !== null ? currentPage.parent_id : null,
          insertAfterId: currentPage !== null ? currentPage.__id : null,
          label: 'Create Page',
          x: window.innerWidth / 2 - 100,
          y: window.innerHeight / 2 - 100,
          subtractHalfWidth: true,
        })
      )
    }
  },
  'core:save-current-notebook': async (dispatch) => {
    if (isInEditor()) {
      let state = store.getState()
      let currentPageId = state.editor.currentPage?.__id
      let currentContent = state.editor.currentDoc
      if (currentPageId) {
        await updatePage(currentPageId, currentContent)
      }

      let res = await invoke('save_notebook')
      console.log(res)
    }
  },

  'core:add-sub-page': (dispatch) => {
    if (isInEditor()) {
      const currentPage = store.getState().editor.currentPage
      if (currentPage) {
        dispatch(
          NebulaModal.showModal({
            type: 'page/create',
            id: 'Create Subpage Keyboard',
            parentId: currentPage !== null ? currentPage.__id : null,
            insertAfterId: null,
            label: 'Create Sub-Page',
            x: window.innerWidth / 2 - 100,
            y: window.innerHeight / 2 - 100,
            subtractHalfWidth: true,
          })
        )
      }
    }
  },
  'view:toggle-split-mode': (dispatch) => {
    if (isInEditor()) {
      dispatch(toggleSplitMode())
    }
  },
  'view:toggle-sidebar': (dispatch) => {
    if (isInEditor()) {
      dispatch(toggleSidebar())
    }
  },
  'view:toggle-no-distractions': (dispatch) => {
    if (isInEditor()) {
      dispatch(toggleNoDistractionsMode())
    }
  },
  'view:toggle-preview-only': (dispatch) => {
    if (isInEditor()) {
      dispatch(togglePreviewOnly())
    }
  },
}
export const getHandler = (command: string) => {
  return handlers[command]
}
export const getCommand = (binding: string): string | undefined => {
  return shortcuts[binding]
}
export const getKeyBindings = (ev: KeyboardEvent) => {
  const ctrlKey = ev.getModifierState('Control')
  const shiftKey = ev.getModifierState('Shift')
  const altKey = ev.getModifierState('Alt')
  const binding = []
  if (ctrlKey) binding.push('ctrl')
  if (shiftKey) binding.push('shift')
  if (altKey) binding.push('alt')

  if (!['Control', 'Shift', 'Alt'].includes(ev.key))
    binding.push(ev.key.toLowerCase())

  return binding.join('+')
}