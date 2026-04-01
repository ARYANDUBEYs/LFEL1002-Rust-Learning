# RUST : Dynamic Fibonacci Calculator (Vector Edition)
A Rust program that uses Dynamic Programming to calculate the $n^{th}$ Fibonacci number by storing intermediate results in a heap-allocated vector.

## Features
- **Memory Pre-allocation**: Uses `Vec::with_capacity` to reserve memory upfront, preventing the program from having to resize the "bucket" as it grows.
- **Indexed Access**: Retrieves values directly using their position (index) in the vector for maximum speed.
- **Dynamic Programming**: Demonstrates a bottom-up approach to solving mathematical sequences by building on sub-problems.
- **64-bit Precision**: Uses `u64` to allow for much larger Fibonacci numbers than a standard `i32` could handle.

## Usage
To calculate the $10^{th}$ Fibonacci number (or any number you set in `main`):

```bash
cargo run
```

## What I Learned
- **Vector Management**: I learned how to use `.push()` to add data to a list and how to access specific items using [index].
- **Capacity vs Length**: I discovered that `with_capacity` makes the program more efficient by telling the computer exactly how much memory I need before I start.
- **Casting Types**: I learned how to use `as usize` to convert a `u64` number into an index that the Vector can understand.
- **Inclusive Ranges**: I practiced using `2..=n` to ensure the loop runs all the way to the final number, including n itself.

## Scholarship Goals
- *Optimization*: My goal is to understand when it is better to store data in a Vector versus just using two variables to save memory.
- *Algorithmic Complexity*: I am learning how storing values (Space Complexity) can sometimes make logic easier to read compared to pure iteration.
*- *Safety with Indices*: I aim to master how to prevent "out-of-bounds" errors when accessing specific parts of a Vector.