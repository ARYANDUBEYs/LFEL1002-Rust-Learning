use std::io::{self, Write}; 

struct Car {
    make: String,
    model: String,
    year: u16,
    color: String,
    mileage: u32,
} 

impl Car {
    fn display(&self) {
        println!(
            "Make: {}\nModel: {}\nYear: {}\nColor: {}\nMileage: {} miles",
            self.make,
            self.model,
            self.year,
            self.color,
            self.mileage
        );
    }
} 

fn main() {
    let car1 = get_car_details();
    let car2 = get_car_details(); 

    println!("Car 1:");
    car1.display(); 

    println!("Car 2:");
    car2.display();
} 

fn get_car_details() -> Car {
    println!("Enter car details:"); 

    let mut input = String::new(); 

    print!("Make:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let make = input.trim().to_string();
    input.clear(); 

    // Model
    print!("Model:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let model = input.trim().to_string();
    input.clear(); 

    // Year
    print!("Year:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let year: u16 = input.trim().parse().expect("Invalid input");
    input.clear(); 

    // Color
    print!("Color:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let color = input.trim().to_string();
    input.clear(); 

    // Mileage
    print!("Mileage (in miles):");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mileage: u32 = input.trim().parse().expect("Invalid input"); 

    // Create and return the car instance
    Car {
        make,
        model,
        year,
        color,
        mileage,
    }
}