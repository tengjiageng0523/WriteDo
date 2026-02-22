<template>
  <div class="stats-panel">
    <header class="panel-header">
      <div>
        <h1 class="panel-title">数据统计</h1>
        <p class="panel-subtitle text-tertiary">你的写作足迹</p>
      </div>
      <button class="btn-ghost btn-sm" @click="exportData">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
        导出数据
      </button>
    </header>

    <!-- 核心指标卡片 -->
    <div class="stats-cards">
      <div class="stat-card accent">
        <div class="stat-icon"><AppIcon name="flame" :size="24" color="#fff" /></div>
        <div class="stat-value">{{ stats.current_streak }}</div>
        <div class="stat-label">连续打卡</div>
        <div class="stat-unit">天</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon"><AppIcon name="pen" :size="24" /></div>
        <div class="stat-value">{{ stats.total_words.toLocaleString() }}</div>
        <div class="stat-label">总字数</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon"><AppIcon name="calendar" :size="24" /></div>
        <div class="stat-value">{{ stats.total_days }}</div>
        <div class="stat-label">写作天数</div>
      </div>
      <div class="stat-card">
        <div class="stat-icon"><AppIcon name="clock" :size="24" /></div>
        <div class="stat-value">{{ formatHours(stats.total_duration) }}</div>
        <div class="stat-label">总时长</div>
      </div>
    </div>

    <!-- 二级指标 -->
    <div class="stats-secondary">
      <div class="secondary-item">
        <span class="secondary-label">最长连续</span>
        <span class="secondary-value">{{ stats.max_streak }} 天</span>
      </div>
      <div class="secondary-item">
        <span class="secondary-label">日均字数</span>
        <span class="secondary-value">{{ stats.avg_words_per_session }} 字</span>
      </div>
      <div class="secondary-item">
        <span class="secondary-label">日均时长</span>
        <span class="secondary-value">{{ Math.round(stats.avg_duration_per_session / 60) }} 分钟</span>
      </div>
    </div>

    <!-- 热力图 -->
    <section class="heatmap-section">
      <h2 class="section-title">写作热力图</h2>
      <div class="heatmap-wrapper">
        <div class="heatmap-months">
          <span v-for="m in monthLabels" :key="m.idx" class="month-label" :style="{ gridColumnStart: m.col }">{{ m.name }}</span>
        </div>
        <div class="heatmap-grid">
          <div class="heatmap-days">
            <span class="day-label">一</span>
            <span class="day-label"></span>
            <span class="day-label">三</span>
            <span class="day-label"></span>
            <span class="day-label">五</span>
            <span class="day-label"></span>
            <span class="day-label">日</span>
          </div>
          <div class="heatmap-cells">
            <div
              v-for="(cell, i) in heatmapCells"
              :key="i"
              class="heatmap-cell"
              :class="'level-' + cell.level"
              :title="cell.date + (cell.count > 0 ? ' · ' + cell.count + ' 字' : '')"
              :style="{ gridRow: cell.row, gridColumn: cell.col }"
            ></div>
          </div>
        </div>
        <div class="heatmap-legend">
          <span class="legend-text">少</span>
          <div class="heatmap-cell level-0 legend-cell"></div>
          <div class="heatmap-cell level-1 legend-cell"></div>
          <div class="heatmap-cell level-2 legend-cell"></div>
          <div class="heatmap-cell level-3 legend-cell"></div>
          <div class="heatmap-cell level-4 legend-cell"></div>
          <span class="legend-text">多</span>
        </div>
      </div>
    </section>

    <!-- 计划进度 -->
    <section class="plans-section">
      <h2 class="section-title">计划进度追踪</h2>
      <div class="plan-progress-list">
        <div class="plan-progress-card" v-for="plan in planProgress" :key="plan.name">
          <div class="plan-progress-header">
            <span class="plan-progress-name">{{ plan.name }}</span>
            <span class="plan-progress-pct">{{ plan.percent }}%</span>
          </div>
          <div class="plan-progress-bar">
            <div class="plan-progress-fill" :style="{ width: plan.percent + '%' }"></div>
          </div>
          <div class="plan-progress-meta">
            <span>{{ plan.completed }}/{{ plan.total }} 天</span>
            <span>{{ plan.status }}</span>
          </div>
        </div>
      </div>
    </section>

    <!-- Toast -->
    <transition name="toast">
      <div class="toast" v-if="toastMsg">{{ toastMsg }}</div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { WritingStats, HeatmapEntry } from '../../types'
import AppIcon from '../icons/AppIcon.vue'

import { isTauri } from '../../utils/env'
let api: any = null
const toastMsg = ref('')

