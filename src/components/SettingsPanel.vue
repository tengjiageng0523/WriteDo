<template>
  <transition name="slide-in">
    <div class="settings-overlay" v-if="visible" @click.self="$emit('close')">
      <div class="settings-panel">
        <header class="settings-header">
          <h3>设置</h3>
          <button class="btn-icon" @click="$emit('close')" title="关闭">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
          </button>
        </header>

        <div class="settings-body">

          <!-- 外观 -->
          <section class="settings-section">
            <h4 class="section-title">外观</h4>

            <div class="setting-row">
              <label class="setting-label">主题模式</label>
              <div class="toggle-group">
                <button 
                  class="toggle-btn" 
                  :class="{ active: !isDark }" 
                  @click="$emit('update:isDark', false)"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>
                  浅色
                </button>
                <button 
                  class="toggle-btn" 
                  :class="{ active: isDark }" 
                  @click="$emit('update:isDark', true)"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>
                  深色
                </button>
              </div>
            </div>
          </section>

          <!-- 编辑器 -->
          <section class="settings-section">
            <h4 class="section-title">编辑器</h4>

            <div class="setting-row">
              <label class="setting-label">打字机位置</label>
              <div class="slider-row">
                <input 
                  type="range" 
                  :value="typewriterPosition" 
                  @input="$emit('update:typewriterPosition', +($event.target as HTMLInputElement).value)"
                  min="20" max="60" step="1"
                  class="setting-slider"
                />
                <span class="slider-value">{{ typewriterPosition }}%</span>
              </div>
            </div>
            <p class="setting-hint">光标在编辑器中的垂直固定位置，越小越靠上</p>

            <div class="setting-row">
              <label class="setting-label">全屏左右边距</label>
              <div class="slider-row">
                <input 
                  type="range" 
                  :value="fullscreenPadding" 
                  @input="$emit('update:fullscreenPadding', +($event.target as HTMLInputElement).value)"
                  min="0" max="25" step="1"
                  class="setting-slider"
                />
                <span class="slider-value">{{ fullscreenPadding }}%</span>
              </div>
            </div>
            <p class="setting-hint">沉浸模式下编辑区域两侧留白比例</p>
          </section>

          <!-- 字体 -->
          <section class="settings-section">
            <h4 class="section-title">字体</h4>

            <div class="setting-row">
              <label class="setting-label">字体家族</label>
              <select :value="fontFamily" @change="$emit('update:fontFamily', ($event.target as HTMLSelectElement).value)" class="setting-select">
                <option v-for="f in allFonts" :key="f.value" :value="f.value">{{ f.label }}</option>
              </select>
            </div>

            <div class="setting-row">
              <label class="setting-label">字号</label>
              <div class="stepper">
                <button class="stepper-btn" @click="$emit('update:fontSize', Math.max(12, fontSize - 1))">−</button>
                <span class="stepper-value">{{ fontSize }}px</span>
                <button class="stepper-btn" @click="$emit('update:fontSize', Math.min(32, fontSize + 1))">+</button>
              </div>
            </div>

            <div class="setting-row">
              <label class="setting-label">行高</label>
              <div class="stepper">
                <button class="stepper-btn" @click="$emit('update:lineHeight', Math.max(0.8, +(lineHeight - 0.1).toFixed(1)))">−</button>
                <span class="stepper-value">{{ lineHeight }}</span>
                <button class="stepper-btn" @click="$emit('update:lineHeight', Math.min(3.0, +(lineHeight + 0.1).toFixed(1)))">+</button>
              </div>
            </div>
          </section>

          <!-- 自定义字体 -->
          <section class="settings-section">
            <h4 class="section-title">自定义字体</h4>
            <p class="section-desc">上传 .ttf / .otf / .woff / .woff2 字体文件</p>

            <div class="custom-fonts-list" v-if="customFonts.length">
              <div class="custom-font-item" v-for="(cf, i) in customFonts" :key="i">
                <span class="custom-font-name" :style="{ fontFamily: cf.family }">{{ cf.name }}</span>
                <div class="custom-font-actions">
                  <button class="btn-text" @click="$emit('update:fontFamily', cf.family)">使用</button>
                  <button class="btn-text btn-danger" @click="$emit('remove-custom-font', i)">删除</button>
                </div>
              </div>
            </div>
            <p v-else class="section-desc" style="font-size:0.8rem;opacity:0.5">暂无自定义字体</p>

            <label class="upload-btn">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
              上传字体文件
              <input type="file" accept=".ttf,.otf,.woff,.woff2" @change="onFileUpload" hidden />
            </label>
          </section>

        </div>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
export interface CustomFont {
  name: string
  family: string
  dataUrl: string
}

defineProps<{
  visible: boolean
  isDark: boolean
  fontFamily: string
  fontSize: number
  lineHeight: number
  typewriterPosition: number
  fullscreenPadding: number
  customFonts: CustomFont[]
  allFonts: { label: string; value: string }[]
}>()

const emit = defineEmits([
  'close',
  'update:isDark',
  'update:fontFamily',
  'update:fontSize',
  'update:lineHeight',
  'update:typewriterPosition',
  'update:fullscreenPadding',
  'add-custom-font',
  'remove-custom-font',
])

const onFileUpload = (e: Event) => {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    const dataUrl = reader.result as string
    const baseName = file.name.replace(/\.[^.]+$/, '')
    const familyName = `custom-${baseName}-${Date.now()}`
    const fontFace = new FontFace(familyName, `url(${dataUrl})`)
    fontFace.load().then((loaded) => {
      document.fonts.add(loaded)
      emit('add-custom-font', {
        name: baseName,
        family: `'${familyName}'`,
        dataUrl,
      } as CustomFont)
    }).catch(() => {
      alert('字体文件加载失败，请确认文件格式正确')
    })
  }
  reader.readAsDataURL(file)
  ;(e.target as HTMLInputElement).value = ''
}
</script>

