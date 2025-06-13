use std::{fs, io};

use crate::args::Config;

pub struct Counts {
    pub files: u32,
    pub dirs: u32
}


pub fn walk(config: &Config, counts: &mut Counts, dir: &str, prefix: &str) -> io::Result<()> {
    let mut paths: Vec<_> = fs::read_dir(dir)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            if !path.is_dir() {
                return true;
            }

            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    return (config.display_all || !name.starts_with(".")) && !config.ignored_dirs.iter().any(|ignored| ignored == name);
            }
            
            true
        })
        .collect();
    let mut paths_length = paths.len();

    paths.sort_by_key(|path| path.file_name().map(|s| s.to_os_string()));


    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap();

        paths_length -= 1;

        if path.is_dir() {
            counts.dirs += 1;
        } else {
            counts.files += 1;
        }

        if paths_length == 0 {
            println!("{}└── {}", prefix, name);
            if path.is_dir() {
                walk(
                    config, 
                    counts, 
                    &format!("{}/{}", dir, name),  
                    &format!("{}    ", prefix)
                )?;
            }
        } else {
            println!("{}├── {}", prefix, name);
            if path.is_dir() {
                walk(
                    config, 
                    counts, 
                    &format!("{}/{}", dir, name),  
                    &format!("{}│   ", prefix)
                )?;
            }
        }
    };

    Ok(())
}