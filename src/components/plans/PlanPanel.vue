<template>
  <div class="plan-panel">
    <!-- 详情视图 -->
    <PlanDetail
      v-if="selectedPlanId"
      :planId="selectedPlanId"
      @back="goBackToList"
      @startWriting="(task: any) => emit('startWriting', task)"
    />

    <!-- 列表视图 -->
    <template v-else>
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
      <div class="plan-card" v-for="plan in plans" :key="plan.id ?? 0" :class="'status-' + plan.status" @click="selectPlan(plan)">
        <div class="plan-card-header">
          <h3 class="plan-name">{{ plan.name }}</h3>
          <div class="plan-header-right">
            <span class="plan-status-badge" :class="plan.status">{{ statusLabel(plan.status) }}</span>
            <!-- 操作菜单 -->
            <div class="plan-actions">
              <button class="action-btn" @click.stop="openEdit(plan)" title="编辑">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
              </button>
              <button class="action-btn action-btn-danger" @click.stop="confirmDelete(plan)" title="删除">
                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
              </button>
            </div>
          </div>
        </div>
        <p class="plan-theme text-tertiary" v-if="plan.theme">{{ plan.theme }}</p>
        <div class="plan-progress">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: progressPercent(plan) + '%' }"></div>
          </div>
          <span class="progress-text">{{ plan.completed_days }}/{{ plan.total_days }} 天</span>
        </div>
        <div class="plan-meta">
          <span class="plan-date">{{ formatDate(plan.start_date) }} 开始</span>
          <!-- 状态切换按钮 -->
          <div class="status-actions">
            <button v-if="plan.status === 'active'" class="status-action-btn pause" @click.stop="changePlanStatus(plan, 'paused')" title="暂停">
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><rect x="6" y="4" width="4" height="16"></rect><rect x="14" y="4" width="4" height="16"></rect></svg>
              暂停
            </button>
            <button v-if="plan.status === 'paused'" class="status-action-btn resume" @click.stop="changePlanStatus(plan, 'active')" title="恢复">
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg>
              恢复
            </button>
            <button v-if="plan.status !== 'completed'" class="status-action-btn complete" @click.stop="changePlanStatus(plan, 'completed')" title="标记完成">
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
              完成
            </button>
            <button v-if="plan.status === 'completed'" class="status-action-btn resume" @click.stop="changePlanStatus(plan, 'active')" title="重新开始">
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="1 4 1 10 7 10"></polyline><path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path></svg>
              重启
            </button>
          </div>
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

    <!-- 编辑对话框 -->
    <div class="import-overlay" v-if="showEditDialog" @click.self="showEditDialog = false">
      <div class="import-dialog" style="width:420px">
        <h3 class="import-title">编辑计划</h3>
        <div class="edit-form">
          <div class="form-group">
            <label>计划名称</label>
            <input v-model="editForm.name" class="edit-input" placeholder="计划名称" />
          </div>
          <div class="form-group">
            <label>主题</label>
            <input v-model="editForm.theme" class="edit-input" placeholder="写作主题（可选）" />
          </div>
          <div class="form-group">
            <label>开始日期</label>
            <input v-model="editForm.start_date" type="date" class="edit-input" />
          </div>
        </div>
        <div class="import-actions">
          <button class="btn-ghost btn-sm" @click="showEditDialog = false">取消</button>
          <button class="btn-primary btn-sm" @click="doEdit">保存</button>
        </div>
      </div>
    </div>

    <!-- 删除确认对话框 -->
    <div class="import-overlay" v-if="showDeleteConfirm" @click.self="showDeleteConfirm = false">
      <div class="import-dialog" style="width:380px">
        <h3 class="import-title" style="color:#ef4444">确认删除</h3>
        <p style="font-size:0.9rem;color:var(--text-secondary);margin-bottom:16px">
          确定要删除计划「{{ deletingPlan?.name }}」吗？<br>
          <span style="font-size:0.8rem;color:var(--text-tertiary)">关联的每日条目也会被删除，此操作不可撤销。</span>
        </p>
        <div class="import-actions">
          <button class="btn-ghost btn-sm" @click="showDeleteConfirm = false">取消</button>
          <button class="btn-danger btn-sm" @click="doDelete">删除</button>
        </div>
      </div>
    </div>

    <!-- Toast -->
    <transition name="toast">
      <div class="toast" v-if="toastMsg">{{ toastMsg }}</div>
    </transition>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { WritingPlan } from '../../types'
import PlanDetail from './PlanDetail.vue'

const emit = defineEmits(['startWriting'])

let api: any = null
import { isTauri } from '../../utils/env'

const plans = ref<WritingPlan[]>([])
const showImport = ref(false)
const importFormat = ref<'json' | 'markdown'>('json')
const importContent = ref('')
const startDate = ref('')
const importError = ref('')
const toastMsg = ref('')

// 详情页导航
const selectedPlanId = ref<number | null>(null)

const selectPlan = (plan: WritingPlan) => {
  if (plan.id) selectedPlanId.value = plan.id
}
const goBackToList = async () => {
  selectedPlanId.value = null
  await loadPlans()
}

// 编辑
const showEditDialog = ref(false)
const editForm = ref({ id: 0, name: '', theme: '', start_date: '' })

// 删除
const showDeleteConfirm = ref(false)
const deletingPlan = ref<WritingPlan | null>(null)

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

// ======= 编辑 =======
const openEdit = (plan: WritingPlan) => {
  editForm.value = {
    id: plan.id!,
    name: plan.name,
    theme: plan.theme || '',
    start_date: plan.start_date
  }
  showEditDialog.value = true
}

