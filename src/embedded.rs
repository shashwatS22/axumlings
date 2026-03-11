include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

impl EmbeddedFiles {
    pub fn write_exercise_to_disk(&self, exercise_ind: usize, path: &str) -> Result<()> {
        let files = self
            .exercise_files
            .get(exercise_ind)
            .context("Exercise index out of bounds")?;
        let dir = &self.exercise_dirs[files.dir_ind];
        let dir_path = Path::new("exercises").join(dir.name);
        fs::create_dir_all(&dir_path).context("Failed to create exercises dir")?;
        fs::write(path, files.exercise).with_context(|| format!("Failed to write {}", path))?;
        Ok(())
    }

    pub fn write_solution_to_disk(&self, exercise_ind: usize, name: &str) -> Result<String> {
        let files = self
            .exercise_files
            .get(exercise_ind)
            .context("Exercise index out of bounds")?;
        let dir = &self.exercise_dirs[files.dir_ind];
        let dir_path = Path::new("solutions").join(dir.name);
        fs::create_dir_all(&dir_path).context("Failed to create solutions dir")?;
        let solution_path = dir_path.join(format!("{}.rs", name));
        let path_str = solution_path.to_string_lossy().into_owned();
        fs::write(&solution_path, files.solution)
            .with_context(|| format!("Failed to write {}", path_str))?;
        Ok(path_str)
    }
}
