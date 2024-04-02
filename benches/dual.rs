// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};

fn dual_via_zip(left: &[i32], right: &[i32]) -> i32 {
  left.iter().zip(right.iter())
      .map(|(l,r)| l * r)
      .sum()
}

fn dual_via_for(left: &[i32], right: &[i32]) -> i32 {
  let mut result = 0;
  for i in 0..left.len() {
    result += left[i] * right[i];
  }
  result
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(-100_000..100_000, 0);
  c.bench_function("dual zip", |b| b.iter(|| dual_via_zip(black_box(&array), black_box(&array))));
  c.bench_function("dual for", |b| b.iter(|| dual_via_for(black_box(&array), black_box(&array))));
}
