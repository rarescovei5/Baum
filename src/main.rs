mod args;
mod walk;

use std::path::PathBuf;
use colored::{Colorize, Color};
use humansize::DECIMAL;
use walk::{Counts, walk};

fn main() {
    let (dir_str, config) = args::parse();
    let mut counts = Counts { files: 0, dirs: 0, bytes: 0 };

    // Convert parsed String into a PathBuf
    let dir_path = if dir_str.as_str() == "." && config.display_level == 2 {
        let cwd = std::env::current_dir().unwrap();
        println!("{}", cwd.display());
        cwd
    } else {
        println!("{}", dir_str);
        PathBuf::from(dir_str)
    };

    // Start walking
    let _ = walk(&config, &mut counts, &dir_path, "");

    // Final summary
    let bytes = if config.display_bytes {
        humansize::format_size(counts.bytes, DECIMAL)
    } else {
        String::new()
    };


    let summary = format!(
        "{}, {} | {}",
        &format!("{} directories", counts.dirs).color(Color::BrightBlue),
        &format!("{} files", counts.files).color(Color::BrightWhite),
        &format!("{}", bytes).color(Color::BrightMagenta)
    );

    println!("\n{summary}");
}
