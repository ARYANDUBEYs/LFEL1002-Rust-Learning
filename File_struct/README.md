# RUST : Using File(Struct) 
This is very basic rust program that demonstrates how to create file handle and write data to the disk at the byte level.

## Features
-**File Handle Creation**: Uses `File::create` for persistent access
-**IO Prelude**: Uses `std::io::prelude::*`to automatically import all the basic tools needed for writing.
-**Safe Writing**: Uses `write_all` to ensure every byte reaches the disk safely by preventing 'short writes'. 

## Usage
Run the program using cargo:
```bash
cargo run
```

## What I learned?
-**The Prelude**: We use `*` to bring traits to local scope.
-**Byte-Level Operations**: Used the `b""` prefix to convert strings into byte slices.
-**Persistent File Handle**: Understood that `File::create` provides a handle which stays open.
-**Prevent Short Writes**: Learned how to prevent "short writes" using `write_all`.

## Scholorship Goals
This project proves I can manage system resources efficiently. My goal is to master Rust's performance-focused tools to contribute to secure, open-source Linux Utilities.