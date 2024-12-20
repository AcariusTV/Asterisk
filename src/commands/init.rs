use std::fs::{File, create_dir};
use std::io::Write;
use std::path::Path;

pub fn handle_init() {
    let wd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Could not get current directory: {}", e),
    };

    let project_name = wd.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unnamed_project");

    match [
        create_asterisk_yml(&wd, project_name),
        setup_src_directory(&wd),
        create_gitignore(&wd),
    ].into_iter().find(|result| result.is_err()) {
        Some(Err(e)) => eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] {}", e),
        Some(Ok(_)) => {},
        None => println!("[\x1b[32m\x1b[1mSuccess\x1b[0m] Successfully initialized project '{}'.", project_name),
    }
}

fn create_asterisk_yml(wd: &Path, project_name: &str) -> Result<(), String> {
    let file_path = wd.join("Asterisk.yml");
    let content = format!(
        "name: {}\nversion: 0.1.0\n\ndependencies:\n  # Add your dependencies here\n",
        project_name
    );
    write_to_file(&file_path, &content, "Asterisk.yml")
}

fn setup_src_directory(wd: &Path) -> Result<(), String> {
    let src_dir = wd.join("src");
    create_dir(&src_dir).map_err(|e| format!("Failed to create `src` directory: {}", e))?;

    let main_rs_path = src_dir.join("main.rs");
    let content = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
    write_to_file(&main_rs_path, &content, "main.rs")
}

fn create_gitignore(wd: &Path) -> Result<(), String> {
    let gitignore_path = wd.join(".gitignore");
    let content = "/output\n";
    write_to_file(&gitignore_path, &content, ".gitignore")
}

fn write_to_file(path: &Path, content: &str, description: &str) -> Result<(), String> {
    File::create(path)
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .map_err(|e| format!("Failed to create `{}`: {}", description, e))
}

fn is_file_modified(_path: &Path) -> bool {
    false
}
