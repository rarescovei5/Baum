enum ContextScope {
    IgnoredDirs,
    Unknown
}

#[derive(Debug)]
pub struct Config {
    pub display_all: bool,
    pub ignored_dirs: Vec<String>
}

pub fn parse() -> (String, Config) {
    let mut args = std::env::args().skip(1);

    let dir  = args.next()
        .unwrap_or_else(|| panic!("No directory supplied."));

    let mut current_context = ContextScope::Unknown;
    
    let mut ignored_dirs: Vec<String> = Vec::new();
    let mut display_all: bool = false;

    for arg in args {
        if arg.starts_with("-") {
            match arg.as_str() {
                "-I" => {
                    current_context = ContextScope::IgnoredDirs;
                }
                "-all" => {
                    display_all = true;
                }
                other => {
                    current_context = ContextScope::Unknown;
                    eprintln!("Warning: unknown option `{}`", other);
                }
            }
            continue;
        }

        match current_context {
            ContextScope::IgnoredDirs => {
                ignored_dirs.push(arg);
            },
            ContextScope::Unknown => {}
        }
    }   

   (dir, Config { display_all, ignored_dirs })
}