use std::env;
use std::process;

use chapter_13::Config; // Changed from chapter_12 to chapter_13

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = chapter_13::run(config) {
        // Changed from chapter_12 to chapter_13
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
