<script setup lang="ts">
import { ref, watch, computed, onMounted, onBeforeUnmount } from 'vue';
import WritingEditor from './components/editor/WritingEditor.vue';
import RitualTransition from './components/editor/RitualTransition.vue';
import TaskPanel from './components/tasks/TaskPanel.vue';
import PlanPanel from './components/plans/PlanPanel.vue';
import StatsPanel from './components/stats/StatsPanel.vue';
import SettingsPanel from './components/SettingsPanel.vue';
import type { CustomFont } from './components/SettingsPanel.vue';

const activeTab = ref('writing');

// === 主题（由设置面板控制）===
const isDarkTheme = ref(false);
watch(isDarkTheme, (dark) => {
  if (dark) {
    document.documentElement.setAttribute('data-theme', 'dark');
  } else {
    document.documentElement.removeAttribute('data-theme');
  }
  localStorage.setItem('theme', dark ? 'dark' : 'light');
});

// === 编辑器状态 ===
const content = ref('');
const isFocusMode = ref(false);
const isTypewriterMode = ref(false);
const isFullscreen = ref(false);
const editorRef = ref<InstanceType<typeof WritingEditor> | null>(null);
const saveToast = ref('');
const showRitual = ref(false);

// 今日写作任务
const todayWriting = ref<{ plan_name: string; plan_id: number; day_number: number; title: string; prompt: string; is_completed: boolean } | null>(null);

const loadTodayWriting = async () => {
  if (isTauri) {
    try {
      if (!apiModule) apiModule = await import('./api');
      todayWriting.value = await apiModule.getTodayWriting();
    } catch (e) {
      console.warn('加载今日写作失败', e);
    }
  }
};

import { isTauri } from './utils/env';
let apiModule: any = null;

const saveWriting = async () => {
  const editor = editorRef.value;
  if (!editor) return;
  const charCount = editor.getCharCount();
  const duration = editor.getDuration();
  const htmlContent = editor.getContent();

  if (charCount === 0) {
    showSaveToast('没有内容可保存');
    return;
  }

  if (isTauri) {
    try {
      if (!apiModule) apiModule = await import('./api');
      await apiModule.saveWriting({
        title: '今日写作',
        content: htmlContent,
        word_count: charCount,
        duration_seconds: duration,
      });
      showSaveToast(`已保存 (${charCount} 字)`);
    } catch (e) {
      console.error('保存失败', e);
      showSaveToast('保存失败');
    }
  } else {
    showSaveToast(`已保存 (${charCount} 字·${Math.floor(duration / 60)}分钟)`);
  }
};

const showSaveToast = (msg: string) => {
  saveToast.value = msg;
  setTimeout(() => { saveToast.value = ''; }, 2500);
};

// === 设置面板 ===
const showSettings = ref(false);

// 字体设置
const fontFamily = ref("'Inter', sans-serif");
const fontSize = ref(18);
const lineHeight = ref(1.8);
const customFonts = ref<CustomFont[]>([]);

// 编辑器设置
const typewriterPosition = ref(38); // 打字机光标固定位置（%）
const fullscreenPadding = ref(8);   // 全屏两侧留白（%）

// 内置字体
const builtinFonts = [
  { label: 'Inter（默认）', value: "'Inter', sans-serif" },
  { label: 'Georgia', value: "'Georgia', serif" },
  { label: '霞鹜文楷', value: "'LXGW WenKai', serif" },
  { label: '思源宋体', value: "'Noto Serif SC', serif" },
  { label: '思源黑体', value: "'Source Han Sans SC', sans-serif" },
  { label: '苹方', value: "'PingFang SC', sans-serif" },
  { label: 'Courier New', value: "'Courier New', monospace" },
  { label: 'Menlo', value: "'Menlo', monospace" },
];

const allFonts = ref([...builtinFonts]);

const refreshAllFonts = () => {
  allFonts.value = [
    ...builtinFonts,
    ...customFonts.value.map(cf => ({ label: `✦ ${cf.name}`, value: cf.family })),
  ];
};

const addCustomFont = (font: CustomFont) => {
  customFonts.value.push(font);
  refreshAllFonts();
  saveSettings();
};

const removeCustomFont = (index: number) => {
  customFonts.value.splice(index, 1);
  refreshAllFonts();
  saveSettings();
};

// === 持久化 ===
const saveSettings = () => {
  localStorage.setItem('writedo-settings', JSON.stringify({
    fontFamily: fontFamily.value,
    fontSize: fontSize.value,
    lineHeight: lineHeight.value,
    typewriterPosition: typewriterPosition.value,
    fullscreenPadding: fullscreenPadding.value,
    customFonts: customFonts.value,
  }));
};

