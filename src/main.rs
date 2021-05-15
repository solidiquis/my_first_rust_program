use std::env;
use std::error::Error;
use std::process;
use std::fs;
use regex::Regex;

struct Config {
    filename: String,
    pattern: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.")
        }

        Ok(Config {
            filename: args[1].clone(),
            pattern: args[2].clone(),
        })
    }

    fn find_matches(&self) -> Result<(), Box<dyn Error>> {
        let txt = fs::read_to_string(&self.filename)?;
        let lines: Vec<&str> = txt.split("\n").collect();
        let re = Regex::new(&self.pattern).unwrap();
        
        if !re.is_match(&txt) {
            return Err("No matches with given pattern.".into())    
        }

        for line in lines.iter() {
            let mut curr_line = line.clone().to_string();

            for mat in re.find_iter(line) {
                curr_line = curr_line.replace(&mat.as_str(), &self.highlight(mat.as_str()));
            }

            println!("{}", &curr_line)
        }

        Ok(())
    }

    fn highlight(&self, txt: &str) -> String {
        format!("\x1B[31m{}\x1B[0m", txt)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    config.find_matches().unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    })
}
