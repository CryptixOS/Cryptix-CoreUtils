use std::fmt;
use std::fmt::Display;
use std::fs::{ReadDir, read_dir};

struct Output {
    entries: Vec<String>,
    list: bool,
}

impl Output {
    pub fn new(entries: &mut ReadDir) -> Output {
        let mut entry_names = Vec::new();

        for entry in entries.flatten() {
            entry_names.push(entry.file_name().into_string().unwrap());
        }

        Output {
            entries: entry_names,
            list: false,
        }
    }

    pub fn builder(entries: &mut ReadDir, options: &[String]) -> Output {
        let mut output = Output::new(entries);

        options.iter().for_each(|option| match option.as_str() {
            "-l" | "--list" => output.list = true,
            _ => {}
        });

        output
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = if self.list {
            self.entries.join("\n")
        } else {
            self.entries.join(" ")
        };

        write!(f, "{}", output)
    }
}

pub fn ls(args: Vec<String>) -> i32 {
    let mut paths = Vec::new();
    let mut options: Vec<String> = Vec::new();

    for arg in args {
        if arg.starts_with('-') {
            options.push(arg);
        } else {
            paths.push(arg);
        }
    }

    paths.remove(0);

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    for path in &paths {
        let read_dir = read_dir(path);

        match read_dir {
            Ok(mut entries) => {
                println!("{}", Output::builder(&mut entries, &options));
            }
            Err(_) => {
                println!("ls: cannot access '{}': No such file or directory", path);
            }
        }
    }

    0
}
