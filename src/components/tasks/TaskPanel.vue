<template>
  <div class="task-panel">
    <!-- 顶部标题栏 -->
    <header class="panel-header">
      <div>
        <h1 class="panel-title">待办任务</h1>
        <p class="panel-subtitle text-tertiary">{{ unfinishedCount }} 项未完成</p>
      </div>
      <div class="header-actions">
        <button class="btn-ghost btn-sm" @click="showImportDialog = true" title="导入任务">
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
          导入
        </button>
        <button class="btn-primary btn-sm" @click="showAddForm = true" v-if="!showAddForm">
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
          新建任务
        </button>
      </div>
    </header>

    <!-- 新建任务表单 -->
    <div class="add-task-form" v-if="showAddForm">
      <input 
        ref="newTaskInput"
        v-model="newTask.title" 
        @keydown.enter="addTask"
        @keydown.escape="cancelAdd"
        placeholder="输入任务标题..."
        class="task-input"
        autofocus
      />
      <div class="form-row">
        <select v-model="newTask.priority" class="task-select">
          <option value="low">低优先级</option>
          <option value="medium">中优先级</option>
          <option value="high">高优先级</option>
        </select>
        <input 
          v-model="newTask.due_date" 
          type="date" 
          class="task-date-input"
          placeholder="截止日期"
        />
        <div class="form-actions">
          <button class="btn-primary btn-sm" @click="addTask" :disabled="!newTask.title.trim()">添加</button>
          <button class="btn-ghost btn-sm" @click="cancelAdd">取消</button>
        </div>
      </div>
    </div>

    <!-- 筛选标签 -->
    <div class="filter-tabs">
      <button 
        v-for="tab in filterTabs" :key="tab.key"
        class="filter-tab" 
        :class="{ active: activeFilter === tab.key }"
        @click="activeFilter = tab.key"
      >
        {{ tab.label }}
        <span class="tab-count" v-if="tab.count > 0">{{ tab.count }}</span>
      </button>
    </div>

    <!-- 任务列表 -->
    <div class="task-list" v-if="filteredTasks.length">
      <div 
        class="task-item" 
        v-for="task in filteredTasks" 
        :key="task.id ?? 0"
        :class="{ 
          'is-completed': task.completed, 
          'is-editing': editingId === task.id,
          'priority-high': task.priority === 'high',
          'priority-medium': task.priority === 'medium',
        }"
      >
        <!-- 勾选框 -->
        <button class="task-checkbox" @click="toggleComplete(task)">
          <svg v-if="task.completed" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
          <svg v-else xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle></svg>
        </button>

        <!-- 任务内容 -->
        <div class="task-body" v-if="editingId !== task.id" @dblclick="startEdit(task)">
          <span class="task-title">{{ task.title }}</span>
          <div class="task-meta">
            <span class="priority-badge" :class="'p-' + task.priority">{{ priorityLabel(task.priority) }}</span>
            <span class="due-date" v-if="task.due_date" :class="{ overdue: isOverdue(task.due_date) }">
              {{ formatDate(task.due_date) }}
            </span>
          </div>
        </div>

        <!-- 编辑模式 -->
        <div class="task-edit" v-else>
          <input 
            v-model="editForm.title" 
            @keydown.enter="saveEdit"
            @keydown.escape="cancelEdit"
            class="task-input"
            ref="editInput"
          />
          <div class="form-row compact">
            <select v-model="editForm.priority" class="task-select">
              <option value="low">低</option>
              <option value="medium">中</option>
              <option value="high">高</option>
            </select>
            <input v-model="editForm.due_date" type="date" class="task-date-input" />
            <button class="btn-primary btn-sm" @click="saveEdit">保存</button>
            <button class="btn-ghost btn-sm" @click="cancelEdit">取消</button>
          </div>
        </div>

        <!-- 删除按钮 -->
        <button class="task-delete" @click="removeTask(task)" title="删除" v-if="editingId !== task.id">
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
        </button>
      </div>
    </div>

    <!-- 空状态 -->
    <div class="empty-state" v-else>
      <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" style="opacity:0.3"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
      <p class="empty-text">{{ activeFilter === 'completed' ? '还没有已完成的任务' : '没有待办任务，享受清闲吧！' }}</p>
      <button class="btn-primary btn-sm" @click="showAddForm = true" v-if="activeFilter !== 'completed'">添加第一个任务</button>
    </div>

    <!-- 导入对话框 -->
    <div class="import-overlay" v-if="showImportDialog" @click.self="showImportDialog = false">
      <div class="import-dialog">
        <h3 class="import-title">批量导入任务</h3>
        <p class="import-desc">粘贴 JSON 格式的任务列表，每个任务包含 title（必填）、priority、due_date</p>
        <textarea v-model="importJson" class="import-textarea" placeholder='[
  { "title": "完成报告", "priority": "high", "due_date": "2026-02-25" },
  { "title": "读书笔记", "priority": "medium" }
]'></textarea>
        <div class="import-actions">
          <span class="import-hint" v-if="importError">{{ importError }}</span>
          <span class="import-hint import-success" v-if="importSuccess">{{ importSuccess }}</span>
          <button class="btn-ghost btn-sm" @click="showImportDialog = false; importJson = ''; importError = ''; importSuccess = ''">取消</button>
          <button class="btn-primary btn-sm" @click="importTasks" :disabled="!importJson.trim()">导入</button>
        </div>
      </div>
    </div>

    <!-- Toast 提示 -->
    <transition name="toast">
      <div class="toast" v-if="toastMsg">{{ toastMsg }}</div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, onMounted } from 'vue'
