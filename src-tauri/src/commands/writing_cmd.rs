//! 写作记录相关 Tauri Commands

use tauri::State;
use crate::commands::task_cmd::DbState;
use crate::errors::AppResult;
use crate::models::writing::*;
use crate::db::writing_dao;

/// 保存写作记录
#[tauri::command]
pub fn save_writing(db: State<'_, DbState>, request: SaveWritingRequest) -> AppResult<Writing> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    writing_dao::save_writing(&conn, &request)
}

/// 获取写作记录列表
#[tauri::command]
pub fn get_writings(db: State<'_, DbState>, filter: WritingFilter) -> AppResult<Vec<Writing>> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    writing_dao::get_writings(&conn, &filter)
}

/// 获取写作统计
#[tauri::command]
pub fn get_writing_stats(db: State<'_, DbState>) -> AppResult<WritingStats> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    writing_dao::get_writing_stats(&conn)
}

/// 获取热力图数据
#[tauri::command]
pub fn get_heatmap(db: State<'_, DbState>) -> AppResult<Vec<HeatmapEntry>> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    writing_dao::get_heatmap_data(&conn)
}

/// 获取单篇写作记录详情
#[tauri::command]
pub fn get_writing_detail(db: State<'_, DbState>, id: i64) -> AppResult<Writing> {
    let conn = db.lock().map_err(|e| crate::errors::AppError::Business(e.to_string()))?;
    writing_dao::get_writing_by_id(&conn, id)
}
