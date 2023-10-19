# Micrograd in Rust

![License](https://img.shields.io/badge/license-MIT-blue.svg)

<!-- ![Build](https://img.shields.io/badge/build-passing-brightgreen.svg) -->

<!-- Update the above badges links to point to your actual data, or use shields.io to generate new ones. -->

Rust version of [Karpathy/micrograd](https://github.com/karpathy/micrograd)

## Features

**TODO**

- [ ] **Automatic Differentiation**: Compute gradients automatically with forward and backward passes.
- [ ] **Neural Network Building Blocks**: Basic tools to create your own neural network architectures.
- [ ] **Computation Graphs**: Visualize your operations and model structure for better understanding and debugging.

## Installation

Make sure you have Rust and Cargo installed. Then, add `micrograd` to your `Cargo.toml` file:

```toml
[dependencies]
micrograd = "0.1.0"

```

## Quick Start

Here's how to create a simple computation graph and compute gradients:

```rust
use micrograd::prelude::*;

fn main() {
    // Create some values
    let a = Value::new(1.0);
    let b = Value::new(2.0);
    let c = Value::new(3.0);

    // Perform computations
    let result = &a * &b + &c;

    // Compute gradients
    result.backward();

    // Access gradients
    println!("{}, {}, {}", a.grad(), b.grad(), c.grad());
}
```
