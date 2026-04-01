# RUST : Pantry Inventory with HashMaps
A practical example of using HashMaps in Rust to store and manage key-value pairs. This project demonstrates how to create a dynamic inventory system where items (keys) are mapped to their specific quantities (values).

## Features
- **Key-Value Mapping** : Utilizes `std::collections::HashMap` to create a direct link between an item name and its count.

- **Dynamic Insertion** : Demonstrates the `.insert()` method to populate the collection with data at runtime.

- **Efficient Iteration** : Uses a `for` loop with references (`&pantry`) to print the entire inventory without losing ownership of the data.

- **Automatic Scaling** : Showcases how `HashMaps` grow automatically as more "snacks" are added to the pantry.

## Usage
```bash
cargo run
```

## What I Learned
- **Unordered Storage** : I learned that `HashMaps` do not store items in the order they were inserted; they use a "Hashing" algorithm to determine where data sits for fast lookups.

- **Standard Collections** : I understood that unlike `Vec`, `HashMaps` must be explicitly imported using `use std::collections::HashMap`.

- **HashMap Syntax** : I learned that a `HashMap` requires two types: one for the Key (the name) and one for the Value (the number).

- **Reference Iteration** : I practiced using `&pantry` in the loop so that the `HashMap` remains available for use even after the loop finishes.

- **Destructuring** : I learned how to use `(k, v)` to neatly separate the key and the value while looping through the collection.

## Scholarship Goals
- **Struct Integration** : I am going to implement *Structs* within a `HashMap` (e.g., HashMap<String, Record>) to store more complex data for each entry.

- **Complex System Design** : I am going to build more complex programs that manage relational data, moving beyond simple key-value pairs.

- **Advanced Data Management** : I aim to learn how to search, update, and filter these complex collections efficiently.