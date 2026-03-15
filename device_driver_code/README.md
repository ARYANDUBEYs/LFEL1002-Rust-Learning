# RUST : Virtual GPIO Controller
A Rust-based hardware simulation that demonstrates how to control GPIO pins using the embedded-hal standard. This project allows you to "toggle" virtual LEDs through a command-line interface.

## Features
- **Standardized Logic** : Uses the industry-standard `embedded-hal` traits, ensuring the logic is identical to real-world firmware.
- **Hardware Simulation** : Mimics physical GPIO pins using a state-tracking system (`bool`) and console logging.
- **Real-time Interaction** : Features an interactive CLI that allows the user to choose which virtual LED to toggle.
- **Platform Independent** : Unlike "Bare Metal" code, this runs on any operating system (Windows, Mac, Linux) because it uses standard `threading` for delays.
- **Safety & Validation** : Implements strict Rust type-checking to handle unexpected inputs and ensure consistent function returns.

## How it Works
- **The Hardware Layer (`MyGpioPin`)**: Implements `set_high()` and `set_low()`. Instead of moving electrons, it prints to your screen.
- **The Driver Layer (`GpioController`)** : Manages the three pins and provides easy functions like `turn_on_led`.
- **The Application Layer (`main`)** : Handles user input and timing.

## Prerequisites
- To run this, you need Rust installed and the following dependencies in your `Cargo.toml`:

Ini, TOML:
```
[dependencies]
embedded-hal = "0.2.7"
```
## Usage
1. Clone this repository.
2. Run the program using Cargo:
```bash
cargo run
```
3. When prompted, enter A, B, or C.

## What I Learned
- **Hardware Abstraction** : I learned that I can use hardware-specific traits (`embedded-hal`) even on a non-hardware device by creating a simulator.
- **Project Documentation** : I practiced summarizing technical logic into a clear `README` for other developers to read.
- **Refined Architecture** : I understood that removing `cortex_m` made the code "Portable," meaning it can run on any OS while keeping the logic the same.

## Scholarship Goals
- **Code Portability** : I am practicing how to separate "Logic" (the controller) from "Environment" (the timing/input) so I can swap them easily.
- **Iterative Improvement** : I aim to look back at my older code and see how much I've improved by solving these "Incompatible Type" errors.