watch([fontFamily, fontSize, lineHeight, typewriterPosition, fullscreenPadding], saveSettings);

onMounted(() => {
  // 恢复主题
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'dark' || (!savedTheme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    isDarkTheme.value = true;
    document.documentElement.setAttribute('data-theme', 'dark');
  }

  // 恢复设置
  const saved = localStorage.getItem('writedo-settings');
  if (saved) {
    try {
      const s = JSON.parse(saved);
      if (s.fontFamily) fontFamily.value = s.fontFamily;
      if (s.fontSize) fontSize.value = s.fontSize;
      if (s.lineHeight) lineHeight.value = s.lineHeight;
      if (s.typewriterPosition != null) typewriterPosition.value = s.typewriterPosition;
      if (s.fullscreenPadding != null) fullscreenPadding.value = s.fullscreenPadding;
      if (s.customFonts) {
        s.customFonts.forEach((cf: CustomFont) => {
          const fontFace = new FontFace(cf.family.replace(/'/g, ''), `url(${cf.dataUrl})`);
          fontFace.load().then(loaded => {
            document.fonts.add(loaded);
            customFonts.value.push(cf);
            refreshAllFonts();
          }).catch(() => {});
        });
      }
    } catch {}
  }
  refreshAllFonts();
});

// === 可拖拽宽度 ===
const editorWidthPercent = ref(85);
const isDragging = ref(false);

const onResizeStart = (e: MouseEvent) => { isDragging.value = true; e.preventDefault(); };
const onResizeMove = (e: MouseEvent) => {
  if (!isDragging.value) return;
  const main = document.querySelector('.main-content') as HTMLElement;
  if (!main) return;
  const rect = main.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const percent = (mouseX / rect.width) * 100;
  const rightMarginPercent = 100 - percent;
  const newWidth = 100 - rightMarginPercent * 2;
  editorWidthPercent.value = Math.max(50, Math.min(100, newWidth));
};
const onResizeEnd = () => { isDragging.value = false; };

// === 沉浸模式（使用 Tauri Window API）===
let tauriWindow: any = null;

const checkFullscreenStatus = async () => {
  if (isTauri) {
    try {
      if (!tauriWindow) {
        const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow');
        tauriWindow = getCurrentWebviewWindow();
      }
      isFullscreen.value = await tauriWindow.isFullscreen();
    } catch {
      isFullscreen.value = !!document.fullscreenElement;
    }
  } else {
    isFullscreen.value = !!document.fullscreenElement;
  }
};

const toggleImmersiveMode = async () => {
  if (!isFullscreen.value) {
    showRitual.value = true;
  } else {
    // 退出沉浸
    if (isTauri && tauriWindow) {
      await tauriWindow.setFullscreen(false);
    } else {
      await document.exitFullscreen().catch(() => {});
    }
    isFullscreen.value = false;
    isFocusMode.value = false;
    isTypewriterMode.value = false;
  }
};

const onRitualComplete = async () => {
  showRitual.value = false;
  if (isTauri) {
    try {
      if (!tauriWindow) {
        const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow');
        tauriWindow = getCurrentWebviewWindow();
      }
      await tauriWindow.setFullscreen(true);
    } catch {
      await document.documentElement.requestFullscreen().catch(() => {});
    }
  } else {
    await document.documentElement.requestFullscreen().catch(() => {});
  }
  isFullscreen.value = true;
  isFocusMode.value = true;
  isTypewriterMode.value = true;
};

// 全屏模式下的内容区样式（用设置中的 padding 值）
const fullscreenContentStyle = computed(() => {
  if (!isFullscreen.value) return {};
  return { paddingLeft: fullscreenPadding.value + '%', paddingRight: fullscreenPadding.value + '%' };
});

onMounted(() => {
  document.addEventListener('mousemove', onResizeMove);
  document.addEventListener('mouseup', onResizeEnd);
  document.addEventListener('fullscreenchange', checkFullscreenStatus);
  loadTodayWriting();
});
onBeforeUnmount(() => {
  document.removeEventListener('mousemove', onResizeMove);
  document.removeEventListener('mouseup', onResizeEnd);
  document.removeEventListener('fullscreenchange', checkFullscreenStatus);
});
</script>

<template>
  <div class="app-layout" :class="{ 'is-immersive-fullscreen': isFullscreen, 'is-dragging': isDragging }">
    <!-- 侧边栏 -->
    <aside class="sidebar">
      <div class="sidebar-header">
        <div class="logo">
          <div class="logo-icon">W</div>
          <span class="logo-text">WriteDo</span>
        </div>
      </div>
      
      <nav class="sidebar-nav">
        <button class="nav-item" :class="{ active: activeTab === 'writing' }" @click="activeTab = 'writing'">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
          <span class="font-medium">码字模式</span>
        </button>
        <button class="nav-item" :class="{ active: activeTab === 'tasks' }" @click="activeTab = 'tasks'">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 11 12 14 22 4"></polyline><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path></svg>
          <span class="font-medium">待办任务</span>
        </button>
        <button class="nav-item" :class="{ active: activeTab === 'plans' }" @click="activeTab = 'plans'">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect><line x1="16" y1="2" x2="16" y2="6"></line><line x1="8" y1="2" x2="8" y2="6"></line><line x1="3" y1="10" x2="21" y2="10"></line></svg>
          <span class="font-medium">写作计划</span>
        </button>
        <button class="nav-item" :class="{ active: activeTab === 'stats' }" @click="activeTab = 'stats'">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="20" x2="18" y2="10"></line><line x1="12" y1="20" x2="12" y2="4"></line><line x1="6" y1="20" x2="6" y2="14"></line></svg>
          <span class="font-medium">数据统计</span>
        </button>
      </nav>
      
      <div class="sidebar-footer">
        <button class="nav-item" @click="showSettings = true" title="设置">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
          <span class="font-medium">设置</span>
        </button>
      </div>
    </aside>

    <!-- 主内容区 -->
    <main class="main-content">
      <div class="content-wrapper" v-if="activeTab === 'writing'"
           :style="[
             { maxWidth: editorWidthPercent + '%' },
             fullscreenContentStyle,
           ]">
        
        <header class="content-header">
          <div>
            <p class="page-subtitle text-secondary" v-if="todayWriting">
              {{ todayWriting.plan_name }} · 第 {{ todayWriting.day_number }} 天
            </p>
            <h1 class="page-title">今日写作</h1>
          </div>
          <button class="btn-primary flex items-center gap-2" @click="toggleImmersiveMode">
            <template v-if="!isFullscreen">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
              沉浸模式
            </template>
            <template v-else>
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 14h6v6M20 10h-6V4M14 10l7-7M3 21l7-7"></path></svg>
              退出沉浸
            </template>
          </button>
        </header>

        <section class="prompt-card" v-if="todayWriting">
          <h2 class="prompt-title">{{ todayWriting.title }}</h2>
          <p class="prompt-desc text-secondary mt-2">
            {{ todayWriting.prompt }}
          </p>
        </section>
        <section class="prompt-card" v-else>
          <p class="prompt-desc text-secondary">今日没有写作任务，自由书写吧！</p>
        </section>

        <WritingEditor 
          ref="editorRef"
          v-model="content"
          :is-focus-mode="isFocusMode"
          :is-typewriter-mode="isTypewriterMode"
          :font-family="fontFamily"
          :font-size="fontSize"
          :line-height="lineHeight"
          :typewriter-position="typewriterPosition"
          @toggle-focus-mode="isFocusMode = !isFocusMode"
          @toggle-typewriter-mode="isTypewriterMode = !isTypewriterMode"
          @save="saveWriting"
        />

        <div class="resize-handle-right" @mousedown="onResizeStart">
          <div class="resize-handle-bar"></div>
        </div>
      </div>
      
      <!-- 待办任务面板 -->
      <TaskPanel v-else-if="activeTab === 'tasks'" />

      <!-- 写作计划面板 -->
      <PlanPanel v-else-if="activeTab === 'plans'" />

      <!-- 数据统计面板 -->
      <StatsPanel v-else-if="activeTab === 'stats'" />
    </main>

    <!-- 设置面板 -->
    <SettingsPanel
      :visible="showSettings"
      :is-dark="isDarkTheme"
      :font-family="fontFamily"
      :font-size="fontSize"
      :line-height="lineHeight"
      :typewriter-position="typewriterPosition"
      :fullscreen-padding="fullscreenPadding"
      :custom-fonts="customFonts"
      :all-fonts="allFonts"
      @close="showSettings = false"
      @update:is-dark="isDarkTheme = $event"
      @update:font-family="fontFamily = $event"
      @update:font-size="fontSize = $event"
      @update:line-height="lineHeight = $event"
      @update:typewriter-position="typewriterPosition = $event"
      @update:fullscreen-padding="fullscreenPadding = $event"
      @add-custom-font="addCustomFont"
      @remove-custom-font="removeCustomFont"
    />

    <!-- 保存 Toast -->
    <transition name="save-toast">
      <div class="save-toast" v-if="saveToast">{{ saveToast }}</div>
    </transition>

    <!-- 写作仪式感过渡 -->
    <RitualTransition :visible="showRitual" @complete="onRitualComplete" />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex; height: 100vh; width: 100vw;
  background-color: var(--bg-surface); transition: background-color 0.3s ease;
}
.app-layout.is-dragging { user-select: none; cursor: col-resize; }