const doEdit = async () => {
  try {
    if (isTauri) {
      if (!api) api = await import('../../api')
      await api.updatePlan({
        id: editForm.value.id,
        name: editForm.value.name || undefined,
        theme: editForm.value.theme || undefined,
        start_date: editForm.value.start_date || undefined,
      })
      showToast('计划已更新')
      await loadPlans()
    } else {
      const idx = plans.value.findIndex(p => p.id === editForm.value.id)
      if (idx >= 0) {
        plans.value[idx].name = editForm.value.name
        plans.value[idx].theme = editForm.value.theme || null
        plans.value[idx].start_date = editForm.value.start_date
      }
      showToast('计划已更新')
    }
    showEditDialog.value = false
  } catch (e: any) {
    showToast('更新失败：' + (e.message || e))
  }
}

// ======= 删除 =======
const confirmDelete = (plan: WritingPlan) => {
  deletingPlan.value = plan
  showDeleteConfirm.value = true
}

const doDelete = async () => {
  if (!deletingPlan.value) return
  try {
    if (isTauri) {
      if (!api) api = await import('../../api')
      await api.deletePlan(deletingPlan.value.id!)
      showToast('计划已删除')
      await loadPlans()
    } else {
      plans.value = plans.value.filter(p => p.id !== deletingPlan.value?.id)
      showToast('计划已删除')
    }
    showDeleteConfirm.value = false
    deletingPlan.value = null
  } catch (e: any) {
    showToast('删除失败：' + (e.message || e))
  }
}

// ======= 状态切换 =======
const changePlanStatus = async (plan: WritingPlan, status: string) => {
  try {
    if (isTauri) {
      if (!api) api = await import('../../api')
      await api.updatePlanStatus(plan.id!, status)
      await loadPlans()
      showToast(`计划已${statusLabel(status)}`)
    } else {
      plan.status = status as any
      showToast(`计划已${statusLabel(status)}`)
    }
  } catch (e: any) {
    showToast('状态更新失败：' + (e.message || e))
  }
}

const showToast = (msg: string) => {
  toastMsg.value = msg
  setTimeout(() => { toastMsg.value = '' }, 2500)
}

const statusLabel = (s: string) => ({ active: '进行中', paused: '已暂停', completed: '已完成' }[s] || s)
const progressPercent = (plan: WritingPlan) => plan.total_days > 0 ? Math.round(plan.completed_days / plan.total_days * 100) : 0
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
  cursor: pointer;
}
.plan-card:hover { transform: translateY(-2px); box-shadow: var(--shadow-md); }
.plan-card.status-completed { border-left-color: #10b981; opacity: 0.8; }
.plan-card.status-paused { border-left-color: #f59e0b; }

.plan-card-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px; }
.plan-header-right { display: flex; align-items: center; gap: 8px; }
.plan-name { font-size: 1.05rem; font-weight: 600; }
.plan-status-badge {
  font-size: 0.7rem; padding: 2px 10px; border-radius: 10px; font-weight: 500;
}
.plan-status-badge.active { background: rgba(59, 130, 246, 0.12); color: #3b82f6; }
.plan-status-badge.completed { background: rgba(16, 185, 129, 0.12); color: #10b981; }
.plan-status-badge.paused { background: rgba(245, 158, 11, 0.12); color: #d97706; }

/* 操作按钮 */
.plan-actions {
  display: flex; gap: 4px; opacity: 0; transition: opacity 0.2s;
}
.plan-card:hover .plan-actions { opacity: 1; }
.action-btn {
  width: 28px; height: 28px; display: flex; align-items: center; justify-content: center;
  border-radius: 6px; background: transparent; border: none;
  color: var(--text-tertiary); cursor: pointer; transition: all 0.2s;
}
.action-btn:hover { background: var(--bg-surface-hover); color: var(--text-primary); }
.action-btn-danger:hover { background: rgba(239, 68, 68, 0.1); color: #ef4444; }

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
.plan-meta { display: flex; justify-content: space-between; align-items: center; }
.plan-date { font-size: 0.8rem; color: var(--text-tertiary); }

/* 状态操作按钮 */
.status-actions { display: flex; gap: 6px; opacity: 0; transition: opacity 0.2s; }
.plan-card:hover .status-actions { opacity: 1; }
.status-action-btn {
  display: flex; align-items: center; gap: 3px;
  font-size: 0.72rem; padding: 3px 8px; border-radius: 6px;
  border: 1px solid var(--border-subtle); background: transparent;
  color: var(--text-tertiary); cursor: pointer; transition: all 0.2s;
}
.status-action-btn.pause:hover { background: rgba(245, 158, 11, 0.1); color: #d97706; border-color: #d97706; }
.status-action-btn.resume:hover { background: rgba(59, 130, 246, 0.1); color: #3b82f6; border-color: #3b82f6; }
.status-action-btn.complete:hover { background: rgba(16, 185, 129, 0.1); color: #10b981; border-color: #10b981; }

/* 编辑表单 */
.edit-form { display: flex; flex-direction: column; gap: 14px; margin-bottom: 16px; }
.form-group { display: flex; flex-direction: column; gap: 4px; }
.form-group label { font-size: 0.8rem; color: var(--text-tertiary); font-weight: 500; }
.edit-input {
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); color: var(--text-primary);
  padding: 8px 12px; font-size: 0.9rem; transition: border-color 0.2s;
}
.edit-input:focus { outline: none; border-color: var(--accent-primary); }

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
.btn-danger {
  background: #ef4444; color: white; border: none; cursor: pointer;
  transition: background 0.2s;
}
.btn-danger:hover { background: #dc2626; }

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
