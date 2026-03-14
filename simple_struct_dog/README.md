# RUST : Dog Struct & Implementation in Rust

A foundational Rust program demonstrating how to define custom data structures and attach behaviors to them using `struct` and `impl` blocks. In Rust, a `struct` (structure) allows you to group related data together, while an `impl` (implementation) block allows you to define functions—called methods—that belong specifically to that data. This program models a `Dog` with physical attributes and interactive behaviors.

## Features
- **Custom Data Modeling**: Uses a `struct` to store a `String` name, a `u8` age, and a `bool` status for hunger.
- **Method Implementation**: Uses `impl` to "teach" the Dog how to perform actions like `bark()` and `eat()`.
- **Shared Borrowing (`&self`)**: The `bark()` method uses a reference to the dog, allowing it to "speak" without taking ownership or changing the dog.
- **Mutable Borrowing (`&mut self`)**: The `eat()` method takes a mutable reference, allowing it to update the dog's internal `is_hungry` state.
- **String Ownership**: Utilizes `String::from()` to ensure the Dog owns its name in memory.

## How to Setup
1. Create a new Rust project: `cargo new dog_struct`.
2. Paste the provided code into `src/main.rs`.
3. Run the code using `cargo run`.

## ## Usage
Run the program to see Buddy the dog interact:
```bash
cargo run
# Output:
# My dog's name is Buddy. He is 3 years old.
# woof! woof!
# Buddy is full now.
```

## What I Learned
- **Struct Basics** : I learned that a `struct` is like a "blueprint" that lets me create my own complex types beyond just numbers or strings.
- **The `impl` Block** : I discovered that methods must be defined inside an `impl` block to be associated with a specific struct.
- **Self Awareness** : I learned that `self` refers to the specific instance of the struct the method is running on.
- **Mutability in Structs** : I practiced using `&mut self` to allow a method to change the data inside the struct.
- **Dot Notation** : I learned how to use `.` to access both fields (like `my_dog.name`) and methods (like `my_dog.bark()`).

## Scholarship Goals
- **Encapsulation** : My goal is to understand how to keep data safe and organized by grouping it with the logic that handles it.
- **Reference Management** : I am practicing when to use &self (read-only) versus &mut self (read-write) to follow Rust's strict safety rules.
- **Modeling Real-World Objects** : I aim to learn how to translate real-world objects and their behaviors into clean, efficient Rust code.
- **Constructor Patterns** : I want to learn how to create "New" functions (like Dog::new()) to make creating new dog instances easier and safer.