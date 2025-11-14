// Copyright by Owen O'Malley 2024

//! Various branching benchmarks
//!
//! This category tests different forms of branching. It also
//! includes some data look up for comparison.
//!
//! # Results
//!
//! 1. Handling programming errors using panic instead of
//!     Result&lt;i32,String&gt; runs 31% faster.
//! 1. For 10 items, looking them up in an array is 8% faster than using match.
//!
//! # Details
//! See [benchmark].

use std::cmp::Ordering;
use std::collections::HashMap;
use criterion::{black_box, Criterion};

/// Iterate through the data, translating the number, and 
/// propagating errors back to the caller.
pub fn iter_match_result(data: &[i32]) -> Result<i32,String> {
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
pub fn iter_match(data: &[i32]) -> i32 {
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

/// Use a for loop and match to access the data.
pub fn for_match(data: &[i32]) -> i32 {
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
pub fn iter_if(data: &[i32]) -> i32 {
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
pub fn for_if(data: &[i32]) -> i32 {
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
pub fn lookup_array(data: &[i32], map: &[i32]) -> i32 {
  data.iter().map(|v| map[*v as usize]).sum()
}

/// Iterate and use a hash map to do the translation.
pub fn lookup_hashmap(data: &[i32], map: &HashMap<i32,i32>) -> i32 {
  data.iter().map(|v| map.get(v).expect("bad digit")).sum()
}

const MID: i32 = 50_000;

/// cmp each number against a specific numer and count the values that are
/// less, equal, and greater.
pub fn cmp_bench(data: &[i32]) -> (usize, usize, usize) {
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

/// Test each number using a series of if statements against a specific numer and count the
/// values that are less, equal, and greater.
pub fn if_bench(data: &[i32]) -> (usize, usize, usize) {
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

/// Branching benchmark driver
///
/// # Branching
/// * **branching iter match:** Test iteration, map, and sum.
/// * **branching match result:** Same as above, but return a Result instead of a panic.
/// * **branching for match:** Test for loop and match.
/// * **branching iter
///
/// # Lookup
/// * **branching look array:** Look up each value in an array using indexing.
/// * **branching look hashmap:** Look up each value in a hashmap.
///
/// # Using numeric cmp
/// * **branching num cmp:*** Test the data using cmp and match the result.
/// * **branching num if:*** Test the data using if statements.
///
/// | name | N | lower | expected | upper |
/// | ---- | - | ----- | -------- | ----- |
/// | branching iter match |  |   3.1418 µs | 3.1449 µs | 3.1480 µs |
/// | branching match result |  |   4.5326 µs | 4.5365 µs | 4.5400 µs |
/// | branching for match |  |   3.2023 µs | 3.2083 µs | 3.2169 µs |
/// | branching iter if |  |   3.1981 µs | 3.2014 µs | 3.2041 µs |
/// | branching for if |  |   3.2240 µs | 3.2262 µs | 3.2286 µs |
/// | branching look array |  |   2.8228 µs | 2.8259 µs | 2.8294 µs |
/// | branching look hashmap |  |   68.506 µs | 68.549 µs | 68.596 µs |
/// | branching num cmp |  |   5.8382 µs | 5.8998 µs | 5.9746 µs |
/// | branching num if |  |   7.8823 µs | 7.8867 µs | 7.8914 µs |
pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(0..10, 0);
  let trans = [1, 2, 3, 5, 7, 11, 13, 17, 19, 23];
  let map = HashMap::from([(0,1), (1, 2), (2, 3), (3, 5), (4, 7), (5, 11),
    (6, 13), (7, 17), (8, 19), (9, 23)]);
  c.bench_function("branching iter match", |b| b.iter(|| iter_match(black_box(&array))));
  c.bench_function("branching match result", |b| b.iter(|| iter_match_result(black_box(&array))));
  c.bench_function("branching for match", |b| b.iter(|| for_match(black_box(&array))));
  c.bench_function("branching iter if", |b| b.iter(|| iter_if(black_box(&array))));
  c.bench_function("branching for if", |b| b.iter(|| for_if(black_box(&array))));
  c.bench_function("branching look array", |b| b.iter(|| lookup_array(black_box(&array), black_box(&trans))));
  c.bench_function("branching look hashmap", |b| b.iter(|| lookup_hashmap(black_box(&array), black_box(&map))));

  let array: [i32; 10_000] = rust_bench::random_array(0..(MID * 2), 0);
  c.bench_function("branching num cmp", |b| b.iter(|| cmp_bench(black_box(&array))));
  c.bench_function("branching num if", |b| b.iter(|| if_bench(black_box(&array))));
}
