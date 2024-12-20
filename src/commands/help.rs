pub fn handle_help() {
    println!("\x1b[36m\x1b[1mAsterisk CLI\x1b[0m - The build system for Rust projects.

\x1b[36m\x1b[1mUSAGE:\x1b[0m
    ast <COMMAND>

\x1b[36m\x1b[1mCOMMANDS:\x1b[0m
    init       Initialize a new project in the current directory
    help       Prints this help information
    build      Build the project if initialized with Asterisk.yml

Use \x1b[36m\x1b[1mast help\x1b[36m\x1b[0m <COMMAND> for more information about a command.");
}