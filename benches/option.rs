// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};

fn option_sum(data: &[Option<String>]) -> usize {
  data.iter().filter_map(|x| x.as_ref().and_then(|s| Some(s.len()))).sum()
}

fn option_idx(data: &[Option<String>]) -> usize {
  let mut result = 0;
  for x in data {
    if x.is_some() {
      result += x.as_ref().unwrap().len();
    }
  }
  result
}

fn option_match(data: &[Option<String>]) -> usize {
  let mut result = 0;
  for x in data {
    result += match x {
      Some(s) => s.len(),
      _ => 0,
    }
  }
  result
}

fn option_match_sum(data: &[Option<String>]) -> usize {
  data.iter().map(|x| match x {
    Some(s) => s.len(),
    _ => 0,
  }).sum()
}

fn option_map_or(data: &[Option<String>]) -> usize {
  data.iter().map(|x| x.as_ref().map_or(0, |s| s.len())).sum()
}

pub fn benchmark(c: &mut Criterion) {
  let array: [Option<String>; 10_000] = rust_bench::random_string_array(0);
  c.bench_function("option sum", |b| b.iter(|| option_sum(black_box(&array))));
  c.bench_function("option idx", |b| b.iter(|| option_idx(black_box(&array))));
  c.bench_function("option match", |b| b.iter(|| option_match(black_box(&array))));
  c.bench_function("option match sum", |b| b.iter(|| option_match_sum(black_box(&array))));
  c.bench_function("option map_or", |b| b.iter(|| option_map_or(black_box(&array))));
}