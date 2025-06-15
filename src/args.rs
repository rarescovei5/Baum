/// Defines which argument context we're currently parsing.
///
/// - `Directory`: Expecting a directory path.
/// - `IgnoredDirs`: Collecting names of directories to ignore.
/// - `DisplayLevel`: Next token sets the output path display level.
/// - `Unknown`: Skip unknown options until context resets.
enum ContextScope {
    Directory,
    IgnoredDirs,
    DisplayLevel,
    Unknown
}

/// Holds all configuration flags and values parsed from CLI.
#[derive(Debug)]
pub struct Config {
    /// Show hidden files and directories (names starting with `.`).
    pub display_hidden: bool,
    /// Only display directory entries, hide files.
    pub display_only_dirs: bool,
    /// How to display paths:
    /// - 0: names only,
    /// - 1: relative paths,
    /// - 2: absolute paths.
    pub display_level: u8,
    /// Show or hide the tree glyph prefixes.
    pub display_indentation: bool,
    /// Append size information (bytes) to each entry.
    pub display_bytes: bool,
    /// List of directory names to ignore during traversal.
    pub ignored_dirs: Vec<String>
}

/// Parse command-line arguments into a target directory and a `Config`.
///
/// Supported flags:
/// - `-v`, `-version`: Print version and exit.
/// - `-h`, `-hidden`: Include hidden entries (starting with `.`).
/// - `-p`, `-prefix`: Disable indentation prefixes.
/// - `-i`, `-ignore`: Subsequent args are directory names to skip.
/// - `-b`, `-bytes`: Show size suffix for each entry.
/// - `-d`, `-directories`: Only show directories, hide files.
/// - `-l`, `-level [0|1|2]`: Set how paths are displayed.
pub fn parse() -> (String, Config) {
    // Skip the program name
    let args = std::env::args().skip(1);

    // Default directory is current (`.`)
    let mut dir  = String::from(".");

    // Expect first argument to be the directory to scan
    let mut current_context = ContextScope::Directory;
    
    // Initialize configuration defaults
    let mut ignored_dirs: Vec<String> = Vec::new();
    let mut display_hidden: bool = false;
    let mut display_level: u8 = 0;
    let mut display_indentation: bool = true;
    let mut display_only_dirs: bool = false;
    let mut display_bytes: bool = false;

    for arg in args {
        // Flag handling
        if arg.starts_with("-") {
            match arg.as_str() {
                "--h" | "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                "--v" | "--version" => {
                    // Version info and early exit
                    println!("Version 0.1.0 - baum");
                    std::process::exit(0);
                },
                "-h" | "-hidden" => display_hidden = true,
                "-p" | "-prefix" => display_indentation = false,
                "-b" | "-bytes" => display_bytes =  true,
                "-d" | "-directories" => display_only_dirs = true,
                "-i" | "-ignore" => current_context = ContextScope::IgnoredDirs,
                "-l" | "-level" => {
                    // Default to level=1, then expect a numeric argument
                    display_level = 1;
                    current_context = ContextScope::DisplayLevel;
                }
                other => {
                    // Unknown option: warn and skip until next valid flag
                    eprintln!(
                        "Warning: unknown option `{}`, skipping...",
                        other
                    );
                    current_context = ContextScope::Unknown;
                }
            }
            continue;
        }

        // Value handling based on current context
        match current_context {
            ContextScope::Directory => {
                dir = arg;
            }
            ContextScope::DisplayLevel => {
                display_level = match arg.parse() {
                    Ok(level) => level,
                    Err(_) => panic!("Display Level (-l or -level) must be followed by an integer or be left alone")
                };
            }
            ContextScope::IgnoredDirs => {
                ignored_dirs.push(arg);
            },
            ContextScope::Unknown => {}
        }
    }   

   (
        dir, 
        Config { 
            display_hidden, 
            display_only_dirs, 
            display_level, 
            display_indentation, 
            display_bytes, 
            ignored_dirs 
        }
    )
}

/// Print usage/help information to stdout.
fn print_help() {
    use colored::Colorize;
    
    let baum_art = r#"
  _                           
 | |                          
 | |__   __ _ _   _ _ __ ___  
 | '_ \ / _` | | | | '_ ` _ \ 
 | |_) | (_| | |_| | | | | | |
 |_.__/ \__,_|\__,_|_| |_| |_|
"#;
    // ASCII art banner
    println!("{}",baum_art.bright_yellow().bold());

    // Usage
    println!("{}","Usage:".yellow().bold());
    println!("  {}","baum [directory] [options]".white());
    println!();

    // Options
    println!("{}","Options:".yellow().bold());
    println!("  {}           {}","--h, --help".green(), "Show this help message and exit".white());
    println!("  {}        {}","--v, --version".green(), "Print version and exit".white());
    println!("  {}      {}","-d, -directories".green(), "Only show directories, hide files".white());
    println!("  {}           {}","-h, -hidden".green(), "Show hidden files (names starting with '.')".white());
    println!("  {}           {}","-p, -prefix".green(), "Disable tree indentation prefixes".white());
    println!("  {}            {}","-b, -bytes".green(), "Display size suffix for each entry".white());
    println!("  {} {}     {}","-i, -ignore","<dir>".blue(), "Ignore the specified directory name (can repeat)".white());
    println!("  {} {}    {}","-l, -level","[0|1|2]".blue(), "Set path display level (0=names, 1=relative, 2=absolute)".white());
    println!();

    // Examples
    println!("{}", "Examples:".yellow().bold());
    println!("  {}", "baum".white());
    println!("  {}", "baum src -b -h".white());
    println!("  {}", "baum .. -d".white());
    println!();

    // Info
    println!("{}", "(Defaults: show files & dirs with prefixes; no sizes, no hidden)".bright_black());
    println!("{}", "Report bugs & contribute: https://github.com/rarescovei5/baum".bright_black());
    println!();
}