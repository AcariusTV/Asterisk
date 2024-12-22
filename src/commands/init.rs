use std::fs::{File, create_dir};
use std::io::Write;
use std::path::Path;

pub fn handle_init() {
    let wd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Could not get current directory: {}", e);
            return;
        }
    };

    let project_name = wd.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unnamed_project");

    let tasks = [
        create_asterisk_yml(&wd, project_name),
        setup_src_directory(&wd),
        create_gitignore(&wd),
    ];

    if let Some(Err(e)) = tasks.iter().find(|result| result.is_err()) {
        eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] {}", e);
    } else {
        println!("[\x1b[32m\x1b[1mSuccess\x1b[0m] Successfully initialized project '{}'.", project_name);
    }
}

fn create_asterisk_yml(wd: &Path, project_name: &str) -> Result<(), String> {
    let content = format!(
        "package:\n  name: {}\n  version: 0.1.0\n\ndependencies:\n  # Add your dependencies here\n",
        project_name
    );
    write_to_file(&wd.join("Asterisk.yml"), &content)
}

fn setup_src_directory(wd: &Path) -> Result<(), String> {
    let src_dir = wd.join("src");
    create_dir(&src_dir).map_err(|e| format!("Failed to create `src` directory: {}", e))?;

    let content = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
    write_to_file(&src_dir.join("main.rs"), &content)
}

fn create_gitignore(wd: &Path) -> Result<(), String> {
    let content = "/output\n";
    write_to_file(&wd.join(".gitignore"), &content)
}

fn write_to_file(path: &Path, content: &str) -> Result<(), String> {
    File::create(path)
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .map_err(|e| format!("Failed to create `{}`: {}", path.display(), e))
}

fn is_file_modified(_path: &Path) -> bool {
    false
}
