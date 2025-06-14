mod args;
mod walk;

use walk::{Counts,walk};

fn main() {
    let (dir, config) = args::parse();
    let mut count = Counts {files: 0, dirs: 0};

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

    
    let _ = walk(&config, &mut count, &dir, "");
    println!("\n{} files, {} directories", count.files, count.dirs);
}
