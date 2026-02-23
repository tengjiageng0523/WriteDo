/**
 * WriteDo - 前端 TypeScript 类型定义
 *
 * 与 Rust 后端 models 保持一一对应，确保前后端类型安全。
 */

// ==================== 任务模块 ====================

/** 任务优先级 */
export type Priority = 'low' | 'medium' | 'high';

/** 重复类型 */
export type RepeatType = 'daily' | 'weekly' | 'monthly';

/** 任务实体 */
export interface Task {
  id: number | null;
  title: string;
  description?: string | null;
  priority: Priority;
  due_date?: string | null;
  repeat?: RepeatType | null;
  completed: boolean;
  created_at?: string | null;
  updated_at?: string | null;
}

/** 创建任务请求 */
export interface CreateTaskRequest {
  title: string;
  description?: string;
  priority?: Priority;
  due_date?: string;
  repeat?: RepeatType;
}

/** 更新任务请求 */
export interface UpdateTaskRequest {
  id: number;
  title?: string;
  description?: string;
  priority?: Priority;
  due_date?: string;
  repeat?: RepeatType;
  completed?: boolean;
}

// ==================== 写作计划模块 ====================

/** 计划状态 */
export type PlanStatus = 'active' | 'paused' | 'completed';

/** 写作计划实体 */
export interface WritingPlan {
  id: number | null;
  name: string;
  theme?: string | null;
  start_date: string;
  total_days: number;
  status: PlanStatus;
  created_at?: string | null;
  completed_days: number;
}

/** 计划每日条目 */
export interface PlanDay {
  id: number | null;
  plan_id: number;
  day_number: number;
  title: string;
  prompt: string;
  scheduled_date?: string | null;
}

/** 计划 + 每日条目完整视图 */
export interface PlanWithDays extends WritingPlan {
  days: PlanDay[];
  completed_days: number;
}

/** 导入写作计划请求 */
export interface ImportPlanRequest {
  name: string;
  theme?: string;
  start_date: string;
  days: ImportPlanDayItem[];
}

/** 导入计划每日条目 */
export interface ImportPlanDayItem {
  day: number;
  title: string;
  prompt: string;
}

/** 自动生成计划请求 */
export interface GeneratePlanRequest {
  name: string;
  theme: string;
  total_days: number;
  start_date: string;
  difficulty?: string;
}

/** 今日写作任务 */
export interface TodayWritingTask {
  plan_name: string;
  plan_id: number;
  day_number: number;
  title: string;
  prompt: string;
  is_completed: boolean;
}

// ==================== 写作记录模块 ====================

/** 写作记录实体 */
export interface Writing {
  id: number | null;
  plan_day_id?: number | null;
  title: string;
  content: string;
  word_count: number;
  duration_seconds: number;
  written_date: string;
  created_at?: string | null;
}

/** 保存写作记录请求 */
export interface SaveWritingRequest {
  plan_day_id?: number | null;
  title: string;
  content: string;
  word_count: number;
  duration_seconds: number;
}

/** 写作统计摘要 */
export interface WritingStats {
  total_days: number;
  total_words: number;
  total_duration: number;
  current_streak: number;
  max_streak: number;
  avg_words_per_session: number;
  avg_duration_per_session: number;
}

/** 热力图数据点 */
export interface HeatmapEntry {
  date: string;
  word_count: number;
  count: number;
}

/** 写作记录过滤条件 */
export interface WritingFilter {
  start_date?: string;
  end_date?: string;
  plan_id?: number;
}
