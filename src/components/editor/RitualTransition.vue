<template>
  <transition name="ritual">
    <div class="ritual-overlay" v-if="visible" @animationend="onComplete">
      <div class="ritual-content">
        <div class="ritual-circle">
          <div class="ritual-ring ring-1"></div>
          <div class="ritual-ring ring-2"></div>
          <div class="ritual-ring ring-3"></div>
          <div class="ritual-icon">✍️</div>
        </div>
        <p class="ritual-text">{{ currentText }}</p>
        <div class="ritual-dots">
          <span v-for="i in 3" :key="i" class="dot" :style="{ animationDelay: (i - 1) * 0.3 + 's' }"></span>
        </div>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount } from 'vue'

const props = defineProps<{ visible: boolean }>()
const emit = defineEmits(['complete'])

const ritualTexts = [
  '深呼吸，放空思绪…',
  '让文字自然流淌…',
  '进入心流状态…',
]

const currentText = ref(ritualTexts[0])
let textTimer: ReturnType<typeof setTimeout> | null = null
let completeTimer: ReturnType<typeof setTimeout> | null = null

watch(() => props.visible, (v) => {
  if (v) {
    currentText.value = ritualTexts[0]
    let idx = 0
    textTimer = setInterval(() => {
      idx++
      if (idx < ritualTexts.length) {
        currentText.value = ritualTexts[idx]
      }
    }, 1000)
    completeTimer = setTimeout(() => {
      if (textTimer) clearInterval(textTimer)
      emit('complete')
    }, 3000)
  } else {
    if (textTimer) clearInterval(textTimer)
    if (completeTimer) clearTimeout(completeTimer)
  }
})

onBeforeUnmount(() => {
  if (textTimer) clearInterval(textTimer)
  if (completeTimer) clearTimeout(completeTimer)
})

const onComplete = () => {}
</script>

<style scoped>
.ritual-overlay {
  position: fixed; inset: 0; z-index: 100;
  background: rgba(0, 0, 0, 0.85);
  display: flex; align-items: center; justify-content: center;
  animation: ritual-in 0.5s ease-out, ritual-out 0.5s ease-in 2.5s forwards;
}

.ritual-content {
  display: flex; flex-direction: column; align-items: center; gap: 24px;
}

.ritual-circle {
  position: relative; width: 120px; height: 120px;
  display: flex; align-items: center; justify-content: center;
}

.ritual-ring {
  position: absolute; border-radius: 50%;
  border: 2px solid rgba(255, 255, 255, 0.15);
}
.ring-1 {
  width: 80px; height: 80px;
  animation: ring-expand 2s ease-out infinite;
}
.ring-2 {
  width: 80px; height: 80px;
  animation: ring-expand 2s ease-out 0.6s infinite;
}
.ring-3 {
  width: 80px; height: 80px;
  animation: ring-expand 2s ease-out 1.2s infinite;
}

.ritual-icon {
  font-size: 2.5rem; z-index: 1;
  animation: icon-pulse 1.5s ease-in-out infinite;
}

.ritual-text {
  color: rgba(255, 255, 255, 0.8);
  font-size: 1.2rem; font-weight: 300;
  letter-spacing: 0.1em;
  animation: text-fade 1s ease-in-out;
}

.ritual-dots {
  display: flex; gap: 8px;
}
.dot {
  width: 6px; height: 6px; border-radius: 50%;
  background: rgba(255, 255, 255, 0.4);
  animation: dot-bounce 1.2s ease-in-out infinite;
}

@keyframes ritual-in {
  from { opacity: 0; }
  to { opacity: 1; }
}
@keyframes ritual-out {
  from { opacity: 1; }
  to { opacity: 0; }
}

@keyframes ring-expand {
  0% { transform: scale(1); opacity: 0.6; }
  100% { transform: scale(1.8); opacity: 0; }
}

@keyframes icon-pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.15); }
}

@keyframes text-fade {
  0% { opacity: 0; transform: translateY(10px); }
  100% { opacity: 1; transform: translateY(0); }
}

@keyframes dot-bounce {
  0%, 100% { transform: translateY(0); opacity: 0.4; }
  50% { transform: translateY(-6px); opacity: 1; }
}

.ritual-enter-active { transition: opacity 0.5s ease; }
.ritual-leave-active { transition: opacity 0.3s ease; }
.ritual-enter-from, .ritual-leave-to { opacity: 0; }
</style>
