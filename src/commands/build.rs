use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::{Write, BufWriter};
use chrono::Local;
use whoami;
use serde_yaml;

#[derive(Deserialize)]
struct AsteriskConfig {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
}

pub fn handle_build() {
    let start_time = Instant::now();
    let wd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Failed to get current directory: {}", e);
            std::process::exit(1);
        }
    };

    let project_name = match get_project_name_from_yml(&wd) {
        Ok(name) => name,
        Err(e) => {
            eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] {}", e);
            return;
        }
    };

    let src_path = wd.join("src").join("main.rs");
    if !src_path.exists() {
        eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] src/main.rs does not exist.");
        return;
    }

    println!("[\x1b[34m\x1b[1mInfo\x1b[0m] Building project '{}'...", project_name);

    let modified_files = Arc::new(Mutex::new(vec![src_path.clone()]));

    let files = modified_files.lock().unwrap();
    files.par_iter().for_each(|file| {
        compile_file(file, &project_name);
    });

    let duration = start_time.elapsed();
    println!("[\x1b[32m\x1b[1mSuccess\x1b[0m] Project '{}' built successfully in {:.2?}.", project_name, duration);
}

fn get_project_name_from_yml(wd: &Path) -> Result<String, String> {
    let yml_path = wd.join("Asterisk.yml");
    let content = fs::read_to_string(&yml_path)
        .map_err(|e| format!("Failed to read Asterisk.yml: {}", e))?;

    let config: AsteriskConfig = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse Asterisk.yml: {}", e))?;

    Ok(config.package.name)
}

fn compile_file(file: &Path, project_name: &str) {
    let output_dir = Path::new("output");
    let log_dir = output_dir.join("logs");

    if let Err(e) = fs::create_dir_all(&log_dir) {
        eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Failed to create log directory: {}", e);
        return;
    }

    let output_path = output_dir.join(format!("{}.exe", project_name));
    let start_build_time = Local::now();
    let output = Command::new("rustc")
        .arg(file)
        .arg("-o")
        .arg(&output_path)
        .output();

    let timestamp = start_build_time.format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_path = log_dir.join(format!("build_{}.txt", timestamp));

    if let Ok(mut log_file) = fs::File::create(&log_file_path).map(BufWriter::new) {
        writeln!(log_file, "Timestamp: {}", start_build_time).unwrap_or_default();
        writeln!(log_file, "Project: {}", project_name).unwrap_or_default();
        writeln!(log_file, "File: {}", file.display()).unwrap_or_default();
        writeln!(log_file, "Operating System: {}", std::env::consts::OS).unwrap_or_default();
        writeln!(log_file, "Rust Version: {}", rust_version()).unwrap_or_default();
        writeln!(log_file, "User: {}", whoami::username()).unwrap_or_default();
        writeln!(log_file, "Platform: {}", whoami::platform()).unwrap_or_default();

        match output {
            Ok(output) if output.status.success() => {
                writeln!(log_file, "Build Status: Success").unwrap_or_default();
                writeln!(log_file, "Build Output:\n{}", String::from_utf8_lossy(&output.stdout)).unwrap_or_default();
            }
            Ok(output) => {
                writeln!(log_file, "Build Status: Failed").unwrap_or_default();
                writeln!(log_file, "Error Output:\n{}", String::from_utf8_lossy(&output.stderr)).unwrap_or_default();
            }
            Err(e) => {
                writeln!(log_file, "Build Status: Failed").unwrap_or_default();
                writeln!(log_file, "Error: {}", e).unwrap_or_default();
            }
        }

        let end_build_time = Local::now();
        let duration = end_build_time.signed_duration_since(start_build_time);
        writeln!(log_file, "Build End Time: {}", end_build_time).unwrap_or_default();
        writeln!(log_file, "Build Duration: {} minutes {} seconds", duration.num_minutes(), duration.num_seconds() % 60).unwrap_or_default();
    } else {
        eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Failed to create log file: {}", log_file_path.display());
    }
}

fn rust_version() -> String {
    Command::new("rustc")
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}
