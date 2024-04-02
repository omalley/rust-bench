// Copyright by Owen O'Malley 2024

use rand::Rng;
use std::ops::Range;

pub fn random_array<const SIZE:usize>(range: Range<i32>) -> [i32; SIZE] {
  let mut rng = rand::thread_rng();
  core::array::from_fn(|_| rng.gen_range(range.clone()))
}

pub fn random_string_array<const SIZE:usize>() -> [Option<String>; SIZE] {
  let mut rng = rand::thread_rng();
  core::array::from_fn(|_| {
    if rng.gen_bool(0.5) {
      None
    } else {
      let len: usize = rng.gen_range(0..20);
      Some(format!("{:1$}", "", len))
    }
  })
}