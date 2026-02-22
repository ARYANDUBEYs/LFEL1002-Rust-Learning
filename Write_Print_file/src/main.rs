use std::fs;
fn main() {
    fs::write("note.txt", "Hi, Rust").expect("Unable to write in file");
    let content = fs::read_to_string("note.txt").expect("Can't read the file");
    println!("The file contains: {}", content);
}