use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::time::Instant;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn handle_build() {
    let start_time = Instant::now();
    let wd = std::env::current_dir().unwrap_or_else(|e| {
        eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Could not get current directory: {}", e);
        std::process::exit(1);
    });

    let project_name = match get_project_name_from_yml(&wd) {
        Some(name) => name,
        None => {
            eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Project not initialized. `Asterisk.yml` not found.");
            return;
        }
    };

    let src_path = wd.join("src").join("main.rs");
    if !src_path.exists() {
        eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] src/main.rs does not exist.");
        return;
    }

    println!("[\x1b[34m\x1b[1mInfo\x1b[0m] Building project '{}'...", project_name);

    let modified_files = get_modified_files(&wd);

    let modified_files = Arc::new(Mutex::new(modified_files));
    let mut handles = vec![];

    for file in modified_files.lock().unwrap().iter() {
        let file = file.clone();
        let handle = thread::spawn(move || {
            compile_file(file);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start_time.elapsed();
    println!("[\x1b[32m\x1b[1mSuccess\x1b[0m] Project '{}' built successfully in {:.2?}.", project_name, duration);
}

fn get_modified_files(wd: &Path) -> HashSet<PathBuf> {
    let mut modified_files = HashSet::new();

    let src_path = wd.join("src");
    if let Ok(entries) = fs::read_dir(src_path) {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().extension().map(|ext| ext == "rs").unwrap_or(false) {
                if is_file_modified(&entry.path()) {
                    modified_files.insert(entry.path());
                }
            }
        }
    }

    modified_files
}

fn is_file_modified(path: &Path) -> bool {
    true
}

fn compile_file(file: PathBuf) {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        if let Err(e) = fs::create_dir(output_dir) {
            eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Failed to create output directory: {}", e);
            return;
        }
    }

    let output_path = output_dir.join(file.file_stem().unwrap()).with_extension("exe");
    let file_clone = file.clone();

    let output = Command::new("rustc")
        .arg(file_clone)
        .arg("-o")
        .arg(output_path)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("[\x1b[32m\x1b[1mSuccess\x1b[0m] Compiled file '{}'.", file.display());
        }
        Ok(output) => {
            eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Failed to compile '{}':\n{}", file.display(), String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Failed to execute rustc for '{}': {}", file.display(), e);
        }
    }
}

fn get_project_name_from_yml(wd: &Path) -> Option<String> {
    let yml_path = wd.join("Asterisk.yml");
    fs::read_to_string(&yml_path)
        .ok()?
        .lines()
        .find(|line| line.starts_with("name:"))
        .map(|line| line.trim_start_matches("name:").trim().to_string())
}
