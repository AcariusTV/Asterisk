use std::fs;
use std::path::Path;
use std::io;

pub fn clean_output() {
    let output_dir = Path::new("output");

    if output_dir.exists() {
        let (total_size, file_count) = match count_files(output_dir) {
            Ok((size, count)) => (size, count),
            Err(e) => {
                eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Failed to count files: {}", e);
                return;
            }
        };

        if let Err(e) = fs::remove_dir_all(output_dir) {
            eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Failed to delete directory: {}", e);
            return;
        }

        let total_size_kb = (total_size as f64 / 1024.0).round() as u64;
        println!("[\x1b[34m\x1b[1mInfo\x1b[0m] {} files deleted, freed {} bytes ({} KB) of space.", file_count, total_size, total_size_kb);
    } else {
        println!("[\x1b[33m\x1b[1mWarning\x1b[0m] 'output' directory does not exist.");
    }
}

fn count_files(dir: &Path) -> io::Result<(u64, u64)> {
    let mut total_size = 0;
    let mut file_count = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let (size, count) = count_files(&path)?;
            total_size += size;
            file_count += count;
        } else {
            total_size += path.metadata()?.len();
            file_count += 1;
        }
    }

    Ok((total_size, file_count))
}
