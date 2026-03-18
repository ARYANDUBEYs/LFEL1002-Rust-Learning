use std::collections::HashMap;
use std::io::{self, Write}; 

#[derive(Debug, Clone)]
struct Pizza {
    name: String,
    price: f64,
} 

fn create_pizza_menu() -> HashMap<String, Pizza> {
    let mut pizza_menu = HashMap::new();
    pizza_menu.insert(
        "Margherita".to_string(),
        Pizza {
            name: "Margherita".to_string(),
            price: 9.99,
        },
    );
    pizza_menu.insert(
        "Pepperoni".to_string(),
        Pizza {
            name: "Pepperoni".to_string(),
            price: 11.99,
        },
    );
    pizza_menu.insert(
        "Vegetarian".to_string(),
        Pizza {
            name: "Vegetarian".to_string(),
            price: 10.99,
        },
    );
    pizza_menu
} 

#[derive(Debug)]
struct Order {
    pizza: String,
    quantity: u32,
} 

impl Order {
    fn total_price(&self, pizza_menu: &HashMap<String, Pizza>) -> Option<f64> {
        match pizza_menu.get(&self.pizza) {
            Some(pizza) => Some(pizza.price * self.quantity as f64),
            None => None,
        }
    }
} 

fn main() {
    let pizza_menu = create_pizza_menu();
    let mut orders: Vec<Order> = Vec::new(); 

    loop {
        println!("Available Pizzas:");
        for (_, pizza) in &pizza_menu {
            println!("{} - ${:.2}", pizza.name, pizza.price);
        } 

        println!("Enter your pizza choice (or 'q' to quit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let pizza_choice = input.trim().to_string();
        input.clear(); 

        if pizza_choice == "q" {
            break;
        } 

        println!("Enter quantity:");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let quantity: u32 = match input.trim().parse() {
            Ok(qty) => qty,
            Err(_) => {
                println!("Invalid quantity. Please enter a valid number.");
                continue;
            }
        }; 
        input.clear(); 

        if !pizza_menu.contains_key(&pizza_choice) {
            println!("Invalid pizza choice. Please select a pizza from the menu.");
            continue;
        }

        let order = Order {
            pizza: pizza_choice.clone(),
            quantity,
        };
        orders.push(order);
    }

    println!("Your order:");
    let mut total_cost = 0.0;
    for order in &orders {
        println!(
            "{} - {} x ${:.2} each = ${:.2}",
            order.pizza,
            order.quantity,
            pizza_menu[&order.pizza].price,
            order.total_price(&pizza_menu).unwrap()
        );
        total_cost += order.total_price(&pizza_menu).unwrap();
    }
    println!("Total Cost: ${:.2}", total_cost);
}