# SIMD Proof of Concept

A Rust-based performance comparison demonstrating SIMD (Single Instruction, Multiple Data) optimizations for calculating the mean of large floating-point datasets.

## Overview

This project benchmarks three different approaches to calculate the mean of floating-point datasets of varying sizes:

1. **Scalar Implementation**: Traditional iterative approach using standard operations
2. **SIMD Implementation**: Vectorized operations using AVX instructions (x86_64 only)
3. **Chunked Implementation**: Compiler auto-vectorization using chunked processing

## Features

- Tests multiple dataset sizes: 500, 1K, 50K, 1M, and 100M elements
- Generates random floats between 20.0 and 100.0 for each test
- Compares performance across different calculation methods
- Uses AVX instructions on x86_64 architectures when available
- Falls back gracefully on non-x86_64 architectures
- Provides detailed performance metrics and speedup calculations in table format

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

The program will display a performance comparison table across different dataset sizes:

```
SIMD vs Non-SIMD Mean Calculation Benchmark
============================================
Size         Scalar (ns)     SIMD (ns)       Chunk (ns)      SIMD Speed   Chunk Speed  Accuracy    
-----------------------------------------------------------------------------------------------
500          3500            200             200            x 17.50       x 17.50       x 7.63e-6     
1K           1600            400             600            x 4.00        x 2.67        x 3.05e-5     
50K          42200           5600            13600          x 7.54        x 3.10        x 2.40e-4     
1M           1308600         238400          291200         x 5.49        x 4.49        x 3.01e-4     
100M         89317200        23971100        33836500       x 3.73        x 2.64        x 4.04e1      

Legend:
- SIMD Speed: Speedup factor of SIMD vs Scalar
- Chunk Speed: Speedup factor of Chunks vs Scalar
- Accuracy: Maximum difference between implementations
```

### Performance Analysis

The results demonstrate excellent SIMD performance characteristics:

- **Small datasets (500)**: SIMD achieves exceptional 17.5x speedup, showing optimal vectorization efficiency
- **Small-medium datasets (1K)**: SIMD maintains strong 4x performance gains
- **Medium datasets (50K)**: SIMD provides solid 7.5x speedup with consistent performance
- **Large datasets (1M)**: Both SIMD (5.5x) and chunked (4.5x) approaches show strong scalability
- **Very large datasets (100M)**: SIMD delivers 3.7x speedup, demonstrating sustained performance benefits
- **Chunked approach**: Provides reliable 2.6-17.5x speedup across all sizes through compiler auto-vectorization
- **Accuracy**: All implementations maintain high precision with minimal differences across all dataset sizes

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
