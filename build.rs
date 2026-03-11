use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(serde::Deserialize)]
struct InfoFile {
    exercises: Vec<ExerciseEntry>,
}

#[derive(serde::Deserialize)]
struct ExerciseEntry {
    name: String,
    dir: Option<String>,
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let info_path = Path::new(&manifest_dir).join("info.toml");
    let info_content = fs::read_to_string(&info_path).expect("Failed to read info.toml");

    let info: InfoFile = toml::from_str(&info_content).expect("Failed to parse info.toml");

    let mut dirs: Vec<String> = info
        .exercises
        .iter()
        .map(|e| e.dir.clone().unwrap_or_default())
        .collect();
    dirs.sort();
    dirs.dedup();

    let dir_inds: Vec<usize> = info
        .exercises
        .iter()
        .map(|ex| {
            let dir = ex.dir.as_deref().unwrap_or("");
            dirs.iter().position(|d| d == dir).unwrap_or(0)
        })
        .collect();

    let mut output = String::new();

    // Generate static for each exercise and solution
    for (i, ex) in info.exercises.iter().enumerate() {
        let dir = ex.dir.as_deref().unwrap_or("");
        let ex_path = if dir.is_empty() {
            format!("/exercises/{}.rs", ex.name)
        } else {
            format!("/exercises/{}/{}.rs", dir, ex.name)
        };
        let sol_path = if dir.is_empty() {
            format!("/.solutions/{}.rs", ex.name)
        } else {
            format!("/.solutions/{}/{}.rs", dir, ex.name)
        };
        output.push_str(&format!(
            "static EXERCISE_{}: &[u8] = include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"{}\"));\n",
            i, ex_path
        ));
        output.push_str(&format!(
            "static SOLUTION_{}: &[u8] = include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"{}\"));\n",
            i, sol_path
        ));
    }

    output.push_str("\npub struct ExerciseDir {\n    pub name: &'static str,\n}\n\n");
    output.push_str("pub struct ExerciseFiles {\n");
    output.push_str("    pub exercise: &'static [u8],\n");
    output.push_str("    pub solution: &'static [u8],\n");
    output.push_str("    pub dir_ind: usize,\n");
    output.push_str("}\n\n");
    output.push_str("pub struct EmbeddedFiles {\n");
    output.push_str("    pub info_file: &'static str,\n");
    output.push_str("    pub exercise_dirs: &'static [ExerciseDir],\n");
    output.push_str("    pub exercise_files: &'static [ExerciseFiles],\n");
    output.push_str("}\n\n");
    output.push_str("pub static EMBEDDED_FILES: EmbeddedFiles = EmbeddedFiles {\n");
    output.push_str(
        "    info_file: include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/info.toml\")),\n",
    );
    output.push_str("    exercise_dirs: &[\n");
    for dir in &dirs {
        output.push_str(&format!("        ExerciseDir {{ name: \"{}\" }},\n", dir));
    }
    output.push_str("    ],\n");
    output.push_str("    exercise_files: &[\n");
    for (i, dir_ind) in dir_inds.iter().enumerate() {
        output.push_str(&format!(
            "        ExerciseFiles {{ exercise: EXERCISE_{}, solution: SOLUTION_{}, dir_ind: {} }},\n",
            i, i, dir_ind
        ));
    }
    output.push_str("    ],\n");
    output.push_str("};\n");

    let out_path = Path::new(&out_dir).join("embedded.rs");
    fs::File::create(&out_path)
        .expect("Failed to create embedded.rs")
        .write_all(output.as_bytes())
        .expect("Failed to write embedded.rs");

    println!("cargo:rerun-if-changed=info.toml");
    for ex in &info.exercises {
        let dir = ex.dir.as_deref().unwrap_or("");
        let ex_path = if dir.is_empty() {
            format!("exercises/{}.rs", ex.name)
        } else {
            format!("exercises/{}/{}.rs", dir, ex.name)
        };
        let sol_path = if dir.is_empty() {
            format!(".solutions/{}.rs", ex.name)
        } else {
            format!(".solutions/{}/{}.rs", dir, ex.name)
        };
        println!("cargo:rerun-if-changed={}", ex_path);
        println!("cargo:rerun-if-changed={}", sol_path);
    }
}
