//! 批量导入相关 Tauri Commands
//!
//! 支持从文件导入任务和写作计划（JSON / Markdown 格式）。

use tauri::State;
use crate::commands::task_cmd::DbState;
use crate::errors::{AppError, AppResult};
use crate::models::task::*;
use crate::models::plan::*;
use crate::db::{task_dao, plan_dao};

/// 从 JSON 字符串批量导入任务
#[tauri::command]
pub fn import_tasks_json(db: State<'_, DbState>, json_content: String) -> AppResult<Vec<Task>> {
    let conn = db.lock().map_err(|e| AppError::Business(e.to_string()))?;

    let items: Vec<ImportTaskItem> = serde_json::from_str(&json_content)?;
    let requests: Vec<CreateTaskRequest> = items.into_iter().map(|item| {
        CreateTaskRequest {
            title: item.title,
            description: item.description,
            priority: item.priority,
            due_date: item.due_date.or(item.due),
            repeat: item.repeat,
        }
    }).collect();

    task_dao::batch_create_tasks(&conn, &requests)
}

/// 从 JSON 字符串导入写作计划
#[tauri::command]
pub fn import_plan_json(db: State<'_, DbState>, json_content: String) -> AppResult<i64> {
    let conn = db.lock().map_err(|e| AppError::Business(e.to_string()))?;
    let request: ImportPlanRequest = serde_json::from_str(&json_content)?;
    plan_dao::create_plan(&conn, &request)
}

/// 从 Markdown 字符串导入写作计划
///
/// 支持格式：
/// ```markdown
/// # 计划名称
///
/// ## Day 1 - 题目
/// 写作提示内容
///
/// ## Day 2 - 题目
/// 写作提示内容
/// ```
#[tauri::command]
pub fn import_plan_markdown(db: State<'_, DbState>, md_content: String, start_date: String) -> AppResult<i64> {
    let conn = db.lock().map_err(|e| AppError::Business(e.to_string()))?;

    let request = parse_markdown_plan(&md_content, &start_date)?;
    plan_dao::create_plan(&conn, &request)
}

/// 解析 Markdown 格式的写作计划
fn parse_markdown_plan(content: &str, start_date: &str) -> AppResult<ImportPlanRequest> {
    let mut plan_name = String::new();
    let mut days: Vec<ImportPlanDayItem> = Vec::new();
    let mut current_day: Option<(i32, String)> = None;
    let mut current_prompt = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // 解析计划名称（# 标题）
        if trimmed.starts_with("# ") && !trimmed.starts_with("## ") {
            plan_name = trimmed[2..].trim().to_string();
            continue;
        }

        // 解析每日条目（## Day N - 题目）
        if trimmed.starts_with("## ") {
            // 保存上一个 day
            if let Some((day_num, title)) = current_day.take() {
                days.push(ImportPlanDayItem {
                    day: day_num,
                    title,
                    prompt: current_prompt.trim().to_string(),
                });
                current_prompt.clear();
            }

            let header = trimmed[3..].trim();
            // 尝试解析 "Day N - 题目" 格式
            if let Some(parsed) = parse_day_header(header) {
                current_day = Some(parsed);
            } else {
                // 退回序号方式
                let day_num = (days.len() + 1) as i32;
                current_day = Some((day_num, header.to_string()));
            }
            continue;
        }

        // 收集当前 day 的提示内容
        if current_day.is_some() && !trimmed.is_empty() {
            if !current_prompt.is_empty() {
                current_prompt.push('\n');
            }
            current_prompt.push_str(trimmed);
        }
    }

    // 保存最后一个 day
    if let Some((day_num, title)) = current_day {
        days.push(ImportPlanDayItem {
            day: day_num,
            title,
            prompt: current_prompt.trim().to_string(),
        });
    }

    if plan_name.is_empty() {
        plan_name = "导入的写作计划".to_string();
    }

    if days.is_empty() {
        return Err(AppError::MarkdownParse("未找到有效的每日写作条目（需要 ## Day N - 题目 格式）".to_string()));
    }

    Ok(ImportPlanRequest {
        name: plan_name,
        theme: None,
        start_date: start_date.to_string(),
        days,
    })
}

/// 解析 "Day N - 题目" 格式的标题
fn parse_day_header(header: &str) -> Option<(i32, String)> {
    let lower = header.to_lowercase();
    if lower.starts_with("day ") || lower.starts_with("day") {
        let rest = if lower.starts_with("day ") {
            &header[4..]
        } else {
            &header[3..]
        };

        // 查找分隔符 " - " 或 " – " 或 "："
        let (num_str, title) = if let Some(pos) = rest.find(" - ") {
            (&rest[..pos], rest[pos + 3..].trim().to_string())
        } else if let Some(pos) = rest.find(" – ") {
            (&rest[..pos], rest[pos + 5..].trim().to_string())
        } else if let Some(pos) = rest.find("：") {
            (&rest[..pos], rest[pos + 3..].trim().to_string())
        } else if let Some(pos) = rest.find(": ") {
            (&rest[..pos], rest[pos + 2..].trim().to_string())
        } else {
            return None;
        };

        if let Ok(day_num) = num_str.trim().parse::<i32>() {
            return Some((day_num, title));
        }
    }
    None
}
