// Copyright by Owen O'Malley 2024

use std::cmp::Ordering;
use std::collections::HashMap;
use criterion::{black_box, Criterion};

// Benchmark the different forms of branching and lookup.
// All of the functions map each number to a new value
// and sum the results.

/// Iterate through the data, translating the number, and 
/// propagating errors back to the caller.
fn iter_match_result(data: &[i32]) -> Result<i32,String> {
  data.iter().map(|v| match *v {
    0 => Ok(1),
    1 => Ok(2),
    2 => Ok(3),
    3 => Ok(5),
    4 => Ok(7),
    5 => Ok(11),
    6 => Ok(13),
    7 => Ok(17),
    8 => Ok(19),
    9 => Ok(23),
    _ => Err(format!("Bad digit {v}")),
  }).sum()
}

/// As above, but panic instead of an error if the data is out
/// of range.
fn iter_match(data: &[i32]) -> i32 {
  data.iter().map(|v| match *v {
    0 => 1,
    1 => 2,
    2 => 3,
    3 => 5,
    4 => 7,
    5 => 11,
    6 => 13,
    7 => 17,
    8 => 19,
    9 => 23,
    _ => panic!("Bad digit {v}"),
  }).sum()
}

/// Use a for loop to access the data.
fn for_match(data: &[i32]) -> i32 {
  let mut result = 0;
  for v in data {
    result += match *v {
      0 => 1,
      1 => 2,
      2 => 3,
      3 => 5,
      4 => 7,
      5 => 11,
      6 => 13,
      7 => 17,
      8 => 19,
      9 => 23,
      _ => panic!("Bad digit {v}"),
    }
  }
  result
}

/// Iterate through the data and convert each number using
/// if then else.
fn iter_if(data: &[i32]) -> i32 {
  data.iter().map(|v| {
    if *v == 0 {
      1
    } else if *v == 1 {
      2
    } else if *v == 2 {
      3
    } else if *v == 3 {
      5
    } else if *v == 4 {
      7
    } else if *v == 5 {
      11
    } else if *v == 6 {
      13
    } else if *v == 7 {
      17
    } else if *v == 8 {
      19
    } else if *v == 9 {
      23
    } else {
      panic!("Bad digit {v}")
    }}).sum()
}

/// Use a for loop and if statements.
fn for_if(data: &[i32]) -> i32 {
  let mut result = 0;
  for v in data {
    if *v == 0 {
      result += 1;
    } else if *v == 1 {
      result += 2;
    } else if *v == 2 {
      result += 3;
    } else if *v == 3 {
      result += 5;
    } else if *v == 4 {
      result += 7;
    } else if *v == 5 {
      result += 11;
    } else if *v == 6 {
      result += 13;
    } else if *v == 7 {
      result += 17;
    } else if *v == 8 {
      result += 19;
    } else if *v == 9 {
      result += 23;
    } else {
      panic!("Bad digit {v}")
    }
  }
  result
}

/// Iterate and use an array to do the translation.
fn lookup_array(data: &[i32], map: &[i32]) -> i32 {
  data.iter().map(|v| map[*v as usize]).sum()
}

/// Iterate and use a hash map to do the translation.
fn lookup_hashmap(data: &[i32], map: &HashMap<i32,i32>) -> i32 {
  data.iter().map(|v| map.get(v).expect("bad digit")).sum()
}

const MID: i32 = 50_000;

fn cmp_bench(data: &[i32]) -> (usize, usize, usize) {
  let mut less = 0;
  let mut equal = 0;
  let mut greater = 0;
  for val in data {
    match val.cmp(&MID) {
      Ordering::Less => less += 1,
      Ordering::Equal => equal += 1,
      Ordering::Greater => greater += 1,
    }
  }
  (less, equal, greater)
}

fn if_bench(data: &[i32]) -> (usize, usize, usize) {
  let mut less = 0;
  let mut equal = 0;
  let mut greater = 0;
  for val in data {
    if *val < MID {
      less += 1;
    } else if *val == MID {
      equal += 1;
    } else {
      greater += 1;
    }
  }
  (less, equal, greater)
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(0..10, 0);
  let trans = [1, 2, 3, 5, 7, 11, 13, 17, 19, 23];
  let map = HashMap::from([(0,1), (1, 2), (2, 3), (3, 5), (4, 7), (5, 11),
    (6, 13), (7, 17), (8, 19), (9, 23)]);
  c.bench_function("branching iter match", |b| b.iter(|| iter_match(black_box(&array))));
  c.bench_function("branching iter match result", |b| b.iter(|| iter_match_result(black_box(&array))));
  c.bench_function("branching for match", |b| b.iter(|| for_match(black_box(&array))));
  c.bench_function("branching iter if", |b| b.iter(|| iter_if(black_box(&array))));
  c.bench_function("branching for if", |b| b.iter(|| for_if(black_box(&array))));
  c.bench_function("lookup array", |b| b.iter(|| lookup_array(black_box(&array), black_box(&trans))));
  c.bench_function("lookup hashmap", |b| b.iter(|| lookup_hashmap(black_box(&array), black_box(&map))));

  let array: [i32; 10_000] = rust_bench::random_array(0..(MID * 2), 0);
  c.bench_function("branching cmp", |b| b.iter(|| cmp_bench(black_box(&array))));
  c.bench_function("branching if", |b| b.iter(|| if_bench(black_box(&array))));
}