.sidebar {
  width: 220px; min-width: 220px; background-color: var(--bg-surface);
  border-right: 1px solid var(--border-subtle); display: flex; flex-direction: column;
  padding: 24px 16px; transition: all 0.4s cubic-bezier(0.165, 0.84, 0.44, 1);
}
.sidebar-header { margin-bottom: 32px; padding: 0 8px; }
.logo { display: flex; align-items: center; gap: 12px; }
.logo-icon {
  width: 30px; height: 30px; background: var(--text-primary); border-radius: 6px;
  display: flex; align-items: center; justify-content: center;
  font-weight: 700; font-size: 1.1rem; color: var(--bg-surface);
}
.logo-text { font-size: 1.1rem; font-weight: 600; letter-spacing: -0.01em; }
.sidebar-nav { display: flex; flex-direction: column; gap: 4px; flex: 1; }
.nav-item {
  display: flex; align-items: center; gap: 12px; width: 100%;
  padding: 8px 12px; border-radius: var(--radius-md);
  color: var(--text-secondary); font-size: 0.95rem; transition: all 0.2s;
}
.nav-item:hover { background-color: var(--bg-surface-hover); color: var(--text-primary); }
.nav-item.active {
  background-color: var(--bg-base); color: var(--text-primary);
  box-shadow: var(--shadow-sm); border: 1px solid var(--border-subtle);
}
.sidebar-footer { padding-top: 16px; }

