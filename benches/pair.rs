// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};

fn compute_window(data: &[i32]) -> i32 {
  data.windows(2).map(|x| x[0] - x[1]).sum()
}

fn compute_idx(data: &[i32]) -> i32 {
  let mut result = 0;
  for i in 0..(data.len() -1) {
    result += data[i] - data[i+1];
  }
  result
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(-100_000..100_000);
  c.bench_function("pair window", |b| b.iter(|| compute_window(black_box(&array))));
  c.bench_function("pair idx", |b| b.iter(|| compute_idx(black_box(&array))));
}
