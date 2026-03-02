use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// Using io::Result<File> with match
fn create_with_match(path: &str) -> io::Result<File> {
    match File::create(path) {
        Ok(f) => {
            println!("Success: Created {}", path);
            Ok(f)
        }
        Err(e) => {
            eprintln!("Custom Error: Cannot create file. Reason: {}", e);
            Err(e)
        }
    }
}

//Using io::Result<()>
fn write_with_question_mark(mut file: File) -> io::Result<()> {
    file.write_all(b"Modular Rust I/O\nLine 2")?;
    Ok(())
}

//Using io::Result<()> with Ok & Err
fn read_with_buffer(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(l) = line {
            println!("Buffered output: {}", l);
        }
        else if let Err(e) = line {
            return Err(e);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let path = "modular_test.txt";

    let file = create_with_match(path)?;
    write_with_question_mark(file)?;
    read_with_buffer(path)?;

    Ok(())
}