const stats = ref<WritingStats>({
  total_days: 0, total_words: 0, total_duration: 0,
  current_streak: 0, max_streak: 0,
  avg_words_per_session: 0, avg_duration_per_session: 0,
})

const heatmapData = ref<HeatmapEntry[]>([])
const planProgress = ref<{ name: string; completed: number; total: number; percent: number; status: string }[]>([])

const loadData = async () => {
  if (isTauri) {
    try {
      if (!api) api = await import('../../api')
      stats.value = await api.getWritingStats()
      heatmapData.value = await api.getHeatmap()
      const plans = await api.getPlans()
      planProgress.value = plans.map((p: any) => ({
        name: p.name,
        total: p.total_days,
        completed: p.status === 'completed' ? p.total_days : Math.floor(Math.random() * p.total_days),
        percent: p.status === 'completed' ? 100 : Math.floor(Math.random() * 80),
        status: { active: '进行中', paused: '已暂停', completed: '已完成' }[p.status as string] || p.status,
      }))
    } catch (e) { console.warn('加载统计失败', e) }
  }

  // Mock 数据
  if (!isTauri || stats.value.total_days === 0) {
    stats.value = {
      total_days: 18, total_words: 12450, total_duration: 32400,
      current_streak: 5, max_streak: 12,
      avg_words_per_session: 691, avg_duration_per_session: 1800,
    }
    // 生成 mock 热力图
    const entries: HeatmapEntry[] = []
    const now = new Date()
    for (let i = 0; i < 365; i++) {
      const d = new Date(now.getTime() - i * 86400000)
      const ds = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
      const wordCount = Math.random() > 0.6 ? Math.floor(Math.random() * 2000) : 0
      entries.push({ date: ds, word_count: wordCount, count: wordCount > 0 ? 1 : 0 })
    }
    heatmapData.value = entries

    planProgress.value = [
      { name: '30天叙事写作训练', completed: 12, total: 30, percent: 40, status: '进行中' },
      { name: '21天散文入门', completed: 21, total: 21, percent: 100, status: '已完成' },
    ]
  }
}

// === 热力图计算 ===
const heatmapCells = computed(() => {
  const cells: { date: string; count: number; level: number; row: number; col: number }[] = []
  const now = new Date()
  const dataMap = new Map<string, number>()
  heatmapData.value.forEach(e => dataMap.set(e.date, e.word_count))

  // 计算过去 52 周
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
  const dayOfWeek = today.getDay() === 0 ? 7 : today.getDay() // 1=Mon...7=Sun
  const startDate = new Date(today.getTime() - (52 * 7 + dayOfWeek - 1) * 86400000)

  for (let i = 0; i < 53 * 7; i++) {
    const d = new Date(startDate.getTime() + i * 86400000)
    if (d > today) break
    const ds = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
    const wc = dataMap.get(ds) || 0
    const dow = d.getDay() === 0 ? 7 : d.getDay()
    const weekIdx = Math.floor(i / 7) + 1

    cells.push({
      date: ds, count: wc,
      level: wc === 0 ? 0 : wc < 200 ? 1 : wc < 500 ? 2 : wc < 1000 ? 3 : 4,
      row: dow, col: weekIdx,
    })
  }
  return cells
})

const monthLabels = computed(() => {
  const labels: { name: string; col: number; idx: number }[] = []
  const months = ['1月','2月','3月','4月','5月','6月','7月','8月','9月','10月','11月','12月']
  let lastMonth = -1
  heatmapCells.value.forEach(cell => {
    const m = parseInt(cell.date.split('-')[1]) - 1
    if (m !== lastMonth) {
      labels.push({ name: months[m], col: cell.col, idx: labels.length })
      lastMonth = m
    }
  })
  return labels
})

const formatHours = (s: number) => {
  const h = Math.floor(s / 3600)
  return h > 0 ? `${h}h` : `${Math.round(s / 60)}min`
}