import type { Task, CreateTaskRequest, Priority } from '../../types'
import { requestPermission, notifyTodayTasks, notifyTaskCompleted } from '../../utils/notify'

// 由于开发阶段可能不在 Tauri 环境中运行，提供 mock fallback
let api: any = null
import { isTauri } from '../../utils/env'

const tasks = ref<Task[]>([])
const showAddForm = ref(false)
const showImportDialog = ref(false)
const importJson = ref('')
const importError = ref('')
const importSuccess = ref('')
const toastMsg = ref('')
const activeFilter = ref<'all' | 'today' | 'completed'>('all')
const editingId = ref<number | null>(null)
const newTaskInput = ref<HTMLInputElement>()
const editInput = ref<HTMLInputElement>()

const newTask = ref<CreateTaskRequest>({
  title: '',
  priority: 'medium',
  due_date: '',
})

const editForm = ref({
  title: '',
  priority: 'medium' as Priority,
  due_date: '',
})

// === 计算属性 ===
const unfinishedCount = computed(() => tasks.value.filter(t => !t.completed).length)

const todayStr = computed(() => {
  const d = new Date()
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
})

const todayTasks = computed(() => 
  tasks.value.filter(t => !t.completed && t.due_date && t.due_date <= todayStr.value)
)

const filterTabs = computed(() => [
  { key: 'all' as const, label: '全部', count: tasks.value.filter(t => !t.completed).length },
  { key: 'today' as const, label: '今日', count: todayTasks.value.length },
  { key: 'completed' as const, label: '已完成', count: tasks.value.filter(t => t.completed).length },
])

const filteredTasks = computed(() => {
  switch (activeFilter.value) {
    case 'today': return todayTasks.value
    case 'completed': return tasks.value.filter(t => t.completed)
    default: return tasks.value.filter(t => !t.completed)
  }
})

