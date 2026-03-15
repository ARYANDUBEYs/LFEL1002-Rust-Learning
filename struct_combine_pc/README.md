# RUST : Component Composition in Rust
A demonstration of **Struct Composition** and **Method Delegation**, modeling a `Computer` that owns and interacts with a `Monitor` . In complex systems, we don't build one giant object; we build small, specialized objects and combine them. This project shows how a "Parent" struct (`Computer`) can own a "Child" struct (`Monitor`) and trigger its behaviors using **delegation**.

## Features
- **Struct Composition**: Demonstrates "Has-A" relationships where one struct contains another as a field.
- **Method Delegation**: The `Computer` calls the `Monitor`'s methods to handle specific tasks like displaying text.
- **Efficient String Slices (`&str`)**: Uses string slices for method arguments to allow flexible, read-only access to text without extra memory allocation.
- **Ownership Transfer**: Shows how creating the parent struct takes ownership of the child components.

## How it Works: The Flow
1. **Initialize Child**: A `Monitor` is created with a brand name.
2. **Assemble Parent**: The `Monitor` is moved into the `Computer` struct during initialization.
3. **Trigger Logic**: The `main` function calls the parent's `startup()` method.
4. **Delegate Task**: The `Computer` reaches into its `screen` field to call `display()`.

## Usage
Since the components are assembled in the code, simply run the project:
```bash
cargo run
# Output:
# Starting up Gaming Rig...
# Monitor [Dell] showing: Hello World!
```

## What I Learned
- **Nesting Structs** : I learned how to use a custom struct as a data type for a field inside another struct.
- **The &str Type** : I discovered that `&str` (string slice) is a lightweight "view" into text, making it perfect for function arguments.
- **Delegation Pattern** : I practiced how a parent struct can "ask" its child components to perform work using dot notation (e.g., `self.screen.display()`).
- **Ownership in Composition** : I understood that when the child is placed inside the parent, the parent becomes responsible for the child's data.

## Scholarship Goals
- **System Architecture** : My goal is to learn how to break large hardware concepts into smaller, reusable software structs.
- **Memory Optimization** : I am practicing when to use `String` (for storage) versus `&str` (for functions) to keep my programs fast.
- **API Design** : I aim to create "clean" parent methods that hide the complex details of the children from the user.