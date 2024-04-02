// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};

fn dual_via_zip(left: &[i32], right: &[i32]) -> i32 {
  left.iter().zip(right.iter())
      .map(|(l,r)| l * r)
      .sum()
}

fn dual_via_index(left: &[i32], right: &[i32]) -> i32 {
  let mut result = 0;
  for i in 0..left.len() {
    result += left[i] * right[i];
  }
  result
}

fn dual_via_tail(left: &[i32], right: &[i32], previous: i32) -> i32 {
  if left.is_empty() {
    previous
  } else {
    dual_via_tail(&left[1..], &right[1..], previous + left[0] * right[0])
  }
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(-100_000..100_000);
  c.bench_function("dual zip", |b| b.iter(|| dual_via_zip(black_box(&array), black_box(&array))));
  c.bench_function("dual index", |b| b.iter(|| dual_via_index(black_box(&array), black_box(&array))));
  c.bench_function("dual tail", |b| b.iter(|| dual_via_tail(black_box(&array), black_box(&array), 0)));
}
