// Copyright by Owen O'Malley 2024

use criterion::{criterion_group, criterion_main};
extern crate num_derive;

mod dispatch;
mod dual;
mod branching;
mod elements;
mod option;
mod sliding;
mod tail;

criterion_group!(benches, branching::benchmark, dispatch::benchmark, dual::benchmark,
  elements::benchmark, option::benchmark, sliding::benchmark, tail::benchmark);
criterion_main!(benches);