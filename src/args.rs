enum ContextScope {
    Directory,
    IgnoredDirs,
    DisplayLevel,
    Unknown
}

#[derive(Debug)]
pub struct Config {
    pub display_hidden: bool,
    pub display_only_dirs: bool,
    pub display_level: u8,
    pub display_indentation: bool,
    pub ignored_dirs: Vec<String>
}

pub fn parse() -> (String, Config) {
    let args = std::env::args().skip(1);

    let mut dir  = String::from(".");

    let mut current_context = ContextScope::Directory;
    
    let mut ignored_dirs: Vec<String> = Vec::new();
    let mut display_hidden: bool = false;
    let mut display_level: u8 = 0;
    let mut display_indentation: bool = true;
    let mut display_only_dirs: bool = false;

    for arg in args {
        if arg.starts_with("-") {
            match arg.as_str() {
                // Outputs the version of baum and quits early
                "-v" | "-version" => {
                    println!("Version 1.0.0 - baum");
                    std::process::exit(0);
                },
                // Shows files begining with a "."
                "-h" | "-hidden" => {
                    display_hidden = true;
                },
                // Ignores the mentioned folders that come after it
                "-I" | "-ignore" => {
                    current_context = ContextScope::IgnoredDirs;
                },
                // Displays only directories
                "-d" | "-directories" => {
                    display_only_dirs = true;
                },
                // Hides indentation
                "-i" | "-indentation" => {
                    display_indentation = false;
                }
                // display_level=0 - Shows only names when printing 
                // display_level=1 - Shows relative paths
                // display_level=2 - Shows absolute paths
                "-l" | "-level" => {
                    display_level = 1;
                    current_context = ContextScope::DisplayLevel;
                }
                // Catch unkown flags
                other => {
                    current_context = ContextScope::Unknown;
                    eprintln!("Warning: unknown option `{}`\n Consider running -help to see what's available", other);
                }
            }
            continue;
        }

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

   (dir, Config { display_hidden, display_only_dirs, display_level, display_indentation, ignored_dirs })
}