use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() -> io::Result<()> {
    let mut file = File::create("Buff.txt")?;
    file.write_all(b"Hi")?;
    let file = File::open("Buff.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let l = line?;
        println!("Read line: {}", l);
    }
    Ok(())
}
