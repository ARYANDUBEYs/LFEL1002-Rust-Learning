#[derive(Debug)]
struct Player {
    name: String,
    level: u32,
    health: f64,
}

fn main() {
    let p1 = Player { 
        name: String::from("Hero"), 
        level: 10, 
        health: 95.5 
    };

    println!("{:?}", p1);
}