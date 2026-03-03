use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn main() {
    let file_path = "input.txt";
    
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening the file: {}", error);
            return;
        }
    };

    let reader = BufReader::new(file); 
    let password_regex = Regex::new(r"(?i)(password|api[_\s]?key)[:=]\s*(\w+)").unwrap();

    for (line_number, line) in reader.lines().enumerate() {
        let line_text = match line { 
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading line {}: {}", line_number + 1, error);
                continue;
            }
        };

        if password_regex.is_match(&line_text) {
           println!("Potential Security issue found in line {} : {}", line_number + 1, line_text);   
        }
    }
}