//! 数据库初始化和迁移
//!
//! 应用启动时创建数据库文件和所有必需的表。

use rusqlite::Connection;
use crate::errors::AppResult;

/// 数据库文件名
pub const DB_NAME: &str = "writedo.db";

/// 初始化数据库，创建所有表
pub fn init_database(conn: &Connection) -> AppResult<()> {
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    create_tasks_table(conn)?;
    create_writing_plans_table(conn)?;
    create_plan_days_table(conn)?;
    create_writings_table(conn)?;

    Ok(())
}

/// 创建任务（待办）表
fn create_tasks_table(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tasks (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            title           TEXT NOT NULL,
            description     TEXT,
            priority        TEXT NOT NULL DEFAULT 'medium',
            due_date        TEXT,
            repeat          TEXT,
            completed       INTEGER NOT NULL DEFAULT 0,
            created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        );"
    )?;
    Ok(())
}

/// 创建写作计划表
fn create_writing_plans_table(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS writing_plans (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            name            TEXT NOT NULL,
            theme           TEXT,
            start_date      TEXT NOT NULL,
            total_days      INTEGER NOT NULL,
            status          TEXT NOT NULL DEFAULT 'active',
            created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        );"
    )?;
    Ok(())
}

/// 创建计划每日条目表
fn create_plan_days_table(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS plan_days (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            plan_id         INTEGER NOT NULL,
            day_number      INTEGER NOT NULL,
            title           TEXT NOT NULL,
            prompt          TEXT NOT NULL,
            scheduled_date  TEXT,
            FOREIGN KEY (plan_id) REFERENCES writing_plans(id) ON DELETE CASCADE
        );"
    )?;
    Ok(())
}

/// 创建写作记录表
fn create_writings_table(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS writings (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            plan_day_id     INTEGER,
            title           TEXT NOT NULL,
            content         TEXT NOT NULL DEFAULT '',
            word_count      INTEGER NOT NULL DEFAULT 0,
            duration_seconds INTEGER NOT NULL DEFAULT 0,
            written_date    TEXT NOT NULL DEFAULT (date('now', 'localtime')),
            created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (plan_day_id) REFERENCES plan_days(id) ON DELETE SET NULL
        );"
    )?;
    Ok(())
}
