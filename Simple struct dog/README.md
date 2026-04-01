# RUST : Pet Management & Data Cloning
A practical exploration of the Clone trait in Rust. This project demonstrates how to create independent copies of structured data, allowing for local modifications and multi-function usage without losing ownership of the original variables.

## Features
- **Explicit Data Duplication** : Uses the `.clone()` method to create a deep copy of a `struct`, ensuring the original remains untouched.
- **Derived Traits** : Implements `#[derive(Clone, Debug)]` to automatically provide copying and printing capabilities to the `Pet` struct.
- **Independent Mutability** : Shows how a cloned object can be marked as `mut` and modified (changing an age from 3 to 5) while the original stays the same.
- **Ownership Persistence**: Demonstrates passing clones into functions so that the `main` function retains ownership of the data for later use.

## Usage 
```bash
cargo run
```

## What I Learned
- **The Clone Trait** : I learned that Clone is required when you want a second, independent copy of data that lives on the heap (like a String).
- **Deep vs. Shallow** : I practiced "Deep Copying," where both the struct and its internal string are duplicated to prevent memory conflicts.
- **Preserving Ownership** : I learned that cloning before calling a function is a reliable way to keep variables alive for the rest of the program.
- **Derived Cloning** : I learned that adding Clone to the derive attribute allows a `struct` to be duplicated with a single method call.
- **Memory Allocation**: I practiced managing heap-allocated strings, understanding that `.clone()` creates a new allocation in memory.
- **Explicit vs. Implicit** : I understood that while simple numbers copy automatically, complex types like my `Pet` struct require explicit cloning to move data.

## Scholarship Goals
- **Struct Integration** : I am going to implement *Structs* within a *HashMap* (e.g., `HashMap<String, Pet>`) to store more complex data for each entry.
- **Complex System Design** : I am going to build more complex programs that manage relational data, moving beyond simple key-value pairs.
- **Advanced Data Management** : I aim to learn how to search, update, and filter these complex collections efficiently.

