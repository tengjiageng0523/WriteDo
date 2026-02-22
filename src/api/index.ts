/**
 * WriteDo - Tauri API 封装层
 *
 * 封装所有与 Rust 后端的通信，提供类型安全的 API 函数。
 * 前端组件应调用此模块而非直接使用 invoke。
 */

import { invoke } from '@tauri-apps/api/core';
import type {
    Task,
    CreateTaskRequest,
    UpdateTaskRequest,
    WritingPlan,
    PlanWithDays,
    ImportPlanRequest,
    GeneratePlanRequest,
    TodayWritingTask,
    Writing,
    SaveWritingRequest,
    WritingStats,
    HeatmapEntry,
    WritingFilter,
} from '../types';

// ==================== 任务 API ====================

/** 获取所有任务 */
export async function getTasks(): Promise<Task[]> {
    return invoke('get_tasks');
}

/** 获取今日待办 */
export async function getTodayTasks(): Promise<Task[]> {
    return invoke('get_today_tasks');
}

/** 创建任务 */
export async function createTask(request: CreateTaskRequest): Promise<Task> {
    return invoke('create_task', { request });
}

/** 更新任务 */
export async function updateTask(request: UpdateTaskRequest): Promise<Task> {
    return invoke('update_task', { request });
}

/** 删除任务 */
export async function deleteTask(id: number): Promise<void> {
    return invoke('delete_task', { id });
}

/** 切换任务完成状态 */
export async function toggleTask(id: number): Promise<Task> {
    return invoke('toggle_task', { id });
}

// ==================== 写作计划 API ====================

/** 获取所有计划 */
export async function getPlans(): Promise<WritingPlan[]> {
    return invoke('get_plans');
}

/** 获取计划详情 */
export async function getPlanDetail(planId: number): Promise<PlanWithDays> {
    return invoke('get_plan_detail', { planId });
}

/** 获取今日写作任务 */
export async function getTodayWriting(): Promise<TodayWritingTask | null> {
    return invoke('get_today_writing');
}

/** 导入写作计划 */
export async function importPlan(request: ImportPlanRequest): Promise<number> {
    return invoke('import_plan', { request });
}

/** 自动生成写作计划 */
export async function generatePlan(request: GeneratePlanRequest): Promise<number> {
    return invoke('generate_plan', { request });
}

/** 更新计划状态 */
export async function updatePlanStatus(planId: number, status: string): Promise<void> {
    return invoke('update_plan_status', { planId, status });
}

/** 更新计划基本信息 */
export async function updatePlan(request: { id: number; name?: string; theme?: string; start_date?: string }): Promise<WritingPlan> {
    return invoke('update_plan', { request });
}

/** 删除计划 */
export async function deletePlan(planId: number): Promise<void> {
    return invoke('delete_plan', { planId });
}

/** 更新每日条目 */
export async function updatePlanDay(request: { id: number; title?: string; prompt?: string }): Promise<void> {
    return invoke('update_plan_day', { request });
}

/** 删除每日条目 */
export async function deletePlanDay(dayId: number): Promise<void> {
    return invoke('delete_plan_day', { dayId });
}

// ==================== 写作记录 API ====================

/** 保存写作记录 */
export async function saveWriting(request: SaveWritingRequest): Promise<Writing> {
    return invoke('save_writing', { request });
}

/** 获取写作记录列表 */
export async function getWritings(filter: WritingFilter = {}): Promise<Writing[]> {
    return invoke('get_writings', { filter });
}

/** 获取写作统计 */
export async function getWritingStats(): Promise<WritingStats> {
    return invoke('get_writing_stats');
}

/** 获取热力图数据 */
export async function getHeatmap(): Promise<HeatmapEntry[]> {
    return invoke('get_heatmap');
}

/** 获取写作记录详情 */
export async function getWritingDetail(id: number): Promise<Writing> {
    return invoke('get_writing_detail', { id });
}

// ==================== 批量导入 API ====================

/** 批量导入任务（JSON 格式） */
export async function importTasksJson(jsonContent: string): Promise<Task[]> {
    return invoke('import_tasks_json', { jsonContent });
}

/** 导入写作计划（JSON 格式） */
export async function importPlanJson(jsonContent: string): Promise<number> {
    return invoke('import_plan_json', { jsonContent });
}

/** 导入写作计划（Markdown 格式） */
export async function importPlanMarkdown(mdContent: string, startDate: string): Promise<number> {
    return invoke('import_plan_markdown', { mdContent, startDate });
}
