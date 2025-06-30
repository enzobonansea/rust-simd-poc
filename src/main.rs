use rand::prelude::*;
use std::time::Instant;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn main() {
    println!("SIMD vs Non-SIMD Mean Calculation Benchmark");
    println!("============================================");
    
    // Generate 1 million random floats between 20 and 100
    const SIZE: usize = 1_000_000;
    let mut rng = thread_rng();
    let data: Vec<f32> = (0..SIZE)
        .map(|_| rng.gen_range(20.0..100.0))
        .collect();
    
    println!("Generated {} random floats between 20.0 and 100.0", SIZE);
    println!();
    
    // Warmup runs
    for _ in 0..3 {
        let _ = calculate_mean_scalar(&data);
        let _ = calculate_mean_simd(&data);
    }
    
    // Benchmark scalar implementation
    let start = Instant::now();
    let scalar_mean = calculate_mean_scalar(&data);
    let scalar_duration = start.elapsed();
    
    // Benchmark SIMD implementation
    let start = Instant::now();
    let simd_mean = calculate_mean_simd(&data);
    let simd_duration = start.elapsed();
    
    // Benchmark using chunks (vectorized but not explicit SIMD)
    let start = Instant::now();
    let chunk_mean = calculate_mean_chunks(&data);
    let chunk_duration = start.elapsed();
    
    // Results
    println!("Results:");
    println!("--------");
    println!("Scalar Mean: {:.6}", scalar_mean);
    println!("SIMD Mean:   {:.6}", simd_mean);
    println!("Chunk Mean:  {:.6}", chunk_mean);
    println!("Max Diff:    {:.6}", 
        (scalar_mean - simd_mean).abs().max((scalar_mean - chunk_mean).abs()));
    println!();
    
    println!("Performance:");
    println!("------------");
    println!("Scalar Time: {:?}", scalar_duration);
    println!("SIMD Time:   {:?}", simd_duration);
    println!("Chunk Time:  {:?}", chunk_duration);
    println!();
    
    // Calculate speedups
    let scalar_ns = scalar_duration.as_nanos() as f64;
    let simd_ns = simd_duration.as_nanos() as f64;
    let chunk_ns = chunk_duration.as_nanos() as f64;
    
    if simd_ns > 0.0 {
        let simd_speedup = scalar_ns / simd_ns;
        println!("SIMD Speedup: {:.2}x", simd_speedup);
        
        if simd_speedup > 1.0 {
            println!("✅ SIMD is {:.1}% faster than scalar!", (simd_speedup - 1.0) * 100.0);
        } else {
            println!("❌ Scalar is {:.1}% faster than SIMD", (1.0 / simd_speedup - 1.0) * 100.0);
        }
    }
    
    if chunk_ns > 0.0 {
        let chunk_speedup = scalar_ns / chunk_ns;
        println!("Chunk Speedup: {:.2}x", chunk_speedup);
        
        if chunk_speedup > 1.0 {
            println!("✅ Chunks are {:.1}% faster than scalar!", (chunk_speedup - 1.0) * 100.0);
        } else {
            println!("❌ Scalar is {:.1}% faster than chunks", (1.0 / chunk_speedup - 1.0) * 100.0);
        }
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
