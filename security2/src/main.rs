use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use regex::Regex;

fn main() {
    print!("Enter the path to the file you want to scan: ");
    io::stdout().flush().unwrap();

    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).expect("Failed to read input");

    let file_path = input_path.trim();

    let password_regex = Regex::new(r"(?i)(password|api[_\s]?key)[:=]\s*(\w+)").unwrap();

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening the file '{}': {}", file_path, error);
            return;
        }
    };

    let reader = BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line_text = match line {
            Ok(text) => text,
            Err(error) => {
                eprintln!("Error reading line {}: {}", line_number + 1, error);
                continue;
            }
        };

        if password_regex.is_match(&line_text) {
            println!("Potential Security issue found in line {} : {}",line_number + 1, line_text);
        }
    }
}