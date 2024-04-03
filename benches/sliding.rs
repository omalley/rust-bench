// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};

// These functions use a sliding window and compute the 
// diference between adjacent elements and then sum the differences.

/// Use high level window function, map, and sum.
fn compute_window(data: &[i32]) -> i32 {
  data.windows(2).map(|x| x[0] - x[1]).sum()
}

/// Use the equivalent for loop.
fn compute_for(data: &[i32]) -> i32 {
  let mut result = 0;
  for i in 0..(data.len() -1) {
    result += data[i] - data[i+1];
  }
  result
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(-100_000..100_000, 0);
  c.bench_function("sliding window", |b| b.iter(|| compute_window(black_box(&array))));
  c.bench_function("sliding for", |b| b.iter(|| compute_for(black_box(&array))));
}