// === 方法 ===
const loadTasks = async () => {
  if (isTauri) {
    try {
      if (!api) {
        api = await import('../../api')
      }
      tasks.value = await api.getTasks()
    } catch (e) {
      console.warn('无法加载任务（后端未启动？）', e)
    }
  }
  // 开发模式 mock 数据
  if (!tasks.value.length && !isTauri) {
    tasks.value = [
      { id: 1, title: '完成 Phase 2 任务管理面板', priority: 'high', due_date: todayStr.value, completed: false, created_at: null, updated_at: null },
      { id: 2, title: '阅读《写作这回事》第三章', priority: 'medium', due_date: todayStr.value, completed: false, created_at: null, updated_at: null },
      { id: 3, title: '整理新博文素材', priority: 'low', due_date: null, completed: false, created_at: null, updated_at: null },
      { id: 4, title: '修复编辑器呼吸灯', priority: 'high', due_date: null, completed: true, created_at: null, updated_at: null },
    ]
  }

  // 请求通知权限并发送今日待办提醒
  await requestPermission()
  const todayCount = tasks.value.filter(t => !t.completed && t.due_date && t.due_date <= todayStr.value).length
  if (todayCount > 0) notifyTodayTasks(todayCount)
}

const addTask = async () => {
  const title = newTask.value.title.trim()
  if (!title) return

  if (isTauri && api) {
    try {
      const created = await api.createTask({
        title,
        priority: newTask.value.priority,
        due_date: newTask.value.due_date || undefined,
      })
      tasks.value.unshift(created)
    } catch (e) {
      console.error('创建任务失败', e)
    }
  } else {
    tasks.value.unshift({
      id: Date.now(),
      title,
      priority: newTask.value.priority || 'medium',
      due_date: newTask.value.due_date || null,
      completed: false,
      created_at: null,
      updated_at: null,
    })
  }

  newTask.value = { title: '', priority: 'medium', due_date: '' }
  showAddForm.value = false
}

const toggleComplete = async (task: Task) => {
  if (isTauri && api && task.id) {
    try {
      const updated = await api.toggleTask(task.id)
      const idx = tasks.value.findIndex(t => t.id === task.id)
      if (idx !== -1) tasks.value[idx] = updated
      if (updated.completed) notifyTaskCompleted(updated.title)
    } catch (e) {
      console.error('切换状态失败', e)
    }
  } else {
    task.completed = !task.completed
    if (task.completed) notifyTaskCompleted(task.title)
  }
}

const removeTask = async (task: Task) => {
  if (isTauri && api && task.id) {
    try {
      await api.deleteTask(task.id)
    } catch (e) {
      console.error('删除失败', e)
      return
    }
  }
  tasks.value = tasks.value.filter(t => t.id !== task.id)
}

const startEdit = (task: Task) => {
  editingId.value = task.id
  editForm.value = {
    title: task.title,
    priority: task.priority,
    due_date: task.due_date || '',
  }
  nextTick(() => { editInput.value?.focus() })
}

const saveEdit = async () => {
  if (!editingId.value || !editForm.value.title.trim()) return

  if (isTauri && api) {
    try {
      const updated = await api.updateTask({
        id: editingId.value,
        title: editForm.value.title,
        priority: editForm.value.priority,
        due_date: editForm.value.due_date || undefined,
      })
      const idx = tasks.value.findIndex(t => t.id === editingId.value)
      if (idx !== -1) tasks.value[idx] = updated
    } catch (e) {
      console.error('更新失败', e)
    }
  } else {
    const task = tasks.value.find(t => t.id === editingId.value)
    if (task) {
      task.title = editForm.value.title
      task.priority = editForm.value.priority
      task.due_date = editForm.value.due_date || null
    }
  }
  editingId.value = null
}

const cancelEdit = () => { editingId.value = null }
const cancelAdd = () => { showAddForm.value = false; newTask.value = { title: '', priority: 'medium', due_date: '' } }

const priorityLabel = (p: string) => ({ high: '高', medium: '中', low: '低' }[p] || '中')

const formatDate = (dateStr: string) => {
  if (dateStr === todayStr.value) return '今天'
  const d = new Date(dateStr)
  const tomorrow = new Date(); tomorrow.setDate(tomorrow.getDate() + 1)
  const tomorrowStr = `${tomorrow.getFullYear()}-${String(tomorrow.getMonth() + 1).padStart(2, '0')}-${String(tomorrow.getDate()).padStart(2, '0')}`
  if (dateStr === tomorrowStr) return '明天'
  return `${d.getMonth() + 1}/${d.getDate()}`
}

