// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};

fn sum_via_index(data: &[i32]) -> i32 {
  let mut sum = 0;
  for i in 0..data.len() {
    sum += data[i];
  }
  sum
}

fn sum_via_for(data: &[i32]) -> i32 {
  let mut sum = 0;
  for i in data {
    sum += i;
  }
  sum
}

fn sum_via_sum(data: &[i32]) -> i32 {
  data.iter().sum()
}

fn sum_via_fold(data: &[i32]) -> i32 {
  data.iter().fold(0, |acc, i| acc + i)
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(-100_000..100_000, 0);
  let vec = array.to_vec();
  c.bench_function("element index array", |b| b.iter(|| sum_via_index(black_box(&array))));
  c.bench_function("element for array", |b| b.iter(|| sum_via_for(black_box(&array))));
  c.bench_function("element sum array", |b| b.iter(|| sum_via_sum(black_box(&array))));
  c.bench_function("element fold array", |b| b.iter(|| sum_via_fold(black_box(&array))));

  c.bench_function("element index vec", |b| b.iter(|| sum_via_index(black_box(&vec))));
  c.bench_function("element for vec", |b| b.iter(|| sum_via_for(black_box(&vec))));
  c.bench_function("element sum vec", |b| b.iter(|| sum_via_sum(black_box(&vec))));
  c.bench_function("element fold vec", |b| b.iter(|| sum_via_fold(black_box(&vec))));
}