<style scoped>
.settings-overlay {
  position: fixed; inset: 0; z-index: 100;
  background: rgba(0, 0, 0, 0.2);
  display: flex; justify-content: flex-end;
}
.settings-panel {
  width: 360px; max-width: 90vw; height: 100%;
  background: var(--bg-base); border-left: 1px solid var(--border-subtle);
  box-shadow: var(--shadow-lg); display: flex; flex-direction: column;
}
.settings-header {
  display: flex; justify-content: space-between; align-items: center;
  padding: 20px 24px; border-bottom: 1px solid var(--border-subtle); flex-shrink: 0;
}
.settings-header h3 { font-size: 1.1rem; font-weight: 600; margin: 0; }

.settings-body { padding: 24px; flex: 1; overflow-y: auto; }

.settings-section { margin-bottom: 28px; }
.section-title {
  font-size: 0.8rem; font-weight: 600; text-transform: uppercase;
  letter-spacing: 0.06em; color: var(--text-tertiary); margin-bottom: 14px;
}
.section-desc { font-size: 0.85rem; color: var(--text-tertiary); margin-bottom: 12px; }

.setting-row {
  display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;
}
.setting-label { font-size: 0.9rem; color: var(--text-secondary); white-space: nowrap; }
.setting-hint {
  font-size: 0.75rem; color: var(--text-tertiary); margin-top: -6px; margin-bottom: 14px;
}

.setting-select {
  font-family: inherit; background: var(--bg-surface); border: 1px solid var(--border-subtle);
  color: var(--text-primary); padding: 5px 10px; border-radius: var(--radius-md);
  font-size: 0.85rem; cursor: pointer; max-width: 170px;
}
.setting-select:focus { outline: none; border-color: var(--accent-primary); }

/* 步进器 */
.stepper {
  display: flex; align-items: center; border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md); overflow: hidden;
}
.stepper-btn {
  width: 32px; height: 32px; display: flex; align-items: center; justify-content: center;
  font-size: 1rem; color: var(--text-secondary); background: var(--bg-surface);
  cursor: pointer; transition: background 0.15s;
}
.stepper-btn:hover { background: var(--bg-surface-hover); color: var(--text-primary); }
.stepper-value {
  min-width: 50px; text-align: center; font-size: 0.85rem;
  color: var(--text-primary); font-variant-numeric: tabular-nums;
}

/* 主题切换按钮组 */
.toggle-group {
  display: flex; border: 1px solid var(--border-subtle); border-radius: var(--radius-md); overflow: hidden;
}
.toggle-btn {
  display: flex; align-items: center; gap: 6px;
  padding: 6px 14px; font-size: 0.85rem; color: var(--text-secondary);
  background: var(--bg-surface); cursor: pointer; transition: all 0.2s;
  border-right: 1px solid var(--border-subtle);
}
.toggle-btn:last-child { border-right: none; }
.toggle-btn:hover { background: var(--bg-surface-hover); }
.toggle-btn.active {
  background: var(--accent-primary); color: #fff; border-color: var(--accent-primary);
}

/* 滑块 */
.slider-row { display: flex; align-items: center; gap: 10px; }
.setting-slider {
  width: 130px; height: 4px; -webkit-appearance: none; appearance: none;
  background: var(--border-subtle); border-radius: 2px; outline: none;
  cursor: pointer; border: none; padding: 0;
}
.setting-slider::-webkit-slider-thumb {
  -webkit-appearance: none; appearance: none;
  width: 16px; height: 16px; border-radius: 50%;
  background: var(--accent-primary); cursor: grab;
  box-shadow: 0 1px 4px rgba(0,0,0,0.2);
}
.setting-slider:focus { box-shadow: none; }
.slider-value {
  font-size: 0.8rem; color: var(--text-tertiary); min-width: 36px;
  text-align: right; font-variant-numeric: tabular-nums;
}

/* 自定义字体 */
.custom-fonts-list { margin-bottom: 12px; }
.custom-font-item {
  display: flex; justify-content: space-between; align-items: center;
  padding: 8px 0; border-bottom: 1px solid var(--border-subtle);
}
.custom-font-name { font-size: 0.9rem; color: var(--text-primary); }
.custom-font-actions { display: flex; gap: 8px; }

.btn-text {
  font-size: 0.8rem; color: var(--accent-primary); cursor: pointer;
  padding: 2px 6px; border-radius: 4px;
}
.btn-text:hover { background: rgba(59, 130, 246, 0.1); }
.btn-danger { color: var(--accent-danger); }
.btn-danger:hover { background: rgba(239, 68, 68, 0.1); }

.upload-btn {
  display: inline-flex; align-items: center; gap: 8px;
  padding: 8px 16px; background: var(--bg-surface);
  border: 1px dashed var(--border-focus); border-radius: var(--radius-md);
  color: var(--text-secondary); font-size: 0.85rem; cursor: pointer; transition: all 0.2s;
}
.upload-btn:hover {
  border-color: var(--accent-primary); color: var(--accent-primary);
  background: rgba(59, 130, 246, 0.05);
}

/* 动画 */
.slide-in-enter-active, .slide-in-leave-active { transition: opacity 0.25s ease; }
.slide-in-enter-active .settings-panel, .slide-in-leave-active .settings-panel {
  transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}
.slide-in-enter-from { opacity: 0; }
.slide-in-enter-from .settings-panel { transform: translateX(100%); }
.slide-in-leave-to { opacity: 0; }
.slide-in-leave-to .settings-panel { transform: translateX(100%); }
</style>
