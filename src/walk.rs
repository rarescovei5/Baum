use std::{fs, io, path::Path};

use colored::{Color, Colorize};
use humansize::DECIMAL;

use crate::args::Config;

pub struct Counts {
    pub files: u32,
    pub dirs: u32,
    pub bytes: u64,
}

const GLYPH_COLOR: Color = Color::BrightBlack;
const DIR_COLOR:   Color = Color::BrightBlue;
const FILE_COLOR:  Color = Color::BrightWhite;
const SIZE_COLOR:  Color = Color::BrightMagenta;

pub fn walk(config: &Config, counts: &mut Counts, dir: &Path, prefix: &str) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|path| should_display(path, config))
        .collect();

    entries.sort_by_key(|p| p.file_name().map(|n| n.to_os_string()));

    let total = entries.len();
    for (i, path) in entries.into_iter().enumerate() {
        let is_last = i + 1 == total;
        let is_dir = path.is_dir();
        let name = path.file_name().unwrap().to_string_lossy();

        // update counts
        if is_dir { counts.dirs += 1 } else { counts.files += 1 }

        // prefix + glyph
        let glyph = if is_last { "└── " } else { "├── " }
            .color(GLYPH_COLOR);
        let next_prefix = format!("{prefix}{}    ", (if is_last { " " } else { "│" }).color(GLYPH_COLOR));

        // compute size suffix once
        let size_suffix = if !is_dir && config.display_bytes {
            let sz = path.metadata()?.len();
            counts.bytes += sz;
            format!(" {}",
                humansize::format_size(sz, DECIMAL)
                    .color(SIZE_COLOR)
            )
        } else {
            String::new()
        };

        // build the core name with coloring
        let colored_name = {
            let base = if config.display_level > 0 {
                format!("{}\\", dir.display())
            } else {
                String::new()
            };
            let color = if is_dir { DIR_COLOR } else { FILE_COLOR };
            format!("{}{}", base.color(color), name.color(color))
        };

        // print
        if config.display_indentation {
            println!("{prefix}{glyph}{colored_name}{size_suffix}");
        } else {
            println!("{colored_name}{size_suffix}");
        }

        // recurse
        if is_dir {
            walk(config, counts, &path, &next_prefix)?;
        }
    }

    Ok(())
}

/// centralize the "should I show it?"" logic
fn should_display(path: &Path, config: &Config) -> bool {
    // skip non‐dir if we're only showing dirs
    if !path.is_dir() && config.display_only_dirs {
        return false;
    }
    // skip hidden / ignored
    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
        if !config.display_hidden && name.starts_with('.') {
            return false;
        }
        if config.ignored_dirs.iter().any(|ig| ig == name) {
            return false;
        }
    }
    true
}
