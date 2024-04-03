// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};
use tailcall::tailcall;

// Study how the compiler deals with tail recursion with
// functions that compute the sum of the array.

/// Match against the slices and recurse with a final addition.
// tailcall rejects this because of the final addition.
fn sum_via_match(data: &[i32]) -> i32 {
  match data {
    [] => 0,
    [val] => *val,
    [val, ..] => val + sum_via_match(&data[1..]),
  }
}

/// Match against the slices with an accumulator to avoid the final
/// addition.
#[tailcall]
fn sum_via_match_accum(data: &[i32], previous: i32) -> i32 {
  match data {
    [] => previous,
    [val] => *val + previous,
    [val, ..] => sum_via_match_accum(&data[1..], val + previous),
  }
}

/// Same as the previous, but with only two match branches. The
/// middle pattern is handled as a special case of the final 
/// pattern. This is significantly faster.
#[tailcall]
fn sum_via_match2_accum(data: &[i32], previous: i32) -> i32 {
  match data {
    [] => previous,
    [val, ..] => sum_via_match2_accum(&data[1..], val + previous),
  }
}

/// Match on the length of the data rather than the front of the slice.
#[tailcall]
fn sum_via_len_match_accum(data: &[i32], previous: i32) -> i32 {
  match data.len() {
    0 => previous,
    1 => data[0] + previous,
    _ => sum_via_len_match_accum(&data[1..], data[0] + previous),
  }
}

/// Take the manual approach of passing in the start index and an
/// accumulator.
#[tailcall]
fn sum_via_if_idx_accum(data: &[i32], i: usize, previous: i32) -> i32 {
  if i < data.len() {
    sum_via_if_idx_accum(data, i + 1, data[i] + previous)
  } else {
    previous
  }
}

/// Use an if instead of the match and use an accumulator.
#[tailcall]
fn sum_via_if_accum(data: &[i32], previous: i32) -> i32 {
  if data.is_empty() {
    previous
  } else {
    sum_via_if_accum(&data[1..], data[0] + previous)
  }
}

/// Finally, use an if branch without the accumulator.
// tailcall rejects this.
fn sum_via_if(data: &[i32]) -> i32 {
  if data.is_empty() {
    0
  } else {
    data[0] + sum_via_if(&data[1..])
  }
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(-100_000..100_000, 0);
  c.bench_function("tail match", |b| b.iter(|| sum_via_match(black_box(&array))));
  c.bench_function("tail match accum", |b| b.iter(|| sum_via_match_accum(black_box(&array), 0)));
  c.bench_function("tail match2 accum", |b| b.iter(|| sum_via_match2_accum(black_box(&array), 0)));
  c.bench_function("tail len match accum", |b| b.iter(|| sum_via_len_match_accum(black_box(&array), 0)));
  c.bench_function("tail if idx accum", |b| b.iter(|| sum_via_if_idx_accum(black_box(&array), 0, 0)));
  c.bench_function("tail if accum", |b| b.iter(|| sum_via_if_accum(black_box(&array), 0)));
  c.bench_function("tail if", |b| b.iter(|| sum_via_if(black_box(&array))));
}
