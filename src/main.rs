use std::env;
use std::fs;
use std::path::Path;

use colored::Colorize;

mod argmanager;
use argmanager::ArgManager;
use argmanager::Value;

pub struct RuGrep {
    pattern: String,
    file_path: String,
    arg_manager: ArgManager
}

impl RuGrep {
    pub fn new(pattern: String, file_path: String, arg_manager: ArgManager) -> Self {
        RuGrep { pattern, file_path, arg_manager }
    }

    fn read_file(&self, pattern: String, file_path: String) {
        let content = fs::read_to_string(&file_path).expect("Failed to read file");

        for (i, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                if let Some(_) = self.arg_manager.get_option("n") {
                    print!("{}:{}:", file_path, i);
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

                Self::read_file(self, self.pattern.to_string(), file_path.to_string_lossy().to_string());
            }
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut am = ArgManager::new(env::args().collect());
    am.add_option("n", Value::None(false));
    am.add_option("v", Value::None(false));
    am.scan();

    if args.len() < 3 {
        println!("{}", "Not enough arguments!".red());
        println!("rugrep.exe [options] pattern [files]");
        return;
    }

    let mut start = 1;
    if am.has_options() {
        start = 2;
    }

    let mut pattern = args[start].to_string();
    if args.len() >= 3 {
        for i in start+1..args.len() - 1  {
            pattern.push(' ');
            pattern.push_str(&args[i]);
        }
    }

    let rugrep = RuGrep::new(
        pattern,
        args.get(args.len() - 1).as_deref().unwrap().to_string(),
        am
    );
    rugrep.show();
}
