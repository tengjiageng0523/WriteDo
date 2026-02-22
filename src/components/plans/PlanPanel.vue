<template>
  <div class="plan-panel">
    <!-- 顶部标题栏 -->
    <header class="panel-header">
      <div>
        <h1 class="panel-title">写作计划</h1>
        <p class="panel-subtitle text-tertiary">{{ plans.length }} 个计划</p>
      </div>
      <div class="header-actions">
        <button class="btn-ghost btn-sm" @click="showImport = true">
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
          导入
        </button>
      </div>
    </header>

    <!-- 计划列表 -->
    <div class="plan-list" v-if="plans.length">
      <div class="plan-card" v-for="plan in plans" :key="plan.id" :class="'status-' + plan.status">
        <div class="plan-card-header">
          <h3 class="plan-name">{{ plan.name }}</h3>
          <span class="plan-status-badge" :class="plan.status">{{ statusLabel(plan.status) }}</span>
        </div>
        <p class="plan-theme text-tertiary" v-if="plan.theme">{{ plan.theme }}</p>
        <div class="plan-progress">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: progressPercent(plan) + '%' }"></div>
          </div>
          <span class="progress-text">{{ plan.total_days }} 天计划</span>
        </div>
        <div class="plan-meta">
          <span class="plan-date">{{ formatDate(plan.start_date) }} 开始</span>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div class="empty-state" v-else>
      <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" style="opacity:0.3"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect><line x1="16" y1="2" x2="16" y2="6"></line><line x1="8" y1="2" x2="8" y2="6"></line><line x1="3" y1="10" x2="21" y2="10"></line></svg>
      <p class="empty-text">还没有写作计划</p>
      <button class="btn-primary btn-sm" @click="showImport = true">导入你的第一个计划</button>
    </div>

    <!-- 导入对话框 -->
    <div class="import-overlay" v-if="showImport" @click.self="showImport = false">
      <div class="import-dialog">
        <h3 class="import-title">导入写作计划</h3>

        <!-- 格式切换 -->
        <div class="format-tabs">
          <button class="format-tab" :class="{ active: importFormat === 'json' }" @click="importFormat = 'json'">JSON</button>
          <button class="format-tab" :class="{ active: importFormat === 'markdown' }" @click="importFormat = 'markdown'">Markdown</button>
        </div>

        <!-- JSON 导入 -->
        <div v-if="importFormat === 'json'">
          <textarea v-model="importContent" class="import-textarea" :placeholder='jsonPlaceholder'></textarea>
        </div>

        <!-- Markdown 导入 -->
        <div v-else>
          <textarea v-model="importContent" class="import-textarea" :placeholder='mdPlaceholder'></textarea>
          <div class="form-row" style="margin-top:8px">
            <label class="text-tertiary" style="font-size:0.85rem">开始日期：</label>
            <input v-model="startDate" type="date" class="task-date-input" />
          </div>
        </div>

        <div class="import-actions">
          <span class="import-hint error" v-if="importError">{{ importError }}</span>
          <button class="btn-ghost btn-sm" @click="closeImport">取消</button>
          <button class="btn-primary btn-sm" @click="doImport" :disabled="!importContent.trim()">导入</button>
        </div>
      </div>
    </div>

    <!-- Toast -->
    <transition name="toast">
      <div class="toast" v-if="toastMsg">{{ toastMsg }}</div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { WritingPlan } from '../../types'

let api: any = null
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window

const plans = ref<WritingPlan[]>([])
const showImport = ref(false)
const importFormat = ref<'json' | 'markdown'>('json')
const importContent = ref('')
const startDate = ref('')
const importError = ref('')
const toastMsg = ref('')

// 设置默认开始日期为今天
const today = new Date()
startDate.value = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`

const jsonPlaceholder = `{
  "name": "30天叙事写作训练",
  "theme": "记叙与表达",
  "start_date": "${startDate.value}",
  "days": [
    { "day": 1, "title": "童年记忆", "prompt": "写一篇关于童年的回忆..." },
    { "day": 2, "title": "一场雨", "prompt": "以雨天为背景..." }
  ]
}`

const mdPlaceholder = `# 30天叙事写作训练

