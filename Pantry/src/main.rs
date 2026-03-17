use std::collections::HashMap;

fn main() {
    let mut pantry = HashMap::new();

    pantry.insert("Apples", 10);
    pantry.insert("Bananas", 3);
    pantry.insert("Cookies", 24);

    for (k, v) in &pantry {
        println!("Item: {} | Quantity: {}", k, v);
    }
}