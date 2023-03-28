# simple-parallel-compute

`simple-parallel-compute` is a Rust crate that provides a simple function for computing a function over a `Vec<T>` in parallel using multiple threads.


## Example

```rust
use simple_parallel_compute::compute;

fn main() {
    let result = compute(0..100, |i| i * 2);
    assert_eq!(result, (0..100).map(|i| i * 2).collect::<Vec<_>>());
}
```

## Usage

To use `simple-parallel-compute` in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
simple-parallel-compute = "0.1.0"
```

Or just run:
```bash
cargo add simple-parallel-compute`
```