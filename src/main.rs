mod commands;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("init") => commands::init::handle_init(),
        Some("help") | None => commands::help::handle_help(),
        Some("build") => commands::build::handle_build(),
        Some(cmd) => eprintln!("[\x1b[31m\x1b[1mError\x1b[0m] Unknown Command: {}", cmd),
    }
}
