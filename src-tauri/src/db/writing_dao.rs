//! 写作记录数据库操作

use rusqlite::{params, Connection};
use crate::errors::{AppError, AppResult};
use crate::models::writing::*;

/// 保存写作记录
pub fn save_writing(conn: &Connection, req: &SaveWritingRequest) -> AppResult<Writing> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    conn.execute(
        "INSERT INTO writings (plan_day_id, title, content, word_count, duration_seconds, written_date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            req.plan_day_id,
            req.title,
            req.content,
            req.word_count,
            req.duration_seconds,
            today,
        ],
    )?;

    let id = conn.last_insert_rowid();
    get_writing_by_id(conn, id)
}

/// 根据 ID 获取写作记录
pub fn get_writing_by_id(conn: &Connection, id: i64) -> AppResult<Writing> {
    let mut stmt = conn.prepare(
        "SELECT id, plan_day_id, title, content, word_count, duration_seconds, written_date, created_at
         FROM writings WHERE id = ?1"
    )?;

    stmt.query_row(params![id], |row| {
        Ok(Writing {
            id: Some(row.get(0)?),
            plan_day_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            word_count: row.get(4)?,
            duration_seconds: row.get(5)?,
            written_date: row.get(6)?,
            created_at: row.get(7)?,
        })
    }).map_err(|_| AppError::NotFound(format!("写作记录 ID {} 不存在", id)))
}

/// 获取写作记录列表（支持过滤）
pub fn get_writings(conn: &Connection, filter: &WritingFilter) -> AppResult<Vec<Writing>> {
    let mut sql = String::from(
        "SELECT w.id, w.plan_day_id, w.title, w.content, w.word_count, w.duration_seconds, w.written_date, w.created_at
         FROM writings w"
    );
    let mut conditions = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref plan_id) = filter.plan_id {
        sql.push_str(" INNER JOIN plan_days pd ON pd.id = w.plan_day_id");
        conditions.push(format!("pd.plan_id = ?{}", param_values.len() + 1));
        param_values.push(Box::new(*plan_id));
    }

    if let Some(ref start) = filter.start_date {
        conditions.push(format!("w.written_date >= ?{}", param_values.len() + 1));
        param_values.push(Box::new(start.clone()));
    }
    if let Some(ref end) = filter.end_date {
        conditions.push(format!("w.written_date <= ?{}", param_values.len() + 1));
        param_values.push(Box::new(end.clone()));
    }

    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }
    sql.push_str(" ORDER BY w.written_date DESC, w.created_at DESC");

    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    let writings = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(Writing {
            id: Some(row.get(0)?),
            plan_day_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            word_count: row.get(4)?,
            duration_seconds: row.get(5)?,
            written_date: row.get(6)?,
            created_at: row.get(7)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;

    Ok(writings)
}

/// 获取写作统计
pub fn get_writing_stats(conn: &Connection) -> AppResult<WritingStats> {
    let total_days: i32 = conn.query_row(
        "SELECT COUNT(DISTINCT written_date) FROM writings",
        [], |row| row.get(0),
    )?;

    let total_words: i64 = conn.query_row(
        "SELECT COALESCE(SUM(word_count), 0) FROM writings",
        [], |row| row.get(0),
    )?;

    let total_duration: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_seconds), 0) FROM writings",
        [], |row| row.get(0),
    )?;

    let total_sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM writings",
        [], |row| row.get(0),
    )?;

    let avg_words = if total_sessions > 0 { total_words as f64 / total_sessions as f64 } else { 0.0 };
    let avg_duration = if total_sessions > 0 { total_duration as f64 / total_sessions as f64 } else { 0.0 };

    // 计算连续天数
    let (current_streak, max_streak) = calculate_streaks(conn)?;

    Ok(WritingStats {
        total_days,
        total_words,
        total_duration,
        current_streak,
        max_streak,
        avg_words_per_session: avg_words,
        avg_duration_per_session: avg_duration,
    })
}

/// 计算连续写作天数
fn calculate_streaks(conn: &Connection) -> AppResult<(i32, i32)> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT written_date FROM writings ORDER BY written_date DESC"
    )?;

    let dates: Vec<String> = stmt.query_map([], |row| {
        row.get::<_, String>(0)
    })?.collect::<Result<Vec<_>, _>>()?;

    if dates.is_empty() {
        return Ok((0, 0));
    }

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut current_streak = 0;
    let mut max_streak = 0;
    let mut streak = 0;

    for (i, date_str) in dates.iter().enumerate() {
        if i == 0 {
            // 检查最近一次写作是否是今天或昨天
            if date_str == &today
                || date_str == &(chrono::Local::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string()
            {
                streak = 1;
            } else {
                // 最近写作不是今天/昨天，当前连续中断
                streak = 1; // 仍算一天
                max_streak = 1;
                // 继续扫描计算最大连续
                current_streak = 0;
            }
        } else {
            let prev = chrono::NaiveDate::parse_from_str(&dates[i - 1], "%Y-%m-%d");
            let curr = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
            if let (Ok(p), Ok(c)) = (prev, curr) {
                if (p - c).num_days() == 1 {
                    streak += 1;
                } else {
                    if current_streak == 0 && i <= dates.len() {
                        current_streak = streak;
                    }
                    max_streak = max_streak.max(streak);
                    streak = 1;
                }
            }
        }
    }

    // 最终更新
    if current_streak == 0 {
        current_streak = streak;
    }
    max_streak = max_streak.max(streak);

    Ok((current_streak, max_streak))
}

/// 获取热力图数据（最近一年）
pub fn get_heatmap_data(conn: &Connection) -> AppResult<Vec<HeatmapEntry>> {
    let one_year_ago = (chrono::Local::now() - chrono::Duration::days(365))
        .format("%Y-%m-%d").to_string();

    let mut stmt = conn.prepare(
        "SELECT written_date, COALESCE(SUM(word_count), 0), COUNT(*)
         FROM writings
         WHERE written_date >= ?1
         GROUP BY written_date
         ORDER BY written_date ASC"
    )?;

    let entries = stmt.query_map(params![one_year_ago], |row| {
        Ok(HeatmapEntry {
            date: row.get(0)?,
            word_count: row.get(1)?,
            count: row.get(2)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;

    Ok(entries)
}
