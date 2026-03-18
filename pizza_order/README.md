# RUST : Interactive Pizza Ordering System
A complex Rust application that simulates a restaurant point-of-sale system. This project integrates HashMaps for data storage, Structs for object modeling, and an interactive CLI loop to manage customer orders and calculate real-time pricing.
## Features
- **Relational Data Mapping** : Uses a `HashMap<String, Pizza>` to link pizza names to complex Pizza structs, allowing for fast menu lookups.
- **Stateful Order Management** : Tracks multiple user selections using a `Vec<Order>`, storing both the item choice and the quantity.
- **Dynamic Price Calculation** : Implements a method on the `Order` struct that references the menu to calculate sub-totals dynamically.
- **Input Validation & Safety** : Features a robust `loop` with `match` blocks to handle invalid numeric inputs and incorrect menu selections without crashing.
- **Automated Billing** : Iterates through all orders to generate an itemized receipt and a final total cost.

## Usage
1. Run the application:
```bash
cargo run
```
2. Select your Pizza: Type the name exactly as shown on the menu (e.g., Margherita).

3. Enter Quantity: Input a whole number. If you enter text by mistake, the program will catch the error and ask again.

4. Add More or Quit: Continue adding items to your cart or type q to finish.

5. Review Receipt: The program will print a formatted table showing each item, the unit price, and the final grand total.

## What I Learned
- **Method Implementation** : I learned how to pass a reference to a `HashMap` into a struct method (`&HashMap<String, Pizza>`) to access global data.
- **String Cloning in Loops** : I practiced using `.clone()` when moving the user's choice into an `Order` struct to satisfy Rust's ownership rules.
- **Buffered Input Management** : I understood the importance of `input.clear()` within a `loop` to prevent previous inputs from interfering with new ones.

## Scholarship Goals
- **Struct Integration** : I have successfully implemented Structs within a `HashMap` to store and manage complex relational data for each entry.
- **Complex System Design** : I am continuing to build complex programs that manage relational data and interactive state, moving beyond simple key-value pairs.
- **Advanced Data Management** : I aim to learn how to implement a "Delete" feature so users can remove an item from their order before checking out.
