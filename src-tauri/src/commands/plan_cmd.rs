//! 写作计划相关 Tauri Commands

use tauri::State;
use crate::commands::task_cmd::DbState;
use crate::errors::AppResult;
use crate::models::plan::*;
use crate::db::plan_dao;
use crate::plan_generator;

/// 获取所有写作计划
#[tauri::command]
pub fn get_plans(db: State<'_, DbState>) -> AppResult<Vec<WritingPlan>> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    plan_dao::get_all_plans(&conn)
}

/// 获取计划详情（含每日条目）
#[tauri::command]
pub fn get_plan_detail(db: State<'_, DbState>, plan_id: i64) -> AppResult<PlanWithDays> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    plan_dao::get_plan_with_days(&conn, plan_id)
}

/// 获取今日写作任务
#[tauri::command]
pub fn get_today_writing(db: State<'_, DbState>) -> AppResult<Option<TodayWritingTask>> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    plan_dao::get_today_writing_task(&conn)
}

/// 导入写作计划（JSON 格式）
#[tauri::command]
pub fn import_plan(db: State<'_, DbState>, request: ImportPlanRequest) -> AppResult<i64> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    plan_dao::create_plan(&conn, &request)
}

/// 自动生成写作计划
#[tauri::command]
pub fn generate_plan(db: State<'_, DbState>, request: GeneratePlanRequest) -> AppResult<i64> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;

    // 生成计划内容
    let import_req = plan_generator::generate_writing_plan(&request)?;

    // 保存到数据库
    plan_dao::create_plan(&conn, &import_req)
}

/// 更新计划状态
#[tauri::command]
pub fn update_plan_status(db: State<'_, DbState>, plan_id: i64, status: String) -> AppResult<()> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    plan_dao::update_plan_status(&conn, plan_id, PlanStatus::from_str(&status))
}

/// 删除计划
#[tauri::command]
pub fn delete_plan(db: State<'_, DbState>, plan_id: i64) -> AppResult<()> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    plan_dao::delete_plan(&conn, plan_id)
}
