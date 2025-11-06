use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path;
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
                if self.arg_manager.is_true(self.arg_manager.get_option("n").unwrap()) {
                    print!("{}:{}:", file_path, i);
                }

                let index = line.find(&pattern);

                for (i, c) in line.chars().into_iter().enumerate() {
                    if self.arg_manager.is_true(self.arg_manager.get_option("nf").unwrap()) {
                        print!("{}", c.to_string());    
                    } else {
                        print!(
                            "{}",
                            if i >= index.unwrap() && i < index.unwrap() + pattern.len() {
                                c.to_string().red()
                            } else {
                                c.to_string().white()
                            }
                        );
                    }
                }

                println!();
            }
        }
    }

    pub fn show(&self) {
        let path = Path::new(&self.file_path);
        if path.to_string_lossy().contains("*") {
            let path_str = path.to_string_lossy();
            
            let dir = path_str.split_at(path_str.find("*").unwrap()).0;
            let path_str = path_str.split_at(path_str.find("*").unwrap() + 2).1;
            
            for f in Path::new(dir).read_dir().unwrap() {
                let file = f.unwrap();

                let file_path = file.path();
                if file_path.is_dir() {
                    continue;
                }

                if file_path.extension().unwrap().eq_ignore_ascii_case(path_str) {
                    Self::read_file(self, self.pattern.to_string(), file_path.to_string_lossy().to_string());
                }
            }
            return;
        }

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
    let mut am = ArgManager::new(env::args().collect());
    am.add_option("n", Value::Bool(false));
    am.add_option("wf", Value::Bool(false));
    am.add_option("nf", Value::Bool(false));
    am.scan();

    if am.length < 3 {
        println!("{}", "Not enough arguments!".red());
        println!("rugrep.exe [options] pattern [files]");
        return;
    }

    let mut start = 1;
    if am.has_options() {
        start = am.options_size + 1;
    }

    let mut pattern = am.arguments[start].to_string();
    if am.length >= 3 {
        for i in start+1..am.length - 1  {
            pattern.push(' ');
            pattern.push_str(&am.arguments[i]);
        }
    }

    let rugrep = RuGrep::new(
        pattern,
        am.arguments.get(am.length - 1).as_deref().unwrap().to_string(),
        am
    );
    rugrep.show();
}
