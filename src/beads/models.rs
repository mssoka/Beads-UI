use serde::{Deserialize, Serialize};
use std::fmt;

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
    pub blocked_by: Vec<String>,
    pub blocks: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Open,
    InProgress,
    Closed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Open => write!(f, "open"),
            Status::InProgress => write!(f, "in_progress"),
            Status::Closed => write!(f, "closed"),
        }
    }
}

impl Status {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "open" => Some(Status::Open),
            "in_progress" => Some(Status::InProgress),
            "closed" => Some(Status::Closed),
            _ => None,
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
}

impl fmt::Display for IssueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IssueType::Task => write!(f, "task"),
            IssueType::Bug => write!(f, "bug"),
            IssueType::Feature => write!(f, "feature"),
            IssueType::Epic => write!(f, "epic"),
        }
    }
}

impl Issue {
    pub fn is_blocked(&self) -> bool {
        !self.blocked_by.is_empty()
    }

}
