<template>
  <div 
    class="writing-editor-container"
    :class="{ 
      'is-focus-mode': isFocusMode, 
      'is-typewriter-mode': isTypewriterMode,
    }"
  >
    <!-- Editor Toolbar -->
    <div class="editor-toolbar" :class="{ 'toolbar-dimmed': !isUserActive }">
      <div class="toolbar-group">
        <button 
          @click="editor?.chain().focus().toggleBold().run()" 
          :class="{ 'is-active': editor?.isActive('bold') }"
          class="btn-icon" title="加粗"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path><path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path></svg>
        </button>
        <button 
          @click="editor?.chain().focus().toggleItalic().run()" 
          :class="{ 'is-active': editor?.isActive('italic') }"
          class="btn-icon" title="斜体"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" y1="4" x2="10" y2="4"></line><line x1="14" y1="20" x2="5" y2="20"></line><line x1="15" y1="4" x2="9" y2="20"></line></svg>
        </button>
      </div>
      
      <div class="toolbar-divider"></div>
      
      <div class="toolbar-group">
        <button 
          @click="editor?.chain().focus().toggleHeading({ level: 2 }).run()" 
          :class="{ 'is-active': editor?.isActive('heading', { level: 2 }) }"
          class="btn-icon" title="二级标题"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 12h8"></path><path d="M4 18V6"></path><path d="M12 18V6"></path><path d="M11 7.3a2 2 0 0 0-2-2H4"></path><path d="M11 16.7a2 2 0 0 1-2 2H4"></path></svg>
        </button>
        <button 
          @click="editor?.chain().focus().toggleBulletList().run()" 
          :class="{ 'is-active': editor?.isActive('bulletList') }"
          class="btn-icon" title="无序列表"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="8" y1="6" x2="21" y2="6"></line><line x1="8" y1="12" x2="21" y2="12"></line><line x1="8" y1="18" x2="21" y2="18"></line><line x1="3" y1="6" x2="3.01" y2="6"></line><line x1="3" y1="12" x2="3.01" y2="12"></line><line x1="3" y1="18" x2="3.01" y2="18"></line></svg>
        </button>
      </div>

      <div class="toolbar-divider"></div>

      <div class="toolbar-group">
        <button 
          @click="$emit('toggle-focus-mode')" 
          :class="{ 'is-active': isFocusMode }"
          class="btn-icon mode-toggle" title="段落聚焦模式"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><circle cx="12" cy="12" r="3"></circle></svg>
        </button>
        <button 
          @click="$emit('toggle-typewriter-mode')" 
          :class="{ 'is-active': isTypewriterMode }"
          class="btn-icon mode-toggle" title="打字机居中模式"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="20" x2="12" y2="4"></line><line x1="5" y1="11" x2="12" y2="4"></line><line x1="19" y1="11" x2="12" y2="4"></line></svg>
        </button>
      </div>

      <div class="toolbar-spacer"></div>

      <div class="toolbar-group">
        <button 
          @click="$emit('save')" 
          class="btn-icon save-btn" title="保存 (Ctrl+S)"
          :class="{ 'has-content': charCount > 0 }"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path><polyline points="17 21 17 13 7 13 7 21"></polyline><polyline points="7 3 7 8 15 8"></polyline></svg>
        </button>
      </div>
    </div>

    <!-- Tiptap Editor Content -->
    <div class="editor-scroll-area" ref="scrollAreaRef">
      <editor-content 
        :editor="editor" 
        class="editor-content-area" 
        :style="editorStyle"
      />
    </div>

    <!-- 底部状态栏 -->
    <div class="editor-footer">
      <div class="footer-left">
        <span class="breathing-dot" :class="{ 'is-active': isActivelyTyping }"></span>
        <span class="status-text">{{ isActivelyTyping ? '正在输入...' : '等待输入' }}</span>
      </div>
      <div class="footer-right">
        {{ charCount }} 字 · {{ formatDuration(writingDuration) }}
      </div>
    </div>

    <!-- 字数里程碑动画 -->
    <transition name="milestone">
      <div class="milestone-toast" v-if="milestoneMsg">{{ milestoneMsg }}</div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import Focus from '@tiptap/extension-focus'
import CharacterCount from '@tiptap/extension-character-count'

