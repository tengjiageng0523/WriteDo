//! 任务（待办事项）数据库操作

use rusqlite::{params, Connection};
use crate::errors::{AppError, AppResult};
use crate::models::task::{Task, CreateTaskRequest, UpdateTaskRequest, Priority, RepeatType};

/// 创建新任务
pub fn create_task(conn: &Connection, req: &CreateTaskRequest) -> AppResult<Task> {
    conn.execute(
        "INSERT INTO tasks (title, description, priority, due_date, repeat) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            req.title,
            req.description,
            req.priority.as_str(),
            req.due_date,
            req.repeat.as_ref().map(|r| r.as_str()),
        ],
    )?;

    let id = conn.last_insert_rowid();
    get_task_by_id(conn, id)
}

/// 根据 ID 获取任务
pub fn get_task_by_id(conn: &Connection, id: i64) -> AppResult<Task> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, priority, due_date, repeat, completed, created_at, updated_at
         FROM tasks WHERE id = ?1"
    )?;

    stmt.query_row(params![id], |row| {
        Ok(Task {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            description: row.get(2)?,
            priority: Priority::from_str(&row.get::<_, String>(3)?),
            due_date: row.get::<_, Option<String>>(4)?.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            repeat: row.get::<_, Option<String>>(5)?.and_then(|s| RepeatType::from_str(&s)),
            completed: row.get::<_, i32>(6)? != 0,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    }).map_err(|_| AppError::NotFound(format!("任务 ID {} 不存在", id)))
}

/// 获取所有任务
pub fn get_all_tasks(conn: &Connection) -> AppResult<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, priority, due_date, repeat, completed, created_at, updated_at
         FROM tasks ORDER BY completed ASC, due_date ASC, priority DESC, created_at DESC"
    )?;

    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            description: row.get(2)?,
            priority: Priority::from_str(&row.get::<_, String>(3)?),
            due_date: row.get::<_, Option<String>>(4)?.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            repeat: row.get::<_, Option<String>>(5)?.and_then(|s| RepeatType::from_str(&s)),
            completed: row.get::<_, i32>(6)? != 0,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;

    Ok(tasks)
}

/// 获取今日待办任务
pub fn get_today_tasks(conn: &Connection) -> AppResult<Vec<Task>> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut stmt = conn.prepare(
        "SELECT id, title, description, priority, due_date, repeat, completed, created_at, updated_at
         FROM tasks
         WHERE (due_date = ?1 OR due_date IS NULL)
         ORDER BY completed ASC, priority DESC, created_at DESC"
    )?;

    let tasks = stmt.query_map(params![today], |row| {
        Ok(Task {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            description: row.get(2)?,
            priority: Priority::from_str(&row.get::<_, String>(3)?),
            due_date: row.get::<_, Option<String>>(4)?.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            repeat: row.get::<_, Option<String>>(5)?.and_then(|s| RepeatType::from_str(&s)),
            completed: row.get::<_, i32>(6)? != 0,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;

    Ok(tasks)
}

/// 更新任务
pub fn update_task(conn: &Connection, req: &UpdateTaskRequest) -> AppResult<Task> {
    // 先确认存在
    get_task_by_id(conn, req.id)?;

    if let Some(ref title) = req.title {
        conn.execute("UPDATE tasks SET title = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2", params![title, req.id])?;
    }
    if let Some(ref desc) = req.description {
        conn.execute("UPDATE tasks SET description = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2", params![desc, req.id])?;
    }
    if let Some(ref priority) = req.priority {
        conn.execute("UPDATE tasks SET priority = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2", params![priority.as_str(), req.id])?;
    }
    if let Some(ref due_date) = req.due_date {
        conn.execute("UPDATE tasks SET due_date = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2", params![due_date, req.id])?;
    }
    if let Some(ref repeat) = req.repeat {
        conn.execute("UPDATE tasks SET repeat = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2", params![repeat.as_str(), req.id])?;
    }
    if let Some(completed) = req.completed {
        conn.execute("UPDATE tasks SET completed = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2", params![completed as i32, req.id])?;
    }

    get_task_by_id(conn, req.id)
}

/// 删除任务
pub fn delete_task(conn: &Connection, id: i64) -> AppResult<()> {
    let affected = conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("任务 ID {} 不存在", id)));
    }
    Ok(())
}

/// 切换任务完成状态
pub fn toggle_task(conn: &Connection, id: i64) -> AppResult<Task> {
    let task = get_task_by_id(conn, id)?;
    let new_completed = !task.completed;
    conn.execute(
        "UPDATE tasks SET completed = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2",
        params![new_completed as i32, id],
    )?;
    get_task_by_id(conn, id)
}

/// 批量创建任务
pub fn batch_create_tasks(conn: &Connection, tasks: &[CreateTaskRequest]) -> AppResult<Vec<Task>> {
    let mut created = Vec::new();
    for req in tasks {
        let task = create_task(conn, req)?;
        created.push(task);
    }
    Ok(created)
}
