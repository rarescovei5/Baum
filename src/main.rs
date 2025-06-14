mod args;
mod walk;

use humansize::DECIMAL;
use walk::{Counts,walk};

fn main() {
    let (dir, config) = args::parse();
    let mut counts = Counts {files: 0, dirs: 0, bytes: 0};

    let dir: String = if dir.as_str() == "." && config.display_level == 2 {
        let res = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .replace('\\', "/");
        println!("{}", &res);
        res
    } else {
        println!("{}", &dir);
        dir
        .replace('\\', "/")
    };

    
    let _ = walk(&config, &mut counts, &dir, "");

    let bytes = if config.display_bytes {
        &format!(" - {}", humansize::format_size(counts.bytes, DECIMAL))
    } else {
        ""
    };

    println!("\n{} files, {} directories{}", counts.files, counts.dirs, bytes);
}