const props = defineProps<{
  modelValue?: string
  isFocusMode?: boolean
  isTypewriterMode?: boolean
  fontFamily?: string
  fontSize?: number
  lineHeight?: number
  typewriterPosition?: number
}>()

const emit = defineEmits(['update:modelValue', 'toggle-focus-mode', 'toggle-typewriter-mode', 'save'])

// 暴露给父组件
defineExpose({
  getCharCount: () => charCount.value,
  getDuration: () => writingDuration.value,
  getContent: () => editor.value?.getHTML() || '',
})

// === 编辑器样式（由父组件通过 props 控制） ===
const editorStyle = computed(() => ({
  fontFamily: props.fontFamily || "'Inter', sans-serif",
  fontSize: (props.fontSize || 18) + 'px',
  lineHeight: props.lineHeight || 1.8,
}))

// === 字数 & 计时 ===
const charCount = ref(0)
const writingDuration = ref(0)
let timerInterval: any = null

// === 打字状态 ===
const isActivelyTyping = ref(false)
const isUserActive = ref(true)
let typingTimer: any = null
let activeTimer: any = null

const scrollAreaRef = ref<HTMLElement | null>(null)

const markTyping = () => {
  isActivelyTyping.value = true
  isUserActive.value = true
  if (typingTimer) clearTimeout(typingTimer)
  typingTimer = setTimeout(() => { isActivelyTyping.value = false }, 1500)
  if (activeTimer) clearTimeout(activeTimer)
  activeTimer = setTimeout(() => { isUserActive.value = false }, 4000)
}

// === 字数里程碑 ===
const milestoneMsg = ref('')
const lastMilestone = ref(0)
const milestones = [100, 200, 500, 1000, 1500, 2000, 3000, 5000]

const checkMilestone = (count: number) => {
  for (const m of milestones) {
    if (count >= m && lastMilestone.value < m) {
      lastMilestone.value = m
      milestoneMsg.value = m >= 1000 ? `🎉 ${m / 1000}k 字！太棒了！` : `✨ ${m} 字！继续加油！`
      setTimeout(() => { milestoneMsg.value = '' }, 2500)
      break
    }
  }
}

// === Ctrl+S 快捷键 ===
const onKeyDown = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 's') {
    e.preventDefault()
    emit('save')
  }
}
onMounted(() => { window.addEventListener('keydown', onKeyDown) })
onBeforeUnmount(() => { window.removeEventListener('keydown', onKeyDown) })

// === Tiptap ===
const editor = useEditor({
  content: props.modelValue || '',
  extensions: [
    StarterKit,
    Placeholder.configure({ placeholder: '在这里开始书写，让思绪自然流淌...' }),
    Focus.configure({ className: 'has-focus', mode: 'all' }),
    CharacterCount.configure({ limit: null }),
  ],
  onUpdate: ({ editor }) => {
    emit('update:modelValue', editor.getHTML())
    const newCount = editor.getText().replace(/\s/g, '').length
    charCount.value = newCount
    checkMilestone(newCount)
    markTyping()

    if (props.isTypewriterMode) scrollToCaret(editor)
  },
  onSelectionUpdate: ({ editor }) => {
    if (props.isTypewriterMode) scrollToCaret(editor)
  },
})

// === 打字机滚动：光标固定在设置指定的垂直位置 ===
const scrollToCaret = (editorInstance: any) => {
  requestAnimationFrame(() => {
    const { view } = editorInstance
    if (!view) return
    const container = scrollAreaRef.value
    if (!container) return
    try {
      const coords = view.coordsAtPos(view.state.selection.head)
      const containerRect = container.getBoundingClientRect()
      const ratio = (props.typewriterPosition || 38) / 100
      const caretRelativeY = coords.top - containerRect.top
      const targetOffset = caretRelativeY - containerRect.height * ratio
      container.scrollTop += targetOffset
    } catch { /* ignore */ }
  })
}

// === 计时器 ===
onMounted(() => {
  timerInterval = setInterval(() => { writingDuration.value++ }, 1000)
})

onBeforeUnmount(() => {
  if (timerInterval) clearInterval(timerInterval)
  if (typingTimer) clearTimeout(typingTimer)
  if (activeTimer) clearTimeout(activeTimer)
  editor.value?.destroy()
})