## Day 1 - 童年记忆
写一篇关于童年的回忆，细致描绘你记忆中的场景。

## Day 2 - 一场雨中的故事
以雨天为背景，写一个真实的故事。`

const loadPlans = async () => {
  if (isTauri) {
    try {
      if (!api) api = await import('../../api')
      plans.value = await api.getPlans()
    } catch (e) {
      console.warn('加载计划失败', e)
    }
  }
  if (!plans.value.length && !isTauri) {
    plans.value = [
      { id: 1, name: '30天叙事写作训练', theme: '记叙与表达', start_date: startDate.value, total_days: 30, status: 'active', created_at: null },
      { id: 2, name: '21天散文入门', theme: '散文基础', start_date: '2026-01-15', total_days: 21, status: 'completed', created_at: null },
    ]
  }
}

const doImport = async () => {
  importError.value = ''
  try {
    if (importFormat.value === 'json') {
      if (isTauri && api) {
        const id = await api.importPlanJson(importContent.value)
        showToast(`计划导入成功 (ID: ${id})`)
        await loadPlans()
      } else {
        const parsed = JSON.parse(importContent.value)
        plans.value.push({
          id: Date.now(), name: parsed.name || '导入计划', theme: parsed.theme || null,
          start_date: parsed.start_date || startDate.value,
          total_days: parsed.days?.length || 0, status: 'active', created_at: null,
        })
        showToast('计划导入成功')
      }
    } else {
      if (isTauri && api) {
        const id = await api.importPlanMarkdown(importContent.value, startDate.value)
        showToast(`计划导入成功 (ID: ${id})`)
        await loadPlans()
      } else {
        const lines = importContent.value.split('\n')
        const name = lines.find(l => l.startsWith('# '))?.replace('# ', '') || '导入计划'
        const dayCount = lines.filter(l => l.startsWith('## ')).length
        plans.value.push({
          id: Date.now(), name, theme: null,
          start_date: startDate.value, total_days: dayCount || 1,
          status: 'active', created_at: null,
        })
        showToast('计划导入成功')
      }
    }
    closeImport()
  } catch (e: any) {
    importError.value = importFormat.value === 'json'
      ? 'JSON 格式错误：' + (e.message || '')
      : '导入失败：' + (e.message || '')
  }
}

const closeImport = () => {
  showImport.value = false
  importContent.value = ''
  importError.value = ''
}

const showToast = (msg: string) => {
  toastMsg.value = msg
  setTimeout(() => { toastMsg.value = '' }, 2500)
}

const statusLabel = (s: string) => ({ active: '进行中', paused: '已暂停', completed: '已完成' }[s] || s)
const progressPercent = (plan: WritingPlan) => plan.status === 'completed' ? 100 : Math.min(50, Math.random() * 60)
const formatDate = (d: string) => { const p = d.split('-'); return `${p[1]}/${p[2]}` }

onMounted(loadPlans)
</script>

<style scoped>
.plan-panel {
  height: 100%; display: flex; flex-direction: column;
  padding: 40px 48px; overflow-y: auto;
}
.panel-header {
  display: flex; justify-content: space-between; align-items: flex-start;
  margin-bottom: 24px; flex-shrink: 0;
}
.panel-title { font-size: 1.6rem; font-weight: 600; letter-spacing: -0.02em; }
.panel-subtitle { font-size: 0.85rem; margin-top: 4px; }
.header-actions { display: flex; gap: 8px; }

/* 计划卡片 */
.plan-list { display: flex; flex-direction: column; gap: 12px; }
.plan-card {
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg); padding: 20px;
  border-left: 4px solid var(--accent-primary); transition: transform 0.2s, box-shadow 0.2s;
}
.plan-card:hover { transform: translateY(-2px); box-shadow: var(--shadow-md); }
.plan-card.status-completed { border-left-color: #10b981; }
.plan-card.status-paused { border-left-color: #f59e0b; }

.plan-card-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px; }
.plan-name { font-size: 1.05rem; font-weight: 600; }
.plan-status-badge {
  font-size: 0.7rem; padding: 2px 10px; border-radius: 10px; font-weight: 500;
}
.plan-status-badge.active { background: rgba(59, 130, 246, 0.12); color: #3b82f6; }
.plan-status-badge.completed { background: rgba(16, 185, 129, 0.12); color: #10b981; }
.plan-status-badge.paused { background: rgba(245, 158, 11, 0.12); color: #d97706; }

.plan-theme { font-size: 0.85rem; margin-bottom: 12px; }
.plan-progress { display: flex; align-items: center; gap: 12px; margin-bottom: 8px; }
.progress-bar {
  flex: 1; height: 6px; background: var(--border-subtle); border-radius: 3px; overflow: hidden;
}
.progress-fill {
  height: 100%; background: var(--accent-primary); border-radius: 3px;
  transition: width 0.5s ease;
}
.plan-card.status-completed .progress-fill { background: #10b981; }
.progress-text { font-size: 0.8rem; color: var(--text-tertiary); white-space: nowrap; }
.plan-meta { display: flex; gap: 12px; }
.plan-date { font-size: 0.8rem; color: var(--text-tertiary); }

/* 空状态 */
.empty-state {
  flex: 1; display: flex; flex-direction: column; align-items: center;
  justify-content: center; gap: 12px; padding: 40px;
}
.empty-text { font-size: 0.95rem; color: var(--text-tertiary); }

/* 导入对话框 */
.import-overlay {
  position: fixed; inset: 0; z-index: 50;
  background: rgba(0,0,0,0.25); display: flex;
  align-items: center; justify-content: center;
}
.import-dialog {
  background: var(--bg-base); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg); padding: 24px;
  width: 520px; max-width: 90vw; box-shadow: var(--shadow-lg);
}
.import-title { font-size: 1.1rem; font-weight: 600; margin-bottom: 12px; }

.format-tabs { display: flex; gap: 4px; margin-bottom: 12px; }
.format-tab {
  padding: 6px 16px; font-size: 0.85rem; border-radius: 6px;
  color: var(--text-secondary); background: transparent;
  border: 1px solid var(--border-subtle); cursor: pointer; transition: all 0.2s;
}
.format-tab.active {
  background: var(--text-primary); color: var(--bg-base); border-color: var(--text-primary);
}

.import-textarea {
  width: 100%; height: 200px; background: var(--bg-surface);
  border: 1px solid var(--border-subtle); border-radius: var(--radius-md);
  color: var(--text-primary); padding: 12px; font-size: 0.85rem;
  font-family: 'Menlo', monospace; resize: vertical;
}
.import-textarea:focus { outline: none; border-color: var(--accent-primary); }

.form-row { display: flex; align-items: center; gap: 8px; }
.task-date-input {
  background: var(--bg-base); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); color: var(--text-primary);
  padding: 5px 8px; font-size: 0.85rem;
}

.import-actions {
  display: flex; justify-content: flex-end; align-items: center;
  gap: 8px; margin-top: 12px;
}
.import-hint.error { font-size: 0.8rem; color: #ef4444; margin-right: auto; }

.btn-sm {
  padding: 5px 14px; font-size: 0.85rem; border-radius: var(--radius-md);
  display: flex; align-items: center; gap: 5px;
}
.btn-ghost {
  color: var(--text-secondary); background: transparent; border: 1px solid var(--border-subtle);
}
.btn-ghost:hover { background: var(--bg-surface-hover); }

/* Toast */
.toast {
  position: fixed; bottom: 32px; left: 50%; transform: translateX(-50%);
  background: var(--text-primary); color: var(--bg-base);
  padding: 10px 24px; border-radius: 20px;
  font-size: 0.9rem; font-weight: 500;
  box-shadow: 0 4px 16px rgba(0,0,0,0.15); z-index: 200;
}
.toast-enter-active, .toast-leave-active { transition: all 0.3s ease; }
.toast-enter-from, .toast-leave-to { opacity: 0; transform: translateX(-50%) translateY(20px); }
</style>
