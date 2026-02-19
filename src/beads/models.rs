use serde::{Deserialize, Serialize};
use std::fmt;

fn default_priority() -> u8 {
    2
}

/// Intermediate struct matching `bd list --json` output exactly.
#[derive(Debug, Deserialize)]
pub struct BdIssue {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub status: String,
    #[serde(default = "default_priority")]
    pub priority: u8,
    #[serde(default)]
    pub issue_type: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub dependency_count: u32,
    #[serde(default)]
    pub dependent_count: u32,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub priority: Priority,
    pub issue_type: IssueType,
    pub labels: Vec<String>,
    pub assignee: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub dependency_count: u32,
    pub dependent_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Open,
    InProgress,
    Closed,
    Blocked,
    Deferred,
    Unknown,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Open => write!(f, "open"),
            Status::InProgress => write!(f, "in_progress"),
            Status::Closed => write!(f, "closed"),
            Status::Blocked => write!(f, "blocked"),
            Status::Deferred => write!(f, "deferred"),
            Status::Unknown => write!(f, "unknown"),
        }
    }
}

impl Status {
    pub fn from_str(s: &str) -> Status {
        match s {
            "open" => Status::Open,
            "in_progress" => Status::InProgress,
            "closed" | "completed" => Status::Closed,
            "blocked" => Status::Blocked,
            "deferred" => Status::Deferred,
            _ => Status::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Priority(pub u8);

impl Priority {
    pub fn new(p: u8) -> Self {
        Priority(p.min(4))
    }

    pub fn label(&self) -> &'static str {
        match self.0 {
            0 => "P0",
            1 => "P1",
            2 => "P2",
            3 => "P3",
            4 => "P4",
            _ => "P?",
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority(2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueType {
    Task,
    Bug,
    Feature,
    Epic,
    Chore,
    Other,
}

impl fmt::Display for IssueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IssueType::Task => write!(f, "task"),
            IssueType::Bug => write!(f, "bug"),
            IssueType::Feature => write!(f, "feature"),
            IssueType::Epic => write!(f, "epic"),
            IssueType::Chore => write!(f, "chore"),
            IssueType::Other => write!(f, "other"),
        }
    }
}

impl From<BdIssue> for Issue {
    fn from(bd: BdIssue) -> Self {
        let status = Status::from_str(&bd.status);
        let priority = Priority::new(bd.priority);
        let issue_type = match bd.issue_type.as_str() {
            "bug" => IssueType::Bug,
            "feature" => IssueType::Feature,
            "epic" => IssueType::Epic,
            "task" => IssueType::Task,
            "chore" => IssueType::Chore,
            _ => IssueType::Other,
        };
        let assignee = if bd.owner.is_empty() {
            None
        } else {
            Some(bd.owner)
        };
        let description = if bd.description.is_empty() {
            None
        } else {
            Some(bd.description)
        };

        Issue {
            id: bd.id,
            title: bd.title,
            description,
            status,
            priority,
            issue_type,
            labels: bd.labels,
            assignee,
            created_at: bd.created_at,
            updated_at: bd.updated_at,
            dependency_count: bd.dependency_count,
            dependent_count: bd.dependent_count,
        }
    }
}

impl Issue {
    pub fn is_blocked(&self) -> bool {
        self.dependency_count > 0
    }
}
