# Simple Calculator

This is a simple calculator library written in Rust. It provides basic arithmetic operations such as addition, subtraction, multiplication, and division.

## Functions# Simple Calculator

This is a simple calculator library written in Rust. It provides basic arithmetic operations such as addition, subtraction, multiplication, and division.

## Functions

- [`add(x: i32, y: i32) -> i32`](src/lib.rs): Returns the sum of `x` and `y`.
- [`sub(x: i32, y: i32) -> i32`](src/lib.rs): Returns the difference of `x` and `y`.# Simple Calculator

This is a simple calculator library written in Rust. It provides basic arithmetic operations such as addition, subtraction, multiplication, and division.

## Functions

- [`add(x: i32, y: i32) -> i32`](src/lib.rs): Returns the sum of `x` and `y`.
- [`sub(x: i32, y: i32) -> i32`](src/lib.rs): Returns the difference of `x` and `y`.
- [`mul(x: i32, y: i32) -> i32`](src/lib.rs): Returns the product of `x` and `y`.
- [`div(x: i32, y: i32) -> i32`](src/lib.rs): Returns the quotient of `x` and `y`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple_calculator = "0.1.0"

Then, in your Rust file:

use simple_calculator::{add, sub, mul, div};

fn main() {
    let sum = add(2, 3);
    let difference = sub(2, 3);
    let product = mul(2, 3);
    let quotient = div(2, 3);
}
```

Deciding which workspace information to collect

Collecting workspace information

Filtering to most relevant information

Sure, here's a basic README for your Rust project:

````markdown
# Simple Calculator

This is a simple calculator library written in Rust. It provides basic arithmetic operations such as addition, subtraction, multiplication, and division.

## Functions

- [`add(x: i32, y: i32) -> i32`](src/lib.rs): Returns the sum of `x` and `y`.
- [`sub(x: i32, y: i32) -> i32`](src/lib.rs): Returns the difference of `x` and `y`.
- [`mul(x: i32, y: i32) -> i32`](src/lib.rs): Returns the product of `x` and `y`.
- [`div(x: i32, y: i32) -> i32`](src/lib.rs): Returns the quotient of `x` and `y`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple_calculator = "0.1.0"
```
````

Then, in your Rust file:

```rust
use simple_calculator::{add, sub, mul, div};

fn main() {
    let sum = add(2, 3);
    let difference = sub(2, 3);
    let product = mul(2, 3);
    let quotient = div(2, 3);
}
```

## Testing

This library includes unit tests for each function. Run the tests with:

```sh
cargo test
```

```

Please replace `"0.1.0"` with the actual version of your library.
```
