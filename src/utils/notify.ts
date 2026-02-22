/**
 * macOS 原生通知工具
 *
 * 封装 Tauri notification 插件，提供简单的通知 API。
 * 非 Tauri 环境（dev 模式）回退到 Web Notification API。
 */

const isTauri = typeof window !== 'undefined' && '__TAURI__' in window

/**
 * 发送系统通知
 */
export async function sendNotification(title: string, body?: string): Promise<void> {
    if (isTauri) {
        try {
            const { sendNotification: tauriNotify } = await import('@tauri-apps/plugin-notification')
            tauriNotify({ title, body })
        } catch (e) {
            console.warn('通知发送失败:', e)
            fallbackNotify(title, body)
        }
    } else {
        fallbackNotify(title, body)
    }
}

/**
 * 请求通知权限（Tauri 环境下自动处理）
 */
export async function requestPermission(): Promise<boolean> {
    if (isTauri) {
        try {
            const { isPermissionGranted, requestPermission } = await import('@tauri-apps/plugin-notification')
            let granted = await isPermissionGranted()
            if (!granted) {
                const perm = await requestPermission()
                granted = perm === 'granted'
            }
            return granted
        } catch {
            return false
        }
    } else {
        if ('Notification' in window) {
            const perm = await Notification.requestPermission()
            return perm === 'granted'
        }
        return false
    }
}

/**
 * 今日任务提醒
 */
export async function notifyTodayTasks(count: number): Promise<void> {
    if (count > 0) {
        await sendNotification(
            'WriteDo - 今日待办',
            `你有 ${count} 个待办任务需要完成 📝`
        )
    }
}

/**
 * 任务完成提醒
 */
export async function notifyTaskCompleted(taskTitle: string): Promise<void> {
    await sendNotification(
        '任务已完成 ✅',
        taskTitle
    )
}

/**
 * Web Notification 回退
 */
function fallbackNotify(title: string, body?: string) {
    if ('Notification' in window && Notification.permission === 'granted') {
        new Notification(title, { body })
    } else {
        console.log(`[通知] ${title}${body ? ': ' + body : ''}`)
    }
}
