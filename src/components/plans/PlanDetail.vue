<template>
  <div class="plan-detail">
    <!-- 顶部导航 -->
    <header class="detail-header">
      <button class="back-btn" @click="$emit('back')">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 18 9 12 15 6"></polyline></svg>
        返回
      </button>
      <div class="detail-header-info">
        <h1 class="detail-title">{{ plan?.name }}</h1>
        <p class="detail-sub text-tertiary">
          {{ plan?.total_days }} 天计划 · 已完成 {{ completedDays }}/{{ days.length }} 天
          <span class="plan-status-badge" :class="plan?.status">{{ statusLabel(plan?.status || '') }}</span>
        </p>
      </div>
    </header>

    <!-- 进度条 -->
    <div class="progress-section">
      <div class="progress-bar-lg">
        <div class="progress-fill-lg" :style="{ width: progressPercent + '%' }"></div>
      </div>
      <span class="progress-label">{{ progressPercent }}%</span>
    </div>

    <!-- 每日条目列表 -->
    <div class="day-list">
      <template v-for="(day, idx) in days" :key="day.id">
        <!-- 插入按钮（在每条前面） -->
        <button class="insert-btn" @click="openAddDay(idx + 1)" title="在此处插入新条目">
          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
        </button>

        <!-- 条目卡片 -->
        <div
          class="day-card"
          :class="{ completed: day.is_completed, today: isToday(day.scheduled_date) }"
        >
          <div class="day-left">
            <div class="day-number" :class="{ done: day.is_completed }">
              <svg v-if="day.is_completed" xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
              <span v-else>{{ idx + 1 }}</span>
            </div>
          </div>

          <div class="day-content">
            <div class="day-top-row">
              <h4 class="day-title">{{ day.title }}</h4>
              <div class="day-actions">
                <button class="day-action-btn" @click="openEditDay(day)" title="编辑">
                  <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
                </button>
                <button class="day-action-btn danger" @click="confirmDeleteDay(day)" title="删除">
                  <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                </button>
              </div>
            </div>
            <p class="day-prompt text-tertiary">{{ day.prompt }}</p>
            <div class="day-meta">
              <span class="day-date" v-if="day.scheduled_date">{{ formatDate(day.scheduled_date) }}</span>
              <span class="day-writing-info" v-if="day.is_completed">
                <svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19l7-7 3 3-7 7-3-3z"></path><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"></path></svg>
                {{ day.writing_title || '已完成写作' }} · {{ day.word_count }} 字
              </span>
              <span class="day-status-pending" v-else-if="isToday(day.scheduled_date)">今日待完成</span>
              <button v-if="isToday(day.scheduled_date) && !day.is_completed" class="start-writing-btn" @click="$emit('startWriting', { plan_name: plan?.name, plan_id: props.planId, day_number: day.day_number, title: day.title, prompt: day.prompt, day_id: day.id })">
                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"/></svg>
                开始写作
              </button>
            </div>
          </div>
        </div>
      </template>

      <!-- 底部添加按钮 -->
      <button class="add-day-btn" @click="openAddDay(days.length + 1)">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
        添加新条目
      </button>
    </div>

    <!-- 空状态 -->
    <div class="empty-days" v-if="!days.length && !loading">
      <p class="text-tertiary">暂无每日条目</p>
      <button class="add-day-btn" @click="openAddDay(1)" style="margin-top:12px">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
        添加第一个条目
      </button>
    </div>

    <!-- 编辑条目对话框 -->
    <div class="overlay" v-if="showEditDay" @click.self="showEditDay = false">
      <div class="edit-dialog">
        <h3 class="dialog-title">编辑第 {{ editDayForm.day_number }} 天</h3>
        <div class="edit-form">
          <div class="form-group">
            <label>标题</label>
            <input v-model="editDayForm.title" class="form-input" />
          </div>
          <div class="form-group">
            <label>写作提示</label>
            <textarea v-model="editDayForm.prompt" class="form-textarea" rows="4"></textarea>
          </div>
        </div>
        <div class="dialog-actions">
          <button class="btn-ghost btn-sm" @click="showEditDay = false">取消</button>
          <button class="btn-primary btn-sm" @click="doEditDay">保存</button>
        </div>
      </div>
    </div>

    <!-- 删除确认 -->
    <div class="overlay" v-if="showDeleteDay" @click.self="showDeleteDay = false">
      <div class="edit-dialog" style="width:360px">
        <h3 class="dialog-title" style="color:#ef4444">确认删除</h3>
        <p style="font-size:0.9rem;color:var(--text-secondary);margin-bottom:16px">
          确定要删除「{{ deletingDay?.title }}」吗？
        </p>
        <div class="dialog-actions">
          <button class="btn-ghost btn-sm" @click="showDeleteDay = false">取消</button>
          <button class="btn-danger btn-sm" @click="doDeleteDay">删除</button>
        </div>
      </div>
    </div>

    <!-- 添加条目对话框 -->
    <div class="overlay" v-if="showAddDay" @click.self="showAddDay = false">
      <div class="edit-dialog">
        <h3 class="dialog-title">添加新条目（第 {{ addDayForm.day_number }} 天）</h3>
        <div class="edit-form">
          <div class="form-group">
            <label>标题</label>
            <input v-model="addDayForm.title" class="form-input" placeholder="例如：童年记忆" />
          </div>
          <div class="form-group">
            <label>写作提示</label>
            <textarea v-model="addDayForm.prompt" class="form-textarea" rows="4" placeholder="写一篇关于..."></textarea>
          </div>
          <div class="form-group">
            <label>日期</label>
            <input v-model="addDayForm.scheduled_date" type="date" class="form-input" />
          </div>
        </div>
        <div class="dialog-actions">
          <button class="btn-ghost btn-sm" @click="showAddDay = false">取消</button>
          <button class="btn-primary btn-sm" @click="doAddDay" :disabled="!addDayForm.title.trim()">添加</button>
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
import { ref, computed, onMounted } from 'vue'

