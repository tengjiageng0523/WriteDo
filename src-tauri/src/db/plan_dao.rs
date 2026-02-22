//! 写作计划数据库操作

use rusqlite::{params, Connection};
use chrono::NaiveDate;
use crate::errors::{AppError, AppResult};
use crate::models::plan::*;

/// 创建写作计划（含每日条目）
pub fn create_plan(conn: &Connection, plan: &ImportPlanRequest) -> AppResult<i64> {
    conn.execute(
        "INSERT INTO writing_plans (name, theme, start_date, total_days) VALUES (?1, ?2, ?3, ?4)",
        params![plan.name, plan.theme, plan.start_date, plan.days.len() as i32],
    )?;

    let plan_id = conn.last_insert_rowid();
    let start = NaiveDate::parse_from_str(&plan.start_date, "%Y-%m-%d")
        .map_err(|e| AppError::Business(format!("日期格式无效: {}", e)))?;

    for day_item in &plan.days {
        let scheduled = start + chrono::Duration::days((day_item.day - 1) as i64);
        conn.execute(
            "INSERT INTO plan_days (plan_id, day_number, title, prompt, scheduled_date) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![plan_id, day_item.day, day_item.title, day_item.prompt, scheduled.to_string()],
        )?;
    }

    Ok(plan_id)
}

/// 获取所有写作计划（不含每日条目详情）
pub fn get_all_plans(conn: &Connection) -> AppResult<Vec<WritingPlan>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, theme, start_date, total_days, status, created_at
         FROM writing_plans ORDER BY created_at DESC"
    )?;

    let plans = stmt.query_map([], |row| {
        Ok(WritingPlan {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            theme: row.get(2)?,
            start_date: row.get(3)?,
            total_days: row.get(4)?,
            status: PlanStatus::from_str(&row.get::<_, String>(5)?),
            created_at: row.get(6)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;

    Ok(plans)
}

/// 获取计划详情（含每日条目 + 写作完成状态）
pub fn get_plan_with_days(conn: &Connection, plan_id: i64) -> AppResult<PlanWithDays> {
    let plan = get_plan_by_id(conn, plan_id)?;

    // LEFT JOIN writings 获取每日写作状态
    let mut stmt = conn.prepare(
        "SELECT pd.id, pd.plan_id, pd.day_number, pd.title, pd.prompt, pd.scheduled_date,
                CASE WHEN w.id IS NOT NULL THEN 1 ELSE 0 END as is_completed,
                COALESCE(w.word_count, 0) as word_count,
                w.title as writing_title,
                w.id as writing_id
         FROM plan_days pd
         LEFT JOIN writings w ON w.plan_day_id = pd.id
         WHERE pd.plan_id = ?1
         ORDER BY pd.day_number ASC"
    )?;
    let days = stmt.query_map(params![plan_id], |row| {
        Ok(PlanDayDetail {
            id: row.get(0)?,
            plan_id: row.get(1)?,
            day_number: row.get(2)?,
            title: row.get(3)?,
            prompt: row.get(4)?,
            scheduled_date: row.get(5)?,
            is_completed: row.get::<_, i32>(6)? != 0,
            word_count: row.get(7)?,
            writing_title: row.get(8)?,
            writing_id: row.get(9)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;

    let completed_days = days.iter().filter(|d| d.is_completed).count() as i32;

    Ok(PlanWithDays {
        plan,
        days,
        completed_days,
    })
}

/// 根据 ID 获取计划
fn get_plan_by_id(conn: &Connection, id: i64) -> AppResult<WritingPlan> {
    let mut stmt = conn.prepare(
        "SELECT id, name, theme, start_date, total_days, status, created_at
         FROM writing_plans WHERE id = ?1"
    )?;

    stmt.query_row(params![id], |row| {
        Ok(WritingPlan {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            theme: row.get(2)?,
            start_date: row.get(3)?,
            total_days: row.get(4)?,
            status: PlanStatus::from_str(&row.get::<_, String>(5)?),
            created_at: row.get(6)?,
        })
    }).map_err(|_| AppError::NotFound(format!("计划 ID {} 不存在", id)))
}

/// 获取今日写作任务
pub fn get_today_writing_task(conn: &Connection) -> AppResult<Option<TodayWritingTask>> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let mut stmt = conn.prepare(
        "SELECT wp.name, wp.id, pd.day_number, pd.title, pd.prompt, pd.id
         FROM plan_days pd
         INNER JOIN writing_plans wp ON wp.id = pd.plan_id
         WHERE pd.scheduled_date = ?1 AND wp.status = 'active'
         LIMIT 1"
    )?;

    let result = stmt.query_row(params![today], |row| {
        let plan_day_id: i64 = row.get(5)?;
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            plan_day_id,
        ))
    });

    match result {
        Ok((plan_name, plan_id, day_number, title, prompt, plan_day_id)) => {
            // 检查今天是否已写作
            let is_completed: bool = conn.query_row(
                "SELECT COUNT(*) FROM writings WHERE plan_day_id = ?1",
                params![plan_day_id],
                |row| row.get::<_, i32>(0),
            )? > 0;

            Ok(Some(TodayWritingTask {
                plan_name,
                plan_id,
                day_number,
                title,
                prompt,
                is_completed,
            }))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AppError::Database(e)),
    }
}

/// 更新计划基本信息
pub fn update_plan(conn: &Connection, req: &UpdatePlanRequest) -> AppResult<WritingPlan> {
    let mut updates = Vec::new();
    let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref name) = req.name {
        updates.push("name = ?");
        values.push(Box::new(name.clone()));
    }
    if let Some(ref theme) = req.theme {
        updates.push("theme = ?");
        values.push(Box::new(theme.clone()));
    }
    if let Some(ref start_date) = req.start_date {
        updates.push("start_date = ?");
        values.push(Box::new(start_date.clone()));
    }

    if updates.is_empty() {
        return get_plan_by_id(conn, req.id);
    }

    values.push(Box::new(req.id));
    let sql = format!(
        "UPDATE writing_plans SET {} WHERE id = ?",
        updates.join(", ")
    );

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
    let affected = conn.execute(&sql, params_refs.as_slice())?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("计划 ID {} 不存在", req.id)));
    }
    get_plan_by_id(conn, req.id)
}

