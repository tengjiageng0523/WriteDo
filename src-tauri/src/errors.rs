//! 全局错误类型定义
//!
//! 统一的错误处理，所有 Tauri Command 返回 `Result<T, AppError>`，
//! 前端通过 `invoke` 的 catch 获取错误信息。

use serde::Serialize;

/// 应用统一错误类型
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// 数据库操作错误
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),

    /// JSON 序列化/反序列化错误
    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),

    /// 文件操作错误
    #[error("文件操作错误: {0}")]
    Io(#[from] std::io::Error),

    /// 业务逻辑错误
    #[error("{0}")]
    Business(String),

    /// 数据未找到
    #[error("未找到: {0}")]
    NotFound(String),

    /// Markdown 解析错误
    #[error("Markdown 解析错误: {0}")]
    MarkdownParse(String),
}

/// 实现 Serialize 以便 Tauri 可以将错误传递给前端
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// 应用 Result 类型别名
pub type AppResult<T> = Result<T, AppError>;