interface PlanDayDetail {
  id: number
  plan_id: number
  day_number: number
  title: string
  prompt: string
  scheduled_date: string | null
  is_completed: boolean
  word_count: number
  writing_title: string | null
  writing_id: number | null
}

interface PlanInfo {
  id: number
  name: string
  theme: string | null
  start_date: string
  total_days: number
  status: string
  created_at: string | null
  days: PlanDayDetail[]
  completed_days: number
}

const props = defineProps<{ planId: number }>()
defineEmits(['back', 'startWriting'])

let api: any = null
import { isTauri } from '../../utils/env'

const plan = ref<PlanInfo | null>(null)
const days = ref<PlanDayDetail[]>([])
const loading = ref(true)
const toastMsg = ref('')

// 编辑
const showEditDay = ref(false)
const editDayForm = ref({ id: 0, day_number: 0, title: '', prompt: '' })

// 删除
const showDeleteDay = ref(false)
const deletingDay = ref<PlanDayDetail | null>(null)

// 添加
const showAddDay = ref(false)
const addDayForm = ref({ day_number: 1, title: '', prompt: '', scheduled_date: '' })

const completedDays = computed(() => days.value.filter(d => d.is_completed).length)
const progressPercent = computed(() => {
  if (!days.value.length) return 0
  return Math.round(completedDays.value / days.value.length * 100)
})

const todayStr = (() => {
  const d = new Date()
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
})()

const isToday = (date: string | null) => date === todayStr

const loadDetail = async () => {
  loading.value = true
  try {
    if (isTauri) {
      if (!api) api = await import('../../api')
      const data = await api.getPlanDetail(props.planId)
      plan.value = data
      days.value = data.days || []
    }
  } catch (e) {
    console.warn('加载计划详情失败', e)
  } finally {
    loading.value = false
  }
}

// 编辑条目
const openEditDay = (day: PlanDayDetail) => {
  editDayForm.value = { id: day.id, day_number: day.day_number, title: day.title, prompt: day.prompt }
  showEditDay.value = true
}

const doEditDay = async () => {
  try {
    if (isTauri && api) {
      await api.updatePlanDay({
        id: editDayForm.value.id,
        title: editDayForm.value.title,
        prompt: editDayForm.value.prompt,
      })
      showToast('条目已更新')
      await loadDetail()
    }
    showEditDay.value = false
  } catch (e: any) {
    showToast('更新失败：' + (e.message || e))
  }
}

// 删除条目
const confirmDeleteDay = (day: PlanDayDetail) => {
  deletingDay.value = day
  showDeleteDay.value = true
}

const doDeleteDay = async () => {
  if (!deletingDay.value) return
  try {
    if (isTauri && api) {
      await api.deletePlanDay(deletingDay.value.id)
      showToast('条目已删除')
      await loadDetail()
    }
    showDeleteDay.value = false
    deletingDay.value = null
  } catch (e: any) {
    showToast('删除失败：' + (e.message || e))
  }
}

