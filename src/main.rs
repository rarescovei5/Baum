mod args;
mod walk;

use walk::{Counts,walk};

fn main() {
    let (dir, config) = args::parse();
    let mut count = Counts {files: 0, dirs: 0};
    println!("{}", &dir);
    let _ = walk(&config, &mut count, &dir, "");
    println!("\n{} files, {} directories", count.files, count.dirs);
}
