# Performance Test: Python vs Rust vs Go

This repository contains a simple performance comparison between Python, Rust, and Go, demonstrating the performance characteristics of these languages using a common computational task.

## Overview

The project implements the same algorithm in three different languages to compare their execution performance. The algorithm performs a series of calculations in a loop, including square operations and conditional additions/subtractions.

## Versions

- Python 3.12.9
- Rust 1.85.0
- Go 1.24.0

## Files

- `main.py` - Python implementation
- `main.rs` - Rust implementation
- `main.go` - Go implementation

## How to Run

### Python Version

```bash
python3 main.py
```

### Rust Version

```bash
rustc main.rs
./main
```

### Go Version

```bash
go run main.go
```

## Implementation Details

Each implementation follows the same algorithm:

1. Performs a loop from 0 to 100,000,000
2. For each number:
   - Adds the square of the number to the total
   - If the number is even, adds it to the total
   - If the number is odd, subtracts it from the total
3. Measures and reports the execution time

## Integer Handling

One important aspect of this performance test is how each language handles integers:

- Python uses arbitrary-precision integers (no fixed size limit), which means calculations won't overflow but may use more memory
- Rust uses u64 (unsigned 64-bit integers), providing a range from 0 to 18,446,744,073,709,551,615
- Go uses int64 (signed 64-bit integers), ranging from -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

These differences in integer handling affect both the performance and results:

- Python's results will be mathematically accurate but slower due to arbitrary-precision arithmetic
- Rust and Go may show different results due to integer overflow behavior
- The square operation in particular can easily exceed the 64-bit limit with large numbers

## Performance Comparison

Run each version to compare their execution times. Typically:

- Rust version shows the best performance due to zero-cost abstractions and compile-time optimizations
- Go version demonstrates good performance with its efficient garbage collector and runtime
- Python version, while more readable and maintainable, may be slower due to its interpreted nature

Note: Actual performance may vary depending on your system specifications and load.
