# SIMD Proof of Concept

A Rust-based performance comparison demonstrating SIMD (Single Instruction, Multiple Data) optimizations for calculating the mean of large floating-point datasets.

## Overview

This project benchmarks three different approaches to calculate the mean of 1 million random floating-point numbers:

1. **Scalar Implementation**: Traditional iterative approach using standard operations
2. **SIMD Implementation**: Vectorized operations using AVX instructions (x86_64 only)
3. **Chunked Implementation**: Compiler auto-vectorization using chunked processing

## Features

- Generates 1 million random floats between 20.0 and 100.0
- Compares performance across different calculation methods
- Uses AVX instructions on x86_64 architectures when available
- Falls back gracefully on non-x86_64 architectures
- Provides detailed performance metrics and speedup calculations

## Prerequisites

- **Rust**: Version 1.70+ (uses 2024 edition)
- **Target Architecture**: x86_64 recommended for full SIMD functionality
- **CPU Features**: AVX support recommended for optimal performance

## Installation

1. Clone or download this repository
2. Navigate to the project directory:
   ```powershell
   cd simd_poc
   ```

## How to Run

### Debug Mode (Default)
```powershell
cargo run
```

### Release Mode (Recommended for Performance Testing)
```powershell
cargo run --release
```

> **Note**: Use `--release` flag for accurate performance measurements, as debug builds include significant overhead that can skew benchmark results.

### Build Only
```powershell
# Debug build
cargo build

# Release build
cargo build --release
```

## Expected Output

The program will display output similar to:

```
SIMD vs Non-SIMD Mean Calculation Benchmark
============================================
Generated 1000000 random floats between 20.0 and 100.0

Results:
--------
Scalar Mean: 59.999234
SIMD Mean:   59.999234
Chunk Mean:  59.999234
Max Diff:    0.000000

Performance:
------------
Scalar Time: 1.2345ms
SIMD Time:   0.4567ms
Chunk Time:  0.6789ms

SIMD Speedup: 2.70x
✅ SIMD is 170.0% faster than scalar!
Chunk Speedup: 1.82x
✅ Chunks are 82.0% faster than scalar!
```

## Architecture Support

- **x86_64 with AVX**: Full SIMD implementation using AVX instructions
- **x86_64 without AVX**: Falls back to chunked implementation
- **Other architectures**: Uses chunked implementation (compiler auto-vectorization)

## Performance Notes

- Performance gains vary depending on:
  - CPU architecture and available instruction sets
  - Memory bandwidth and cache hierarchy
  - Compiler optimizations
  - Dataset size and alignment

- For production use, consider:
  - Data alignment for optimal SIMD performance
  - Larger datasets to amortize setup costs
  - Architecture-specific optimizations

## Dependencies

- `rand = "0.8"` - For generating random test data

## Project Structure

```
simd_poc/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs         # Main application with benchmark implementations
├── target/             # Compiled binaries (generated)
└── README.md           # This file
```

## Technical Details

### SIMD Implementation
- Uses AVX (`_mm256_*`) intrinsics for processing 8 floats simultaneously
- Includes runtime feature detection with `is_x86_feature_detected!("avx")`
- Handles non-aligned data and remainder elements

### Safety
- SIMD code uses `unsafe` blocks as required by Rust's intrinsics
- Proper bounds checking and memory safety maintained
- Feature detection prevents execution on unsupported hardware

## Contributing

This is a proof of concept project. Feel free to experiment with:
- Different SIMD instruction sets (SSE, AVX2, AVX-512)
- Various data types and operations
- Alternative benchmarking approaches
- Cross-platform SIMD abstractions

## License

This project is provided as-is for educational and demonstration purposes.
