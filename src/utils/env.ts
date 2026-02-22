/**
 * 判断当前是否在 Tauri 环境中运行
 * Tauri v2 使用 __TAURI_INTERNALS__，v1 使用 __TAURI__
 */
export const isTauri: boolean =
    typeof window !== 'undefined' &&
    ('__TAURI_INTERNALS__' in window || '__TAURI__' in window)
