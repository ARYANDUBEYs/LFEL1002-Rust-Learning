use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file = File::create("io_test.txt").expect("Fail");
    file.write_all(b"Learn").expect("Fail");
}