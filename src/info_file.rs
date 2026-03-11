use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct ExerciseInfo {
    pub name: String,
    pub dir: Option<String>,
    #[serde(default = "default_true")]
    pub test: bool,
    #[serde(default)]
    pub strict_clippy: bool,
    pub hint: String,
}

fn default_true() -> bool {
    true
}

impl ExerciseInfo {
    pub fn path(&self) -> String {
        if let Some(ref dir) = self.dir {
            format!("exercises/{}/{}.rs", dir, self.name)
        } else {
            format!("exercises/{}.rs", self.name)
        }
    }

    pub fn sol_path(&self) -> String {
        if let Some(ref dir) = self.dir {
            format!("solutions/{}/{}.rs", dir, self.name)
        } else {
            format!("solutions/{}.rs", self.name)
        }
    }
}

#[derive(Deserialize)]
pub struct InfoFile {
    pub format_version: u8,
    pub welcome_message: Option<String>,
    pub final_message: Option<String>,
    pub exercises: Vec<ExerciseInfo>,
}

impl InfoFile {
    pub fn parse_from_embedded() -> Result<Self> {
        let content = crate::embedded::EMBEDDED_FILES.info_file;
        Self::parse_str(content)
    }

    pub fn parse_str(content: &str) -> Result<Self> {
        let info: InfoFile = toml::from_str(content).context("Failed to parse info.toml")?;
        if info.exercises.is_empty() {
            bail!("No exercises found in info.toml");
        }
        Ok(info)
    }

    pub fn parse_from_disk() -> Result<Self> {
        let content = fs::read_to_string("info.toml").context("Failed to read info.toml")?;
        Self::parse_str(&content)
    }
}
