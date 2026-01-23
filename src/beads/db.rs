use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use super::models::{Issue, IssueType, Priority, Status};

pub struct BeadsDb {
    db_path: PathBuf,
}

impl BeadsDb {
    pub fn new(beads_dir: impl AsRef<Path>) -> Result<Self> {
        let db_path = beads_dir.as_ref().join("beads.db");

        if !db_path.exists() {
            anyhow::bail!("Database not found: {}", db_path.display());
        }

        Ok(BeadsDb { db_path })
    }

    pub fn find_beads_dir() -> Result<PathBuf> {
        let mut current = std::env::current_dir()?;

        loop {
            let beads_dir = current.join(".beads");
            if beads_dir.is_dir() {
                return Ok(beads_dir);
            }

            if !current.pop() {
                anyhow::bail!("Not in a beads project (no .beads directory found)");
            }
        }
    }

    fn connect(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
            .with_context(|| format!("Failed to open database: {}", self.db_path.display()))
    }

    pub fn load_issues(&self, label_filter: Option<&str>) -> Result<Vec<Issue>> {
        let conn = self.connect()?;

        let label_filter_sql = if let Some(label) = label_filter {
            format!(
                "AND EXISTS (
                    SELECT 1 FROM labels
                    WHERE labels.issue_id = i.id
                    AND labels.label = '{}'
                )",
                label
            )
        } else {
            String::new()
        };

        let query = format!(
            "SELECT
                i.id,
                i.title,
                i.description,
                i.status,
                COALESCE(i.priority, 2) as priority,
                i.issue_type,
                i.assignee,
                i.created_at,
                i.updated_at,
                COALESCE(
                    (SELECT json_group_array(l.label)
                     FROM labels l
                     WHERE l.issue_id = i.id),
                    '[]'
                ) as labels,
                COALESCE(
                    (SELECT json_group_array(d.depends_on_id)
                     FROM dependencies d
                     WHERE d.issue_id = i.id),
                    '[]'
                ) as blocked_by,
                COALESCE(
                    (SELECT json_group_array(d.issue_id)
                     FROM dependencies d
                     WHERE d.depends_on_id = i.id),
                    '[]'
                ) as blocks
            FROM issues i
            WHERE i.deleted_at IS NULL
              {}
            ORDER BY i.priority ASC, i.updated_at DESC",
            label_filter_sql
        );

        let mut stmt = conn.prepare(&query)?;
        let issues = stmt
            .query_map([], |row| {
                let status_str: String = row.get(3)?;
                let priority_val: i64 = row.get(4)?;
                let type_str: String = row.get(5)?;
                let labels_json: String = row.get(9)?;
                let blocked_by_json: String = row.get(10)?;
                let blocks_json: String = row.get(11)?;

                Ok(Issue {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: Status::from_str(&status_str).unwrap_or(Status::Open),
                    priority: Priority::new(priority_val as u8),
                    issue_type: match type_str.as_str() {
                        "bug" => IssueType::Bug,
                        "feature" => IssueType::Feature,
                        "epic" => IssueType::Epic,
                        _ => IssueType::Task,
                    },
                    assignee: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                    labels: serde_json::from_str(&labels_json).unwrap_or_default(),
                    blocked_by: serde_json::from_str(&blocked_by_json).unwrap_or_default(),
                    blocks: serde_json::from_str(&blocks_json).unwrap_or_default(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(issues)
    }

    pub fn get_issue(&self, id: &str) -> Result<Option<Issue>> {
        let issues = self.load_issues(None)?;
        Ok(issues.into_iter().find(|i| i.id == id))
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }
}
