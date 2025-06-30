use rand::prelude::*;
use std::time::Instant;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn main() {
    println!("SIMD vs Non-SIMD Mean Calculation Benchmark");
    println!("============================================");
    
    // Test different sizes
    let sizes = [500, 1000, 50000, 1000000, 100000000];
    
    // Print table header
    println!("{:<12} {:<15} {:<15} {:<15} {:<12} {:<12} {:<12}", 
        "Size", "Scalar (ns)", "SIMD (ns)", "Chunk (ns)", 
        "SIMD Speed", "Chunk Speed", "Accuracy");
    println!("{}", "-".repeat(95));
    
    for &size in &sizes {
        let results = benchmark_size(size);
        
        // Calculate speedups
        let scalar_ns = results.scalar_time.as_nanos() as f64;
        let simd_ns = results.simd_time.as_nanos() as f64;
        let chunk_ns = results.chunk_time.as_nanos() as f64;
        
        let simd_speedup = if simd_ns > 0.0 { scalar_ns / simd_ns } else { 0.0 };
        let chunk_speedup = if chunk_ns > 0.0 { scalar_ns / chunk_ns } else { 0.0 };
        
        // Calculate max difference for accuracy
        let max_diff = (results.scalar_mean - results.simd_mean).abs()
            .max((results.scalar_mean - results.chunk_mean).abs());
        
        println!("{:<12} {:<15} {:<15} {:<15} {:<12.2}x {:<12.2}x {:<12.2e}", 
            format_size(size),
            scalar_ns as u64,
            simd_ns as u64,
            chunk_ns as u64,
            simd_speedup,
            chunk_speedup,
            max_diff);
    }
    
    println!();
    println!("Legend:");
    println!("- SIMD Speed: Speedup factor of SIMD vs Scalar");
    println!("- Chunk Speed: Speedup factor of Chunks vs Scalar");
    println!("- Accuracy: Maximum difference between implementations");
}

struct BenchmarkResults {
    scalar_mean: f32,
    simd_mean: f32,
    chunk_mean: f32,
    scalar_time: std::time::Duration,
    simd_time: std::time::Duration,
    chunk_time: std::time::Duration,
}

fn benchmark_size(size: usize) -> BenchmarkResults {
    // Generate random floats between 20 and 100
    let mut rng = thread_rng();
    let data: Vec<f32> = (0..size)
        .map(|_| rng.gen_range(20.0..100.0))
        .collect();
    
    // Warmup runs
    for _ in 0..3 {
        let _ = calculate_mean_scalar(&data);
        let _ = calculate_mean_simd(&data);
        let _ = calculate_mean_chunks(&data);
    }
    
    // Benchmark scalar implementation
    let start = Instant::now();
    let scalar_mean = calculate_mean_scalar(&data);
    let scalar_time = start.elapsed();
    
    // Benchmark SIMD implementation
    let start = Instant::now();
    let simd_mean = calculate_mean_simd(&data);
    let simd_time = start.elapsed();
    
    // Benchmark using chunks
    let start = Instant::now();
    let chunk_mean = calculate_mean_chunks(&data);
    let chunk_time = start.elapsed();
    
    BenchmarkResults {
        scalar_mean,
        simd_mean,
        chunk_mean,
        scalar_time,
        simd_time,
        chunk_time,
    }
}

fn format_size(size: usize) -> String {
    match size {
        n if n >= 1_000_000 => format!("{}M", n / 1_000_000),
        n if n >= 1_000 => format!("{}K", n / 1_000),
        n => n.to_string(),
    }
}

/// Calculate mean using scalar operations
fn calculate_mean_scalar(data: &[f32]) -> f32 {
    let sum: f32 = data.iter().sum();
    sum / data.len() as f32
}

/// Calculate mean using SIMD operations (AVX on x86_64)
#[cfg(target_arch = "x86_64")]
fn calculate_mean_simd(data: &[f32]) -> f32 {
    if is_x86_feature_detected!("avx") {
        unsafe { calculate_mean_simd_avx(data) }
    } else {
        calculate_mean_scalar(data)
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn calculate_mean_simd(data: &[f32]) -> f32 {
    calculate_mean_chunks(data)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
unsafe fn calculate_mean_simd_avx(data: &[f32]) -> f32 {
    let mut sum = _mm256_setzero_ps();
    let mut i = 0;
    
    // Process 8 floats at a time using AVX
    while i + 8 <= data.len() {
        unsafe {
            let chunk = _mm256_loadu_ps(data.as_ptr().add(i));
            sum = _mm256_add_ps(sum, chunk);
        }
        i += 8;
    }
    
    // Extract the sum from the AVX register
    let mut result = [0.0f32; 8];
    unsafe {
        _mm256_storeu_ps(result.as_mut_ptr(), sum);
    }
    let simd_sum: f32 = result.iter().sum();
    
    // Handle remaining elements
    let remaining_sum: f32 = data[i..].iter().sum();
    
    (simd_sum + remaining_sum) / data.len() as f32
}

/// Calculate mean using chunked approach (compiler auto-vectorization)
fn calculate_mean_chunks(data: &[f32]) -> f32 {
    const CHUNK_SIZE: usize = 8;
    let chunks = data.chunks_exact(CHUNK_SIZE);
    let remainder = chunks.remainder();
    
    let chunk_sum: f32 = chunks
        .map(|chunk| chunk.iter().sum::<f32>())
        .sum();
    
    let remainder_sum: f32 = remainder.iter().sum();
    
    (chunk_sum + remainder_sum) / data.len() as f32
}