.main-content {
  flex: 1; display: flex; flex-direction: column;
  background-color: var(--bg-base); overflow: hidden;
  transition: background-color 0.3s ease; position: relative;
}
.content-wrapper {
  padding: 40px 48px; width: 100%; margin: 0 auto; height: 100%;
  display: flex; flex-direction: column; overflow-y: auto;
  position: relative; transition: max-width 0.15s ease, padding 0.3s ease;
}
.content-header {
  display: flex; justify-content: space-between; align-items: flex-start;
  margin-bottom: 20px; flex-shrink: 0;
}
.page-title { font-size: 1.6rem; font-weight: 600; letter-spacing: -0.02em; }
.page-subtitle { font-size: 0.8rem; letter-spacing: 0.05em; margin-bottom: 4px; }

.prompt-card {
  padding-left: 16px; margin-bottom: 28px;
  border-left: 3px solid var(--accent-primary); flex-shrink: 0;
}
.prompt-title { font-size: 1.15rem; font-weight: 600; margin-bottom: 4px; }
.prompt-desc { font-size: 0.95rem; line-height: 1.6; }

.resize-handle-right {
  position: absolute; top: 0; right: -4px; width: 8px; height: 100%;
  cursor: col-resize; z-index: 10; display: flex; align-items: center; justify-content: center;
}
.resize-handle-bar {
  width: 3px; height: 40px; border-radius: 3px;
  background-color: transparent; transition: background-color 0.2s;
}
.resize-handle-right:hover .resize-handle-bar { background-color: var(--border-focus); }
.app-layout.is-dragging .resize-handle-bar { background-color: var(--accent-primary); }

.is-immersive-fullscreen .sidebar {
  width: 0; min-width: 0; padding: 0; opacity: 0; overflow: hidden; border-right: none;
}
.is-immersive-fullscreen .content-wrapper {
  max-width: 100% !important; /* 全屏时取消 max-width 限制，靠 padding 控制边距 */
  padding-top: 48px;
}

.save-toast {
  position: fixed; bottom: 32px; left: 50%; transform: translateX(-50%);
  background: var(--text-primary); color: var(--bg-base);
  padding: 10px 24px; border-radius: 20px;
  font-size: 0.9rem; font-weight: 500;
  box-shadow: 0 4px 16px rgba(0,0,0,0.15);
  z-index: 200;
}
.save-toast-enter-active { transition: all 0.3s ease; }
.save-toast-leave-active { transition: all 0.3s ease; }
.save-toast-enter-from { opacity: 0; transform: translateX(-50%) translateY(20px); }
.save-toast-leave-to { opacity: 0; transform: translateX(-50%) translateY(20px); }
</style>