const formatDuration = (seconds: number) => {
  const m = Math.floor(seconds / 60)
  const s = seconds % 60
  return `${m}分${s < 10 ? '0' : ''}${s}秒`
}

watch(() => props.modelValue, (newVal) => {
  if (editor.value && newVal && editor.value.getHTML() !== newVal) {
    editor.value.commands.setContent(newVal, false)
  }
})
</script>

<style scoped>
.writing-editor-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
  min-height: 0;
}

.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
  opacity: 1;
  transition: opacity 0.5s ease;
}
.editor-toolbar.toolbar-dimmed { opacity: 0.25; }
.editor-toolbar:hover { opacity: 1 !important; }

.toolbar-group { display: flex; gap: 2px; }
.toolbar-divider { width: 1px; height: 16px; background-color: var(--border-subtle); margin: 0 4px; }

.btn-icon.is-active, .mode-toggle.is-active {
  background-color: var(--bg-surface-hover);
  color: var(--text-primary);
}
.mode-toggle.is-active {
  color: var(--accent-primary);
  background-color: rgba(59, 130, 246, 0.1);
}

.editor-scroll-area {
  flex: 1;
  overflow-y: auto;
  padding: 16px 0;
  min-height: 0;
}

.editor-content-area {
  width: 100%;
  cursor: text;
  padding-bottom: 55vh; /* 打字机模式底部留白 */
}

:deep(.ProseMirror) {
  outline: none;
  color: var(--text-primary);
  min-height: 300px;
}
:deep(.ProseMirror p) {
  margin-bottom: 1em;
  transition: opacity 0.4s ease;
}
:deep(.ProseMirror h1),
:deep(.ProseMirror h2),
:deep(.ProseMirror h3) {
  margin-top: 1.5em;
  margin-bottom: 0.8em;
}
:deep(.ProseMirror p.is-editor-empty:first-child::before) {
  content: attr(data-placeholder);
  float: left;
  color: var(--text-tertiary);
  pointer-events: none;
  height: 0;
}

.is-focus-mode :deep(.ProseMirror > *) { opacity: 0.25; transition: opacity 0.4s ease; }
.is-focus-mode :deep(.ProseMirror > .has-focus) { opacity: 1; }

.editor-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 4px;
  border-top: 1px solid var(--border-subtle);
  background: var(--bg-base);
  flex-shrink: 0;
}
.footer-left { display: flex; align-items: center; gap: 8px; }
.footer-right {
  font-size: 0.85rem; color: var(--text-tertiary);
  font-weight: 500; font-variant-numeric: tabular-nums;
}
.status-text { font-size: 0.75rem; color: var(--text-tertiary); }

.breathing-dot {
  display: inline-block; width: 10px; height: 10px;
  border-radius: 50%; background-color: var(--border-focus);
  transition: background-color 0.3s ease; flex-shrink: 0;
}
.breathing-dot.is-active {
  background-color: #10b981;
  box-shadow: 0 0 6px 2px rgba(16, 185, 129, 0.5);
  animation: breathe 1.8s ease-in-out infinite;
}
@keyframes breathe {
  0%, 100% { transform: scale(1); box-shadow: 0 0 6px 2px rgba(16, 185, 129, 0.5); }
  50% { transform: scale(1.3); box-shadow: 0 0 12px 4px rgba(16, 185, 129, 0.3); }
}

.toolbar-spacer { flex: 1; }

.save-btn { color: var(--text-tertiary); }
.save-btn.has-content { color: var(--text-secondary); }
.save-btn:hover { color: var(--accent-primary); }

/* 里程碑 toast */
.milestone-toast {
  position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: #fff; padding: 14px 32px; border-radius: 16px;
  font-size: 1.1rem; font-weight: 600; letter-spacing: 0.02em;
  box-shadow: 0 8px 32px rgba(99, 102, 241, 0.35);
  z-index: 20; pointer-events: none;
}
.milestone-enter-active { transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1); }
.milestone-leave-active { transition: all 0.3s ease; }
.milestone-enter-from { opacity: 0; transform: translate(-50%, -50%) scale(0.6); }
.milestone-leave-to { opacity: 0; transform: translate(-50%, -50%) translateY(-20px); }
</style>