const isOverdue = (dateStr: string) => dateStr < todayStr.value

// === 导入任务 ===
const importTasks = async () => {
  importError.value = ''
  importSuccess.value = ''
  try {
    const parsed = JSON.parse(importJson.value)
    if (!Array.isArray(parsed)) { importError.value = '格式错误：需要一个 JSON 数组'; return }

    if (isTauri && api) {
      const imported = await api.importTasksJson(importJson.value)
      tasks.value.unshift(...imported)
      showToast(`成功导入 ${imported.length} 个任务`)
    } else {
      // mock 模式
      const mockTasks = parsed.map((item: any, i: number) => ({
        id: Date.now() + i,
        title: item.title || '未命名任务',
        priority: item.priority || 'medium',
        due_date: item.due_date || null,
        completed: false,
        created_at: null,
        updated_at: null,
      }))
      tasks.value.unshift(...mockTasks)
      showToast(`成功导入 ${mockTasks.length} 个任务`)
    }
    importJson.value = ''
    showImportDialog.value = false
  } catch (e: any) {
    importError.value = 'JSON 解析失败：' + (e.message || '格式不正确')
  }
}

// === Toast ===
const showToast = (msg: string) => {
  toastMsg.value = msg
  setTimeout(() => { toastMsg.value = '' }, 2500)
}

onMounted(loadTasks)
</script>

<style scoped>
.task-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 40px 48px;
  overflow-y: auto;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
  flex-shrink: 0;
}

.panel-title {
  font-size: 1.6rem;
  font-weight: 600;
  letter-spacing: -0.02em;
}

.panel-subtitle {
  font-size: 0.85rem;
  margin-top: 4px;
}

