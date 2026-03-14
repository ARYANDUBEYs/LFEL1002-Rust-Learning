use std::io::{self, Write};
struct Bike {
    name : String,
    speed : f64,
    distance : f64,
}
impl Bike {
    fn time(&self) -> f64{
        self.distance/self.speed
    }
}

fn main() {
    print!("Enter the bike's name: ");
    io::stdout().flush().unwrap();
    let mut nam = String::new();
    io::stdin().read_line(&mut nam).unwrap();

    print!("Enter distance to cover: ");
    io::stdout().flush().unwrap();
    let mut dist_str = String::new();
    io::stdin().read_line(&mut dist_str).unwrap();
    let dist : f64 = dist_str.trim().parse().expect("Enter a number: ");

    print!("Enter speed: ");
    io::stdout().flush().unwrap();
    let mut sp_str = String::new();
    io::stdin().read_line(&mut sp_str).unwrap();
    let sp : f64 = sp_str.trim().parse().expect("Enter a number: ");

    let my_bike = Bike {
        name : nam.trim().to_string(),
        speed : sp,
        distance : dist,
    };
    println!("My bike {} will take {} units to reach the destination", my_bike.name, my_bike.time());
}