use std::env;
use std::fs;
use std::path::Path;

use colored::Colorize;

pub struct RuGrep {
    pattern: String,
    file_path: String,
    line_number: bool
}

impl RuGrep {
    pub fn new(pattern: String, file_path: String, line_number: bool) -> Self {
        RuGrep { pattern, file_path, line_number }
    }

    fn read_file(&self, pattern: String, file_path: String) {
        let content = fs::read_to_string(&file_path).expect("Failed to read file");

        for (i, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                if self.line_number {
                    print!("{}:{}:", self.file_path, i);
                }
                let index = line.find(&pattern);

                for (i, c) in line.chars().into_iter().enumerate() {
                    print!(
                        "{}",
                        if i >= index.unwrap() && i < index.unwrap() + pattern.len() {
                            c.to_string().red()
                        } else {
                            c.to_string().white()
                        }
                    );
                }

                println!();
            }
        }
    }

    pub fn show(&self) {
        let path = Path::new(&self.file_path);
        if !path.exists() {
            println!("[-] [err] Couldn't find the file '{}'", self.file_path);
            return;
        }

        if path.is_file() {
            Self::read_file(self, self.pattern.clone(), self.file_path.clone());
            return;
        }

        if path.is_dir() {
            for f in path.read_dir().unwrap() {
                let file = f.unwrap();

                let file_path = file.path();
                if file_path.is_dir() {
                    continue;
                }

                Self::read_file(self, self.pattern.to_string(), file.path().to_string_lossy().to_string());
            }
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    /*for (i, argument) in args.iter().enumerate() {
        println!("{} {}", i, argument);
    }*/

    if args.len() < 3 {
        println!("{}", "Not enough arguments!".red());
        println!("rugrep.exe [options] pattern [files]");
        return;
    }

    let mut line: bool = false;
    let mut has_options: bool = false;
    if args[1].starts_with("-") {
        has_options = true;
        let options = args[1].split_off(1);

        for c in options.chars() {
            if c.eq_ignore_ascii_case(&'n') {
                line = true;
            }
        }
    }

    let mut start = 1;
    if has_options {
        start = 2;
    }

    let mut pattern = args[start].to_string();
    if args.len() >= 3 {
        for i in start+1..args.len() - 1  {
            pattern.push(' ');
            pattern.push_str(&args[i]);
        }
    }
    println!("{}", pattern);

    let rugrep = RuGrep::new(
        pattern,
        args.get(args.len() - 1).as_deref().unwrap().to_string(),
        line
    );
    rugrep.show();
}
