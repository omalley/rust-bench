// Copyright by Owen O'Malley 2024
use std::ops::Range;
use rand_chacha::ChaChaRng;
use rand_chacha::rand_core::SeedableRng;
use rand::Rng;

pub fn random_array<const SIZE:usize>(range: Range<i32>, seed: u64) -> [i32; SIZE] {
  let mut rng: ChaChaRng = SeedableRng::seed_from_u64(seed);
  core::array::from_fn(|_| rng.gen_range(range.clone()))
}

pub fn random_string_array<const SIZE:usize>(seed: u64) -> [Option<String>; SIZE] {
  let mut rng: ChaChaRng = SeedableRng::seed_from_u64(seed);
  core::array::from_fn(|_| {
    if rng.gen_bool(0.5) {
      None
    } else {
      let len: usize = rng.gen_range(0..20);
      Some(format!("{:1$}", "", len))
    }
  })
}