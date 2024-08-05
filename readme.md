# Rust Tutorial: Stack vs. Heap Allocation

This repository provides a basic tutorial on memory management in Rust, specifically focusing on the difference between
stack and heap allocation. The examples demonstrate how to safely allocate large data structures on the heap to avoid
stack overflow errors.

## Table of Contents

- [Introduction](#introduction)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Understanding Stack vs. Heap](#understanding-stack-vs-heap)
    - [Stack Allocation](#stack-allocation)
    - [Heap Allocation with `Vec`](#heap-allocation-with-vec)
- [Running the Code](#running-the-code)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Rust's memory management system is one of its key features, ensuring memory safety without a garbage collector. This
tutorial explores the difference between stack and heap memory, with a focus on avoiding stack overflow errors by
correctly allocating large data on the heap.

## Prerequisites

- **Rust**: Ensure you have Rust installed. If not, you can install it
  from [rust-lang.org](https://www.rust-lang.org/learn/get-started).
- **Cargo**: Cargo is the Rust package manager, which comes with Rust by default.

## Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/kavicastelo/rust_tutorial.git
   cd rust_tutorial
   ```

2. **Build the project**:
   ```sh
   cargo build
   ```

3. **Run the project**:
   ```sh
   cargo run
   ```

## Understanding Stack vs. Heap

### Stack Allocation

The stack is used for storing local variables and function call frames. It’s fast but limited in size. Allocating large
data structures directly on the stack can cause stack overflow errors.

```rust
fn main() {
    // This might cause a stack overflow
    let large_array = [0u8; 10_000_000];
    println!("Large array on stack");
}
```

### Heap Allocation with `Vec`

To safely handle large data structures, you can allocate memory on the heap using a `Vec`. This prevents stack overflow
by avoiding large stack allocations.

```rust
fn main() {
    // Directly allocate a large vector on the heap
    let large_data = vec![0u8; 10_000_000]; // Vector allocated on the heap
    println!("Large data length: {}", large_data.len());
}
```

In this example, the `vec!` macro dynamically allocates memory on the heap, ensuring that the program doesn't encounter
stack overflow issues.

## Running the Code

To run the examples provided:

1. Stack Allocation Example:
   Uncomment the stack allocation example in `src/main.rs` and run the program:
   ```sh
   cargo run
   ```
2. Heap Allocation with `Vec` Example:
   The heap allocation example is the default. Simply run:
   ```sh
   cargo run
   ```
   This will allocate a large data structure on the heap and print its length to the console.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -m 'Add YourFeature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
