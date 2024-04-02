// Copyright by Owen O'Malley 2024

use criterion::{criterion_group, criterion_main};

mod dual;
mod dynamic;
mod elements;
mod option;
mod pair;
mod tail;

criterion_group!(benches, elements::benchmark, dual::benchmark, dynamic::benchmark, tail::benchmark,
                 pair::benchmark, option::benchmark);
criterion_main!(benches);