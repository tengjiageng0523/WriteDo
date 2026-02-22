//! 写作计划数据模型

use serde::{Deserialize, Serialize};

/// 计划状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlanStatus {
    Active,
    Paused,
    Completed,
}

impl PlanStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PlanStatus::Active => "active",
            PlanStatus::Paused => "paused",
            PlanStatus::Completed => "completed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => PlanStatus::Active,
            "paused" => PlanStatus::Paused,
            "completed" => PlanStatus::Completed,
            _ => PlanStatus::Active,
        }
    }
}

/// 写作计划实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingPlan {
    pub id: Option<i64>,
    pub name: String,
    #[serde(default)]
    pub theme: Option<String>,
    pub start_date: String,
    pub total_days: i32,
    #[serde(default = "default_status")]
    pub status: PlanStatus,
    pub created_at: Option<String>,
}

fn default_status() -> PlanStatus {
    PlanStatus::Active
}

/// 计划中的每日条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDay {
    pub id: Option<i64>,
    pub plan_id: i64,
    pub day_number: i32,
    pub title: String,
    pub prompt: String,
    pub scheduled_date: Option<String>,
}

/// 每日条目详情（含写作完成状态）
#[derive(Debug, Clone, Serialize)]
pub struct PlanDayDetail {
    pub id: i64,
    pub plan_id: i64,
    pub day_number: i32,
    pub title: String,
    pub prompt: String,
    pub scheduled_date: Option<String>,
    /// 是否已完成写作
    pub is_completed: bool,
    /// 写作字数
    pub word_count: i32,
    /// 写作标题
    pub writing_title: Option<String>,
    /// 关联的 writing ID
    pub writing_id: Option<i64>,
}

/// 更新每日条目请求
#[derive(Debug, Deserialize)]
pub struct UpdatePlanDayRequest {
    pub id: i64,
    pub title: Option<String>,
    pub prompt: Option<String>,
}

/// 导入写作计划的请求（JSON 格式）
#[derive(Debug, Deserialize)]
pub struct ImportPlanRequest {
    pub name: String,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(alias = "startDate")]
    pub start_date: String,
    pub days: Vec<ImportPlanDayItem>,
}

/// 导入计划中的每日条目
#[derive(Debug, Deserialize)]
pub struct ImportPlanDayItem {
    pub day: i32,
    pub title: String,
    pub prompt: String,
}

/// 自动生成计划的请求
#[derive(Debug, Deserialize)]
pub struct GeneratePlanRequest {
    /// 计划名称
    pub name: String,
    /// 写作主题类型：叙事/议论/描写/创意/日记/综合
    pub theme: String,
    /// 总天数
    pub total_days: i32,
    /// 开始日期 (YYYY-MM-DD)
    pub start_date: String,
    /// 难度：beginner / intermediate / advanced
    #[serde(default = "default_difficulty")]
    pub difficulty: String,
}

fn default_difficulty() -> String {
    "beginner".to_string()
}

/// 更新写作计划的请求
#[derive(Debug, Deserialize)]
pub struct UpdatePlanRequest {
    pub id: i64,
    pub name: Option<String>,
    pub theme: Option<String>,
    pub start_date: Option<String>,
}

/// 计划 + 每日条目的完整视图（用于前端展示）
#[derive(Debug, Clone, Serialize)]
pub struct PlanWithDays {
    #[serde(flatten)]
    pub plan: WritingPlan,
    pub days: Vec<PlanDayDetail>,
    /// 已完成的天数
    pub completed_days: i32,
}

/// 今日写作任务视图
#[derive(Debug, Clone, Serialize)]
pub struct TodayWritingTask {
    pub plan_name: String,
    pub plan_id: i64,
    pub day_number: i32,
    pub title: String,
    pub prompt: String,
    /// 今天是否已完成写作
    pub is_completed: bool,
}
