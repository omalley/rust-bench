// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};

fn filter_map(data: &[Option<String>]) -> usize {
  data.iter().filter_map(|x| x.as_ref().and_then(|s| Some(s.len()))).sum()
}

fn for_if(data: &[Option<String>]) -> usize {
  let mut result = 0;
  for x in data {
    if x.is_some() {
      result += x.as_ref().unwrap().len();
    }
  }
  result
}

fn for_if_let(data: &[Option<String>]) -> usize {
  let mut result = 0;
  for x in data {
    if let Some(s) = x {
      result += s.len();
    }
  }
  result
}

fn for_match(data: &[Option<String>]) -> usize {
  let mut result = 0;
  for x in data {
    result += match x {
      Some(s) => s.len(),
      _ => 0,
    }
  }
  result
}

fn match_sum(data: &[Option<String>]) -> usize {
  data.iter().map(|x| match x {
    Some(s) => s.len(),
    _ => 0,
  }).sum()
}

fn map_or(data: &[Option<String>]) -> usize {
  data.iter().map(|x| x.as_ref().map_or(0, |s| s.len())).sum()
}

pub fn benchmark(c: &mut Criterion) {
  let array: [Option<String>; 10_000] = rust_bench::random_string_array(0);
  c.bench_function("option for if", |b| b.iter(|| for_if(black_box(&array))));
  c.bench_function("option for if_let", |b| b.iter(|| for_if_let(black_box(&array))));
  c.bench_function("option for match", |b| b.iter(|| for_match(black_box(&array))));
  c.bench_function("option filer_map", |b| b.iter(|| filter_map(black_box(&array))));
  c.bench_function("option match sum", |b| b.iter(|| match_sum(black_box(&array))));
  c.bench_function("option map_or", |b| b.iter(|| map_or(black_box(&array))));
}