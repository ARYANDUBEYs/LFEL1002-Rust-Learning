use std::io;
fn fibonacci(n:u32) {
    let mut a = 0;
    let mut b = 1;
    while a <= n{
        print!("{} ", a);
        let t = a + b;
        a = b;
        b = t;
    }
    print!("\n");
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("fail to get num");
    let n:u32 = input.trim().parse().expect("Invalid number");
    fibonacci(n);
}