/* 新建表单 */
.add-task-form {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  padding: 16px;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.task-input {
  width: 100%;
  background: transparent;
  border: none;
  font-size: 1rem;
  color: var(--text-primary);
  outline: none;
  padding: 4px 0;
  margin-bottom: 10px;
  border-bottom: 1px solid var(--border-subtle);
}
.task-input:focus { border-bottom-color: var(--accent-primary); }

.form-row {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}
.form-row.compact { margin-top: 8px; }

.task-select {
  background: var(--bg-base);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  padding: 5px 8px;
  font-size: 0.85rem;
  cursor: pointer;
}
.task-select:focus { outline: none; border-color: var(--accent-primary); }

.task-date-input {
  background: var(--bg-base);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  padding: 5px 8px;
  font-size: 0.85rem;
}
.task-date-input:focus { outline: none; border-color: var(--accent-primary); }

.form-actions { display: flex; gap: 6px; margin-left: auto; }

.btn-sm {
  padding: 5px 14px;
  font-size: 0.85rem;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  gap: 5px;
}

.btn-ghost {
  color: var(--text-secondary);
  background: transparent;
  border: 1px solid var(--border-subtle);
}
.btn-ghost:hover { background: var(--bg-surface-hover); }

/* 筛选标签 */
.filter-tabs {
  display: flex;
  gap: 4px;
  margin-bottom: 16px;
  flex-shrink: 0;
}

.filter-tab {
  padding: 6px 14px;
  font-size: 0.85rem;
  border-radius: 20px;
  color: var(--text-secondary);
  background: transparent;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
}
.filter-tab:hover { background: var(--bg-surface-hover); color: var(--text-primary); }
.filter-tab.active {
  background: var(--text-primary);
  color: var(--bg-base);
}

.tab-count {
  font-size: 0.75rem;
  background: rgba(128, 128, 128, 0.2);
  padding: 1px 7px;
  border-radius: 10px;
}
.filter-tab.active .tab-count {
  background: rgba(255, 255, 255, 0.2);
}

/* 任务列表 */
.task-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.task-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px 12px;
  border-radius: var(--radius-md);
  transition: background 0.15s;
  position: relative;
  border-left: 3px solid transparent;
}
.task-item:hover { background: var(--bg-surface-hover); }
.task-item.priority-high { border-left-color: #ef4444; }
.task-item.priority-medium { border-left-color: #f59e0b; }
.task-item.is-completed { opacity: 0.5; }
.task-item.is-completed .task-title { text-decoration: line-through; }

.task-checkbox {
  flex-shrink: 0;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 2px;
  transition: color 0.2s;
  margin-top: 1px;
}
.task-checkbox:hover { color: var(--accent-primary); }
.task-item.is-completed .task-checkbox { color: var(--accent-success); }

.task-body {
  flex: 1;
  min-width: 0;
  cursor: default;
}

.task-title {
  font-size: 0.95rem;
  color: var(--text-primary);
  line-height: 1.5;
}

.task-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
}

.priority-badge {
  font-size: 0.7rem;
  padding: 1px 8px;
  border-radius: 10px;
  font-weight: 500;
}
.p-high { background: rgba(239, 68, 68, 0.12); color: #ef4444; }
.p-medium { background: rgba(245, 158, 11, 0.12); color: #d97706; }
.p-low { background: rgba(107, 114, 128, 0.12); color: var(--text-tertiary); }

.due-date {
  font-size: 0.75rem;
  color: var(--text-tertiary);
}
.due-date.overdue { color: #ef4444; font-weight: 500; }

.task-edit { flex: 1; }

.task-delete {
  opacity: 0;
  color: var(--text-tertiary);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
  flex-shrink: 0;
}
.task-item:hover .task-delete { opacity: 1; }
.task-delete:hover { color: #ef4444; background: rgba(239, 68, 68, 0.1); }

/* 空状态 */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 40px;
}
.empty-text {
  font-size: 0.95rem;
  color: var(--text-tertiary);
}

.header-actions { display: flex; gap: 8px; align-items: center; }

/* 导入对话框 */
.import-overlay {
  position: fixed; inset: 0; z-index: 50;
  background: rgba(0,0,0,0.25); display: flex;
  align-items: center; justify-content: center;
}
.import-dialog {
  background: var(--bg-base); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg); padding: 24px;
  width: 480px; max-width: 90vw; box-shadow: var(--shadow-lg);
}
.import-title { font-size: 1.1rem; font-weight: 600; margin-bottom: 8px; }
.import-desc { font-size: 0.85rem; color: var(--text-tertiary); margin-bottom: 12px; }
.import-textarea {
  width: 100%; height: 160px; background: var(--bg-surface);
  border: 1px solid var(--border-subtle); border-radius: var(--radius-md);
  color: var(--text-primary); padding: 12px; font-size: 0.85rem;
  font-family: 'Menlo', monospace; resize: vertical;
}
.import-textarea:focus { outline: none; border-color: var(--accent-primary); }
.import-actions {
  display: flex; justify-content: flex-end; align-items: center;
  gap: 8px; margin-top: 12px;
}
.import-hint { font-size: 0.8rem; color: #ef4444; margin-right: auto; }
.import-success { color: #10b981; }

/* Toast */
.toast {
  position: fixed; bottom: 32px; left: 50%; transform: translateX(-50%);
  background: var(--text-primary); color: var(--bg-base);
  padding: 10px 24px; border-radius: 20px;
  font-size: 0.9rem; font-weight: 500;
  box-shadow: 0 4px 16px rgba(0,0,0,0.15);
  z-index: 200;
}
.toast-enter-active { transition: all 0.3s ease; }
.toast-leave-active { transition: all 0.3s ease; }
.toast-enter-from { opacity: 0; transform: translateX(-50%) translateY(20px); }
.toast-leave-to { opacity: 0; transform: translateX(-50%) translateY(20px); }
</style>
