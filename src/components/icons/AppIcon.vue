<!-- 统一 SVG 图标组件 -->
<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :width="size" :height="size"
    viewBox="0 0 24 24"
    fill="none"
    :stroke="color"
    :stroke-width="strokeWidth"
    stroke-linecap="round"
    stroke-linejoin="round"
    class="icon"
    :class="'icon-' + name"
  >
    <!-- 火焰 / 连续打卡 -->
    <template v-if="name === 'flame'">
      <path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" fill="currentColor" stroke="none"/>
    </template>

    <!-- 钢笔 / 写作 -->
    <template v-else-if="name === 'pen'">
      <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"/>
    </template>

    <!-- 日历 / 天数 -->
    <template v-else-if="name === 'calendar'">
      <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
      <line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/>
      <line x1="3" y1="10" x2="21" y2="10"/>
    </template>

    <!-- 时钟 / 时长 -->
    <template v-else-if="name === 'clock'">
      <circle cx="12" cy="12" r="10"/>
      <polyline points="12 6 12 12 16 14"/>
    </template>

    <!-- 写作 / 仪式感 -->
    <template v-else-if="name === 'quill'">
      <path d="M20 4c-1 1-3.5 1.5-5 2l-7 7-2.5 2.5L4 17l1.5-1.5L8 13l7-7c.5-1.5 1-4 2-5z" fill="currentColor" stroke="none" opacity="0.8"/>
      <path d="M4 17v3h3l2-2"/>
      <path d="M15 5s2-1 4 1 1 4 1 4"/>
    </template>

    <!-- 星星 / 里程碑 -->
    <template v-else-if="name === 'star'">
      <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" fill="currentColor" stroke="none"/>
    </template>

    <!-- 奖杯 / 成就 -->
    <template v-else-if="name === 'trophy'">
      <path d="M6 9H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h2"/>
      <path d="M18 9h2a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2h-2"/>
      <path d="M6 3h12v7a6 6 0 0 1-12 0V3z"/>
      <path d="M9 21h6"/><path d="M12 16v5"/>
    </template>

    <!-- 对勾 / 完成 -->
    <template v-else-if="name === 'check'">
      <polyline points="20 6 9 17 4 12"/>
    </template>

    <!-- 叉号 / 失败 -->
    <template v-else-if="name === 'x'">
      <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
    </template>

    <!-- 文件 / 保存 -->
    <template v-else-if="name === 'save'">
      <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
      <polyline points="17 21 17 13 7 13 7 21"/><polyline points="7 3 7 8 15 8"/>
    </template>

    <!-- 导出 -->
    <template v-else-if="name === 'export'">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/>
    </template>

    <!-- 目标 / 里程碑小 -->
    <template v-else-if="name === 'target'">
      <circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/>
    </template>

    <!-- 闪电 / 能量 -->
    <template v-else-if="name === 'zap'">
      <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" fill="currentColor" stroke="none"/>
    </template>
  </svg>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  name: string
  size?: number | string
  color?: string
  strokeWidth?: number | string
}>(), {
  size: 20,
  color: 'currentColor',
  strokeWidth: 2,
})
</script>

<style scoped>
.icon { flex-shrink: 0; vertical-align: middle; }
</style>
