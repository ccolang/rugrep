use std::env;
use std::fs;

use colored::Colorize;

pub struct RuGrep {
    pattern: String,
    file_path: String,
}

impl RuGrep {
    fn new(pattern: String, file_path: String) -> Self {
        RuGrep { pattern, file_path }
    }

    fn show(&self) {
        if !fs::exists(&self.file_path).unwrap() {
            println!("[-] [err] Couldn't find the file '{}'", self.file_path);
            return;
        }

        let content = fs::read_to_string(&self.file_path).expect("Failed to read file");

        for line in content.lines() {
            if line.to_lowercase().contains(&self.pattern.to_lowercase()) {
                let index = line.find(&self.pattern);

                let mut i: usize = 0;
                for c in line.chars().into_iter() {
                    print!(
                        "{}",
                        if i >= index.unwrap() && i < index.unwrap() + self.pattern.len() {
                            c.to_string().red()
                        } else {
                            c.to_string().white()
                        }
                    );

                    i += 1;
                }

                println!();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    /*let mut i: i32 = 0;
    for argument in args.iter() {
        //println!("{} {}", i, argument);

        i += 1;
    }*/

    if args.len() < 3 {
        println!("{}", "Not enough arguments!".red());
        println!("rugrep.exe [options] pattern [files]");
        return;
    }

    let mut pattern = args[1].to_string();
    if args.len() >= 4 {
        for i in 2..args.len() -1  {
            pattern.push(' ');
            pattern.push_str(&args[i]);
        }
    }

    let rugrep = RuGrep::new(
        pattern,
        args.get(args.len() - 1).as_deref().unwrap().to_string(),
    );
    rugrep.show();
}
