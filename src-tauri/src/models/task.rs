//! 任务（待办事项）数据模型

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// 任务优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "high" => Priority::High,
            "medium" => Priority::Medium,
            _ => Priority::Low,
        }
    }
}

/// 重复类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RepeatType {
    Daily,
    Weekly,
    Monthly,
}

impl RepeatType {
    pub fn as_str(&self) -> &str {
        match self {
            RepeatType::Daily => "daily",
            RepeatType::Weekly => "weekly",
            RepeatType::Monthly => "monthly",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "daily" => Some(RepeatType::Daily),
            "weekly" => Some(RepeatType::Weekly),
            "monthly" => Some(RepeatType::Monthly),
            _ => None,
        }
    }
}

/// 任务实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i64>,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: Priority,
    pub due_date: Option<NaiveDate>,
    #[serde(default)]
    pub repeat: Option<RepeatType>,
    #[serde(default)]
    pub completed: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

fn default_priority() -> Priority {
    Priority::Medium
}

/// 创建任务的请求参数
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: Priority,
    pub due_date: Option<String>,
    #[serde(default)]
    pub repeat: Option<RepeatType>,
}

/// 更新任务的请求参数
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub due_date: Option<String>,
    pub repeat: Option<RepeatType>,
    pub completed: Option<bool>,
}

/// 批量导入任务的单条数据
#[derive(Debug, Deserialize)]
pub struct ImportTaskItem {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: Priority,
    pub due_date: Option<String>,
    #[serde(default, alias = "due")]
    pub due: Option<String>,
    #[serde(default)]
    pub repeat: Option<RepeatType>,
}
