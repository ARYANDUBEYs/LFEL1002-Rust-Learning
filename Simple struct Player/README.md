# Rust : Player Struct & Debugging
A foundational example of using Structs and the Debug trait in Rust. This project demonstrates how to model a game character and print its entire state instantly using derived traits.

## Features
- **Custom Data Modeling** : Defines a Player struct to group related data: a name (`String`), a rank (`u32`), and precise health points (`f64`).
- **Automatic Debugging** : Uses the `#[derive(Debug)]` attribute, which tells the Rust compiler to automatically create a "printable" version of the struct.
- **Efficient Logging** : Demonstrates the `{:?}` syntax (the "Debug formatter") to print all fields of a struct in a single line without manual formatting.

## Usage
``` bash
cargo run
```

## What I Learned
- **Attribute Macros** : I learned that lines starting with `#[...]` provide special instructions to the compiler.
- **String Ownership** : I practiced using `String::from()` to create a heap-allocated string for the player's name.
- **Standard Output** : I understood that `println!` can handle more than just simple strings if the data type implements the correct trait (like `Debug`).

## Scholarship Goals
- **Custom Formatting** : My next goal is to implement the `Display` trait manually so I can control exactly how the `player` looks to a user (e.g., "Hero [LVL 10]").
- **Struct Methods** : I am practicing adding an `impl Player` block to include functions like level_up() or take_damage().
- **Pretty Printing** : I aim to try the `{:#?}` flag to see how Rust automatically handles indentation for complex, nested structs.