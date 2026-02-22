//! 写作记录数据模型

use serde::{Deserialize, Serialize};

/// 写作记录实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Writing {
    pub id: Option<i64>,
    /// 关联的计划日条目ID（可选，自由写作时为空）
    pub plan_day_id: Option<i64>,
    pub title: String,
    pub content: String,
    /// 字数统计
    pub word_count: i32,
    /// 写作时长（秒）
    pub duration_seconds: i32,
    /// 写作日期
    pub written_date: String,
    pub created_at: Option<String>,
}

/// 保存写作记录的请求
#[derive(Debug, Deserialize)]
pub struct SaveWritingRequest {
    /// 关联的计划日条目ID（可选）
    pub plan_day_id: Option<i64>,
    pub title: String,
    pub content: String,
    pub word_count: i32,
    pub duration_seconds: i32,
}

/// 写作统计摘要
#[derive(Debug, Clone, Serialize)]
pub struct WritingStats {
    /// 总写作天数
    pub total_days: i32,
    /// 总字数
    pub total_words: i64,
    /// 总写作时长（秒）
    pub total_duration: i64,
    /// 当前连续天数
    pub current_streak: i32,
    /// 最长连续天数
    pub max_streak: i32,
    /// 平均每次字数
    pub avg_words_per_session: f64,
    /// 平均每次时长（秒）
    pub avg_duration_per_session: f64,
}

/// 写作热力图数据点
#[derive(Debug, Clone, Serialize)]
pub struct HeatmapEntry {
    pub date: String,
    pub word_count: i32,
    /// 写作次数
    pub count: i32,
}

/// 按日期查询写作记录的过滤条件
#[derive(Debug, Deserialize)]
pub struct WritingFilter {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub plan_id: Option<i64>,
}
