use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::embedded::EMBEDDED_FILES;
use crate::info_file::InfoFile;

pub fn init() -> Result<()> {
    let info = InfoFile::parse_from_embedded()?;

    fs::create_dir_all("exercises").context("Failed to create exercises/")?;
    fs::create_dir_all("solutions").context("Failed to create solutions/")?;

    for dir in EMBEDDED_FILES.exercise_dirs {
        let dir_path = Path::new("exercises").join(dir.name);
        fs::create_dir_all(&dir_path)?;
        let sol_dir = Path::new("solutions").join(dir.name);
        fs::create_dir_all(&sol_dir)?;
    }

    const INIT_SOLUTION_PLACEHOLDER: &[u8] =
        b"// Solution will appear here after you pass the exercise.\nfn main() {}\n";

    for (i, ex_info) in info.exercises.iter().enumerate() {
        let files = &EMBEDDED_FILES.exercise_files[i];
        let ex_path = ex_info.path();
        let sol_path = ex_info.sol_path();
        fs::write(&ex_path, files.exercise)
            .with_context(|| format!("Failed to write {}", ex_path))?;
        fs::write(&sol_path, INIT_SOLUTION_PLACEHOLDER)
            .with_context(|| format!("Failed to write {}", sol_path))?;
    }

    let cargo_toml = generate_cargo_toml(&info)?;
    fs::write("Cargo.toml", cargo_toml).context("Failed to write Cargo.toml")?;

    let gitignore = "target/\n.axumlings-state.txt\n";
    fs::write(".gitignore", gitignore).context("Failed to write .gitignore")?;

    Ok(())
}

fn generate_cargo_toml(info: &InfoFile) -> Result<String> {
    let mut bins = String::new();
    for ex in &info.exercises {
        let ex_path = ex.path();
        bins.push_str(&format!(
            "[[bin]]\nname = \"{}\"\npath = \"{}\"\n\n",
            ex.name, ex_path
        ));
    }

    let template = r#"[package]
name = "exercises"
version = "0.1.0"
edition = "2021"
publish = false

"#;

    let deps = r#"
[dependencies]
axum = { version = "0.7", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
tower = { version = "0.4", features = ["util", "limit"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "1"
anyhow = "1"
tower-http = { version = "0.5", features = ["trace", "cors", "timeout"] }
axum-extra = { version = "0.9", features = ["typed-header"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
clap = { version = "4", features = ["derive"] }
"#;

    Ok(format!("{}{}{}", template, bins, deps))
}
