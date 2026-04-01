use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("input.txt")?;
    write!(file, "{}", "10");
    let file_path = File::open("input.txt")?;
    let reader = BufReader::new(file_path);
    for line in reader.lines() {
        let line_text = line?;
        let number:i32 = line_text.trim().parse().expect("fail to get number");
        let double = number * 2;
        println!("i: {} , o: {}", number, double);
        let mut output_file = File::create("output.txt"); 
        writeln!(output_file?, "{}", double)?;
    }
    Ok(())
}
