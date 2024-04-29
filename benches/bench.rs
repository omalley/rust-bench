// Copyright by Owen O'Malley 2024

//! This is a set of benchmarks that I wrote to test the performance of
//! various Rust features and their relative performance. Many of the
//! benchmarks are based on similar ones that Pavan wrote for Scala.
//! Unlike Scala, Rust's higher level abstractions come at zero or low
//! cost.
//!
//! My environment is:
//!
//! * Mac OS 14.4.1
//! * Apple M2 Max, 12 cores, 38 GPU (Mac14,6)
//! * Rust 1.77.2
//!
//! As always, benchmarks run on a multi-process OS have substantial
//! jitter. (Criterion does do warm ups and many iterations, but the
//! exact numbers change.)
//!
//! The benchmarks all use random data with a fixed seed to create
//! an array of 10,000 elements. I fixed the seed to remove that
//! source of noise.
//!
//! ## Usage
//!
//! > cargo bench *pattern*
//!
//! Runs all the benchmarks with names that contain the pattern.

use criterion::{criterion_group, criterion_main};
extern crate num_derive;

pub mod dispatch;
pub mod dual;
pub mod branching;
pub mod elements;
pub mod option;
pub mod sliding;
pub mod tail;

criterion_group!(benches, branching::benchmark, dispatch::benchmark, dual::benchmark,
  elements::benchmark, option::benchmark, sliding::benchmark, tail::benchmark);
criterion_main!(benches);