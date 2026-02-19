use std::process::Command;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

use super::models::{BdIssue, Issue};

pub struct BeadsClient {
    project_dir: PathBuf,
    beads_dir: PathBuf,
}

impl BeadsClient {
    pub fn new(beads_dir: PathBuf) -> Result<Self> {
        let project_dir = beads_dir
            .parent()
            .ok_or_else(|| anyhow::anyhow!(".beads has no parent dir"))?
            .to_path_buf();
        Ok(BeadsClient {
            project_dir,
            beads_dir,
        })
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

    pub fn beads_dir(&self) -> &Path {
        &self.beads_dir
    }

    pub fn load_issues(&self, label_filter: Option<&str>) -> Result<Vec<Issue>> {
        let mut cmd = Command::new("bd");
        cmd.arg("list")
            .arg("--json")
            .arg("--all")
            .arg("--limit")
            .arg("500");
        cmd.current_dir(&self.project_dir);

        if let Some(label) = label_filter {
            cmd.arg("--label").arg(label);
        }

        let output = cmd
            .output()
            .context("Failed to run `bd`. Is it installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("bd list failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            return Ok(Vec::new());
        }

        let bd_issues: Vec<BdIssue> =
            serde_json::from_str(&stdout).context("Failed to parse bd list JSON")?;

        Ok(bd_issues.into_iter().map(Issue::from).collect())
    }
}