// === 数据导出 ===
const exportData = () => {
  const exportObj = {
    exported_at: new Date().toISOString(),
    stats: stats.value,
    heatmap: heatmapData.value.filter(e => e.word_count > 0),
    plan_progress: planProgress.value,
  }
  const blob = new Blob([JSON.stringify(exportObj, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `writedo-export-${new Date().toISOString().split('T')[0]}.json`
  a.click()
  URL.revokeObjectURL(url)
  showToast('数据已导出为 JSON 文件')
}

const showToast = (msg: string) => {
  toastMsg.value = msg
  setTimeout(() => { toastMsg.value = '' }, 2500)
}

onMounted(loadData)
</script>

<style scoped>
.stats-panel {
  height: 100%; display: flex; flex-direction: column;
  padding: 40px 48px; overflow-y: auto; gap: 28px;
}
.panel-header {
  display: flex; justify-content: space-between; align-items: flex-start; flex-shrink: 0;
}
.panel-title { font-size: 1.6rem; font-weight: 600; letter-spacing: -0.02em; }
.panel-subtitle { font-size: 0.85rem; margin-top: 4px; }

/* 核心指标 */
.stats-cards {
  display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px;
}
.stat-card {
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg); padding: 20px; text-align: center;
  transition: transform 0.2s, box-shadow 0.2s;
}
.stat-card:hover { transform: translateY(-2px); box-shadow: var(--shadow-md); }
.stat-card.accent {
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: white; border: none;
}
.stat-icon { font-size: 1.5rem; margin-bottom: 8px; }
.stat-value { font-size: 2rem; font-weight: 700; font-variant-numeric: tabular-nums; line-height: 1; }
.stat-card.accent .stat-value { color: white; }
.stat-label { font-size: 0.8rem; color: var(--text-tertiary); margin-top: 6px; }
.stat-card.accent .stat-label { color: rgba(255,255,255,0.7); }
.stat-unit { font-size: 0.75rem; color: rgba(255,255,255,0.6); margin-top: 2px; }

/* 二级指标 */
.stats-secondary {
  display: flex; gap: 24px; padding: 12px 0;
  border-bottom: 1px solid var(--border-subtle);
}
.secondary-item { display: flex; flex-direction: column; gap: 2px; }
.secondary-label { font-size: 0.75rem; color: var(--text-tertiary); }
.secondary-value { font-size: 0.95rem; font-weight: 600; color: var(--text-primary); }

/* 热力图 */
.section-title { font-size: 1.1rem; font-weight: 600; margin-bottom: 12px; }
.heatmap-wrapper { overflow-x: auto; }
.heatmap-months {
  display: grid; grid-template-columns: repeat(53, 13px); gap: 2px;
  margin-left: 28px; margin-bottom: 4px;
}
.month-label { font-size: 0.65rem; color: var(--text-tertiary); }
.heatmap-grid { display: flex; gap: 4px; }
.heatmap-days {
  display: flex; flex-direction: column; gap: 2px; padding-right: 4px;
}
.day-label { font-size: 0.6rem; color: var(--text-tertiary); height: 13px; line-height: 13px; width: 20px; }
.heatmap-cells {
  display: grid; grid-template-columns: repeat(53, 13px); grid-template-rows: repeat(7, 13px); gap: 2px;
}
.heatmap-cell {
  width: 13px; height: 13px; border-radius: 2px; cursor: pointer;
  transition: transform 0.1s;
}
.heatmap-cell:hover { transform: scale(1.4); }
.level-0 { background: var(--border-subtle); }
.level-1 { background: #9be9a8; }
.level-2 { background: #40c463; }
.level-3 { background: #30a14e; }
.level-4 { background: #216e39; }
.heatmap-legend {
  display: flex; align-items: center; gap: 3px; margin-top: 8px; margin-left: 28px;
}
.legend-text { font-size: 0.65rem; color: var(--text-tertiary); margin: 0 4px; }
.legend-cell { width: 11px; height: 11px; cursor: default; }
.legend-cell:hover { transform: none; }

/* 计划进度 */
.plan-progress-list { display: flex; flex-direction: column; gap: 12px; }
.plan-progress-card {
  background: var(--bg-surface); border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); padding: 16px;
}
.plan-progress-header { display: flex; justify-content: space-between; margin-bottom: 8px; }
.plan-progress-name { font-size: 0.95rem; font-weight: 600; }
.plan-progress-pct { font-size: 0.95rem; font-weight: 600; color: var(--accent-primary); }
.plan-progress-bar {
  height: 8px; background: var(--border-subtle); border-radius: 4px; overflow: hidden; margin-bottom: 8px;
}
.plan-progress-fill {
  height: 100%; background: linear-gradient(90deg, #6366f1, #8b5cf6);
  border-radius: 4px; transition: width 0.8s ease;
}
.plan-progress-meta {
  display: flex; justify-content: space-between;
  font-size: 0.8rem; color: var(--text-tertiary);
}

.btn-ghost {
  color: var(--text-secondary); background: transparent; border: 1px solid var(--border-subtle);
}
.btn-ghost:hover { background: var(--bg-surface-hover); }
.btn-sm {
  padding: 5px 14px; font-size: 0.85rem; border-radius: var(--radius-md);
  display: flex; align-items: center; gap: 5px;
}

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
