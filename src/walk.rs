use std::{fs, io};

use humansize::DECIMAL;
use colored::Colorize;

use crate::args::Config;

pub struct Counts {
    pub files: u32,
    pub dirs: u32,
    pub bytes: u64,
}

const GLYPH_COLOR: &str  = "bright_black";
const DIR_COLOR: &str    = "blue";
const FILE_COLOR: &str   = "green";
const SIZE_COLOR: &str   = "yellow";

pub fn walk(config: &Config, counts: &mut Counts, dir: &str, prefix: &str) -> io::Result<()> {
    let mut paths: Vec<_> = fs::read_dir(dir)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            if !path.is_dir() && config.display_only_dirs {
                return false;
            }

            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    return (config.display_hidden || !name.starts_with(".") ) 
                    && !config.ignored_dirs.iter().any(|ignored| ignored == name);
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

        let bytes = if !path.is_dir() && config.display_bytes {
            let size_bytes = path.metadata().unwrap().len();
            counts.bytes += size_bytes;
            &format!(" - {}", humansize::format_size(size_bytes, DECIMAL).color(SIZE_COLOR))
        } else {
            ""
        };

        let displayed_path = if config.display_level > 0 {
            let (colored_dir,colored_name) = if path.is_dir() {
                (&format!("{}/",dir).color(DIR_COLOR), name.color(DIR_COLOR))
            } else {
                (&format!("{}/",dir).color(FILE_COLOR), name.color(FILE_COLOR))
            };
            &format!("{}{}{}", colored_dir, colored_name, bytes)
        } else {
            let colored_name = if path.is_dir() {
                name.color(DIR_COLOR)
            } else {
                name.color(FILE_COLOR)
            };
            &format!("{}{}", colored_name, bytes)
        };

        let (next_prefix, glyph) = if paths_length == 0 { 
            (&format!("{}    ", prefix),"└── " )
        } else {  
            (&format!("{}│    ", prefix),"├── ") 
        };
        let glyph = glyph.color(GLYPH_COLOR);

        if config.display_indentation {
            println!("{}{}{}", prefix, glyph, displayed_path);
        } else {
            println!("{displayed_path}");
        }
        if path.is_dir() {
            walk(
                config, 
                counts, 
                &format!("{}/{}", dir, name),  
                next_prefix
            )?;
        }  
    };

    Ok(())
}