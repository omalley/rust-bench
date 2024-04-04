// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};

// Compare the various ways to handle Option values.

// These functions take a slice of Option<String> and compute the
// sum of the lengths of the String values.

/// Iterate through the data, apply filter_map to drop the None values
/// and keep the string lengths. Finally sum is applied.
fn filter_map(data: &[Option<String>]) -> usize {
  data.iter().filter_map(|x|
     x.as_ref().and_then(|s| Some(s.len()))).sum()
}

/// A for loop and if to test whether it is a String.
fn for_if(data: &[Option<String>]) -> usize {
  let mut result = 0;
  for x in data {
    if x.is_some() {
      result += x.as_ref().unwrap().len();
    }
  }
  result
}

/// As above, but with if let to find the Strings.
fn for_if_let(data: &[Option<String>]) -> usize {
  let mut result = 0;
  for x in data {
    if let Some(s) = x {
      result += s.len();
    }
  }
  result
}

/// A for loop with match to find strings.
fn for_match(data: &[Option<String>]) -> usize {
  let mut result = 0;
  for x in data {
    match x {
      Some(s) => result += s.len(),
      _ => {},
    }
  }
  result
}

/// Iteration, match to find the Strings, and sum.
fn match_sum(data: &[Option<String>]) -> usize {
  data.iter().map(|x| match x {
    Some(s) => s.len(),
    _ => 0,
  }).sum()
}

/// Uses Option.map_or, which provides a default value if the input is None
/// or applies a lambda when it has a value.
fn map_or(data: &[Option<String>]) -> usize {
  data.iter().map(|x| 
    x.as_ref().map_or(0, |s| s.len())).sum()
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