const showToast = (msg: string) => {
  toastMsg.value = msg
  setTimeout(() => { toastMsg.value = '' }, 2500)
}

const statusLabel = (s: string) => ({ active: '进行中', paused: '已暂停', completed: '已完成' }[s] || s)
const formatDate = (d: string) => {
  const p = d.split('-')
  return `${p[1]}/${p[2]}`
}

// 添加条目
const openAddDay = (dayNumber: number) => {
  const d = new Date()
  const defaultDate = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
  addDayForm.value = { day_number: dayNumber, title: '', prompt: '', scheduled_date: defaultDate }
  showAddDay.value = true
}

const doAddDay = async () => {
  try {
    if (isTauri) {
      if (!api) api = await import('../../api')
      await api.addPlanDay({
        plan_id: props.planId,
        day_number: addDayForm.value.day_number,
        title: addDayForm.value.title,
        prompt: addDayForm.value.prompt,
        scheduled_date: addDayForm.value.scheduled_date || undefined,
      })
      showToast('条目已添加')
      await loadDetail()
    }
    showAddDay.value = false
  } catch (e: any) {
    showToast('添加失败：' + (e.message || e))
  }
}

onMounted(loadDetail)
</script>

<style scoped>
.plan-detail {
  height: 100%; display: flex; flex-direction: column;
  padding: 32px 48px; overflow-y: auto;
}

.detail-header {
  display: flex; align-items: flex-start; gap: 12px; margin-bottom: 20px;
}
.back-btn {
  display: flex; align-items: center; gap: 4px;
  padding: 6px 10px; border-radius: 8px;
  background: transparent; border: 1px solid var(--border-subtle);
  color: var(--text-secondary); font-size: 0.85rem; cursor: pointer;
  transition: all 0.2s; flex-shrink: 0; margin-top: 2px;
}
.back-btn:hover { background: var(--bg-surface-hover); color: var(--text-primary); }