/// 更新计划状态
pub fn update_plan_status(conn: &Connection, plan_id: i64, status: PlanStatus) -> AppResult<()> {
    let affected = conn.execute(
        "UPDATE writing_plans SET status = ?1 WHERE id = ?2",
        params![status.as_str(), plan_id],
    )?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("计划 ID {} 不存在", plan_id)));
    }
    Ok(())
}

/// 删除计划（级联删除每日条目）
pub fn delete_plan(conn: &Connection, plan_id: i64) -> AppResult<()> {
    let affected = conn.execute("DELETE FROM writing_plans WHERE id = ?1", params![plan_id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("计划 ID {} 不存在", plan_id)));
    }
    Ok(())
}

/// 更新每日条目
pub fn update_plan_day(conn: &Connection, req: &UpdatePlanDayRequest) -> AppResult<()> {
    let mut updates = Vec::new();
    let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref title) = req.title {
        updates.push("title = ?");
        values.push(Box::new(title.clone()));
    }
    if let Some(ref prompt) = req.prompt {
        updates.push("prompt = ?");
        values.push(Box::new(prompt.clone()));
    }

    if updates.is_empty() {
        return Ok(());
    }

    values.push(Box::new(req.id));
    let sql = format!(
        "UPDATE plan_days SET {} WHERE id = ?",
        updates.join(", ")
    );

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
    let affected = conn.execute(&sql, params_refs.as_slice())?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("条目 ID {} 不存在", req.id)));
    }
    Ok(())
}

/// 删除每日条目（删除后自动重排序号 + 更新 total_days）
pub fn delete_plan_day(conn: &Connection, day_id: i64) -> AppResult<()> {
    // 先获取 plan_id
    let plan_id: i64 = conn.query_row(
        "SELECT plan_id FROM plan_days WHERE id = ?1",
        params![day_id],
        |row| row.get(0),
    ).map_err(|_| AppError::NotFound(format!("条目 ID {} 不存在", day_id)))?;

    // 删除条目
    conn.execute("DELETE FROM plan_days WHERE id = ?1", params![day_id])?;

    // 按原有顺序重新编号
    let mut stmt = conn.prepare(
        "SELECT id FROM plan_days WHERE plan_id = ?1 ORDER BY day_number ASC"
    )?;
    let ids: Vec<i64> = stmt.query_map(params![plan_id], |row| row.get(0))?
        .filter_map(|r| r.ok()).collect();

    for (i, id) in ids.iter().enumerate() {
        conn.execute(
            "UPDATE plan_days SET day_number = ?1 WHERE id = ?2",
            params![(i + 1) as i32, id],
        )?;
    }

    // 更新 total_days
    conn.execute(
        "UPDATE writing_plans SET total_days = ?1 WHERE id = ?2",
        params![ids.len() as i32, plan_id],
    )?;

    Ok(())
}

/// 在指定位置插入新条目（后续条目自动后移）
pub fn add_plan_day(conn: &Connection, req: &AddPlanDayRequest) -> AppResult<()> {
    let plan_id = req.plan_id;
    let insert_at = req.day_number; // 要插入的位置

    // 获取计划的 start_date 来计算 scheduled_date
    let start_date: String = conn.query_row(
        "SELECT start_date FROM writing_plans WHERE id = ?1",
        params![plan_id],
        |row| row.get(0),
    ).map_err(|_| AppError::NotFound(format!("计划 ID {} 不存在", plan_id)))?;

    // 把 >= insert_at 的条目 day_number 全部 +1
    conn.execute(
        "UPDATE plan_days SET day_number = day_number + 1 WHERE plan_id = ?1 AND day_number >= ?2",
        params![plan_id, insert_at],
    )?;

    // 使用自定义日期或自动计算
    let scheduled = req.scheduled_date.clone().unwrap_or_else(|| {
        NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
            .map(|d| d + chrono::Duration::days((insert_at - 1) as i64))
            .map(|d| d.to_string())
            .unwrap_or_default()
    });

    // 插入新条目
    conn.execute(
        "INSERT INTO plan_days (plan_id, day_number, title, prompt, scheduled_date) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![plan_id, insert_at, req.title, req.prompt, scheduled],
    )?;

    // 更新 total_days
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM plan_days WHERE plan_id = ?1",
        params![plan_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "UPDATE writing_plans SET total_days = ?1 WHERE id = ?2",
        params![count, plan_id],
    )?;

    Ok(())
}

