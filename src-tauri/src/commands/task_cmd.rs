//! 任务管理相关 Tauri Commands

use tauri::State;
use std::sync::Mutex;
use rusqlite::Connection;
use crate::errors::AppResult;
use crate::models::task::*;
use crate::db::task_dao;

/// 数据库连接状态
pub type DbState = Mutex<Connection>;

/// 获取所有任务
#[tauri::command]
pub fn get_tasks(db: State<'_, DbState>) -> AppResult<Vec<Task>> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    task_dao::get_all_tasks(&conn)
}

/// 获取今日待办
#[tauri::command]
pub fn get_today_tasks(db: State<'_, DbState>) -> AppResult<Vec<Task>> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    task_dao::get_today_tasks(&conn)
}

/// 创建任务
#[tauri::command]
pub fn create_task(db: State<'_, DbState>, request: CreateTaskRequest) -> AppResult<Task> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    task_dao::create_task(&conn, &request)
}

/// 更新任务
#[tauri::command]
pub fn update_task(db: State<'_, DbState>, request: UpdateTaskRequest) -> AppResult<Task> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    task_dao::update_task(&conn, &request)
}

/// 删除任务
#[tauri::command]
pub fn delete_task(db: State<'_, DbState>, id: i64) -> AppResult<()> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    task_dao::delete_task(&conn, id)
}

/// 切换任务完成状态
#[tauri::command]
pub fn toggle_task(db: State<'_, DbState>, id: i64) -> AppResult<Task> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    task_dao::toggle_task(&conn, id)
}