.detail-title { font-size: 1.4rem; font-weight: 600; letter-spacing: -0.02em; }
.detail-sub { font-size: 0.85rem; margin-top: 4px; display: flex; align-items: center; gap: 8px; }
.plan-status-badge {
  font-size: 0.65rem; padding: 2px 8px; border-radius: 10px; font-weight: 500;
}
.plan-status-badge.active { background: rgba(59, 130, 246, 0.12); color: #3b82f6; }
.plan-status-badge.completed { background: rgba(16, 185, 129, 0.12); color: #10b981; }
.plan-status-badge.paused { background: rgba(245, 158, 11, 0.12); color: #d97706; }

/* 进度条 */
.progress-section {
  display: flex; align-items: center; gap: 12px; margin-bottom: 24px;
}
.progress-bar-lg {
  flex: 1; height: 8px; background: var(--border-subtle); border-radius: 4px; overflow: hidden;
}
.progress-fill-lg {
  height: 100%; border-radius: 4px;
  background: linear-gradient(90deg, var(--accent-primary), #8b5cf6);
  transition: width 0.5s ease;
}
.progress-label {
  font-size: 0.85rem; font-weight: 600; color: var(--accent-primary); min-width: 40px;
}

/* 每日条目 */
.day-list { display: flex; flex-direction: column; gap: 8px; }
.day-card {
  display: flex; gap: 14px; padding: 16px;
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); transition: all 0.2s;
}
.day-card:hover { border-color: var(--accent-primary); box-shadow: 0 2px 8px rgba(0,0,0,0.06); }
.day-card.completed { opacity: 0.75; }
.day-card.today { border-left: 3px solid var(--accent-primary); }

.day-number {
  width: 32px; height: 32px; border-radius: 50%;
  display: flex; align-items: center; justify-content: center;
  font-size: 0.8rem; font-weight: 600;
  background: var(--bg-surface-hover); color: var(--text-secondary);
  flex-shrink: 0;
}
.day-number.done { background: rgba(16, 185, 129, 0.15); color: #10b981; }

.day-content { flex: 1; min-width: 0; }
.day-top-row { display: flex; justify-content: space-between; align-items: flex-start; }
.day-title { font-size: 0.95rem; font-weight: 600; margin-bottom: 4px; }
.day-prompt { font-size: 0.82rem; line-height: 1.5; margin-bottom: 6px; }
.day-meta { display: flex; gap: 12px; align-items: center; }
.day-date { font-size: 0.75rem; color: var(--text-tertiary); }
.day-writing-info {
  display: flex; align-items: center; gap: 4px;
  font-size: 0.75rem; color: #10b981;
}
.day-status-pending {
  font-size: 0.72rem; padding: 1px 8px; border-radius: 8px;
  background: rgba(59, 130, 246, 0.1); color: #3b82f6;
}

/* 开始写作按钮 */
.start-writing-btn {
  display: inline-flex; align-items: center; gap: 4px;
  padding: 3px 10px; border-radius: 8px; border: none;
  background: var(--accent-primary); color: white;
  font-size: 0.72rem; font-weight: 500; cursor: pointer;
  transition: all 0.2s;
}
.start-writing-btn:hover { opacity: 0.85; transform: scale(1.02); }

/* 操作按钮 */
.day-actions { display: flex; gap: 4px; opacity: 0; transition: opacity 0.2s; }
.day-card:hover .day-actions { opacity: 1; }
.day-action-btn {
  width: 24px; height: 24px; display: flex; align-items: center; justify-content: center;
  border-radius: 5px; background: transparent; border: none;
  color: var(--text-tertiary); cursor: pointer; transition: all 0.2s;
}
.day-action-btn:hover { background: var(--bg-surface-hover); color: var(--text-primary); }
.day-action-btn.danger:hover { background: rgba(239, 68, 68, 0.1); color: #ef4444; }

/* 空状态 */
.empty-days { text-align: center; padding: 40px; }

/* 插入按钮 */
.insert-btn {
  display: flex; align-items: center; justify-content: center;
  width: 100%; height: 0; padding: 0; border: none;
  background: transparent; cursor: pointer; position: relative;
  margin: -2px 0; z-index: 1; opacity: 0; transition: all 0.2s;
}
.insert-btn:hover {
  height: 28px; opacity: 1; margin: 2px 0;
}
.insert-btn:hover svg {
  background: var(--accent-primary); color: white;
  border-radius: 50%; padding: 3px; width: 18px; height: 18px;
}
.insert-btn::before {
  content: ''; position: absolute; left: 0; right: 0; top: 50%;
  height: 1px; background: var(--accent-primary); opacity: 0.3;
}

/* 底部添加按钮 */
.add-day-btn {
  display: flex; align-items: center; justify-content: center; gap: 6px;
  width: 100%; padding: 12px; margin-top: 4px;
  border: 1px dashed var(--border-subtle); border-radius: var(--radius-md);
  background: transparent; color: var(--text-tertiary);
  font-size: 0.85rem; cursor: pointer; transition: all 0.2s;
}
.add-day-btn:hover {
  border-color: var(--accent-primary); color: var(--accent-primary);
  background: rgba(59, 130, 246, 0.04);
}

/* 对话框 */
.overlay {
  position: fixed; inset: 0; z-index: 50;
  background: rgba(0,0,0,0.25); display: flex;
  align-items: center; justify-content: center;
}
.edit-dialog {
  background: var(--bg-base); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg); padding: 24px; width: 460px;
  max-width: 90vw; box-shadow: var(--shadow-lg);
}
.dialog-title { font-size: 1.05rem; font-weight: 600; margin-bottom: 16px; }
.dialog-actions { display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px; }

.edit-form { display: flex; flex-direction: column; gap: 12px; }
.form-group { display: flex; flex-direction: column; gap: 4px; }
.form-group label { font-size: 0.8rem; color: var(--text-tertiary); font-weight: 500; }
.form-input {
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); color: var(--text-primary);
  padding: 8px 12px; font-size: 0.9rem;
}
.form-input:focus, .form-textarea:focus { outline: none; border-color: var(--accent-primary); }
.form-textarea {
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); color: var(--text-primary);
  padding: 8px 12px; font-size: 0.85rem; resize: vertical;
  font-family: inherit;
}

.btn-sm {
  padding: 5px 14px; font-size: 0.85rem; border-radius: var(--radius-md);
  display: flex; align-items: center; gap: 5px; cursor: pointer;
}
.btn-ghost {
  color: var(--text-secondary); background: transparent; border: 1px solid var(--border-subtle);
}
.btn-ghost:hover { background: var(--bg-surface-hover); }
.btn-danger {
  background: #ef4444; color: white; border: none; transition: background 0.2s;
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
