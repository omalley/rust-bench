// Copyright by Owen O'Malley 2024

//! Various method call benchmarks
//!
//! All the benchmarks take an 10,000 element array, apply a function to
//! it and sum the results.
//!
//! We consider three variants of calling code:
//! * Calling a method through a trait.
//! * Calling a lambda.
//! * Calling a method on an enum that branches internally.
//!
//! # Results:
//! 1. Methods called through a trait are the same as lambdas and both are
//!     indirect jumps.
//! 1. Methods on enums are much faster, because they don't have an indirect
//!     jump.
//! 1. Indirect jumps are *very* sensitive to the distinct number and ordering
//!     of targets. Having a list with 50 different classes in a random order
//!     takes 59 microseconds. Sorting that same lists by class, reduces it to
//!     9 microseconds.
//! 1. Templates, because they generate the instantiations, are much faster.
//! 1. Additionally on the _Apple M2 Max_ and _Intel Core i7-6700K_, there is
//!     an additional effect where much better performance is observed
//!     if the distribution of the indirect jump targets are evenly
//!     distributed. This was unexpected and is not observed on the
//!     _AMD Ryzen Threadripper PRO 5955WX_.
//!
//!     To study this effect, I generate an array with N*50 elements where each
//!     of the 50 classes occurs exactly
//!     N times. That array is repeated to form an array of 10,000 elements.
//!     On the Apple M2, at N=1 we get 21 microseconds, which is significantly
//!     faster than 59 microseconds for the random distribution. Surprisingly,
//!     it stays pretty constant until N=40. To verify that this is not a feature
//!     of Rust, I wrote a C++ benchmark that shows consistent results.
//!
//! # Details
//! See [benchmark].

use criterion::{black_box, Criterion};
use num_derive::FromPrimitive;
use paste::paste;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaChaRng;

const SIZE: usize = 10_000;

/// A utility function to translate the digits via a match
fn map_digit(v: i32) -> i32 {
  match v {
    0 => 1,
    1 => 4,
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

/// A trait for generating fixed i32 values.
pub trait Processor {
  fn process(&self) -> i32;
}

/// A class that stores the number to generate.
#[derive(Debug)]
pub struct GeneralProcessor {
  x: i32,
}

impl GeneralProcessor {
  /// Create an object to return the value from a given input.
  pub fn from(i: i32) -> Self {
    GeneralProcessor {x: map_digit(i)}
  }
}

impl Processor for GeneralProcessor {
  fn process(&self) -> i32 {
    self.x
  }
}

/// Define a three value enum.
#[derive(FromPrimitive)]
pub enum Enum3 {
  Value0, Value1, Value2,
}

impl Processor for Enum3 {
  fn process(&self) -> i32 {
    match self {
      Enum3::Value0 => 1,
      Enum3::Value1 => 4,
      Enum3::Value2 => 3,
    }
  }
}

/// Define a ten value enum.
#[derive(FromPrimitive)]
pub enum Enum10 {
  Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
}

impl Processor for Enum10 {
  fn process(&self) -> i32 {
    match self {
      Enum10::Zero => 1,
      Enum10::One => 4,
      Enum10::Two => 3,
      Enum10::Three => 5,
      Enum10::Four => 7,
      Enum10::Five => 11,
      Enum10::Six => 13,
      Enum10::Seven => 17,
      Enum10::Eight => 19,
      Enum10::Nine => 23,
    }
  }
}

/// A macro to make defining a lot of classes that implement Processor easier.
/// Also defines an equivalent enum.
macro_rules! define_structs {
    ( $({$id:literal, $value:literal}),* ) => {
      paste!{

        $(// Define the struct Processor<id> and its implementation
          struct [<Processor $id>] {
            // nothing
          }

          impl Processor for [<Processor $id>] {
            fn process(&self) -> i32 {
              $value
            }
        })*

        fn processor_from_i32(i: i32) -> Box<dyn Processor> {
          match i {
            $($id => Box::new([<Processor $id>]{}) as Box<dyn Processor>,)*
            _ => panic!("Bad name {i}"),
          }
        }

        // Define the equivalent enum
        #[derive(FromPrimitive)]
        pub enum Enum50 {
          $([<Value $id>],)*
        }

        impl Processor for Enum50 {
          fn process(&self) -> i32 {
            match self {
              $(Enum50::[<Value $id>] => $value,)*
            }
          }
        }
      }
    }
}

// Define Processor0 to Processor49 and Enum50.
define_structs!({0, 1}, {1, 4}, {2, 3}, {3, 5},
  {4, 7}, {5, 11}, {6, 13}, {7, 17}, {8, 19},
  {9, 23}, {10, 25}, {11, 27}, {12, 29}, {13, 31},
  {14, 33}, {15, 35}, {16, 37}, {17, 39}, {18, 41},
  {19, 43}, {20, 45}, {21, 47}, {22, 49}, {23, 51},
  {24, 53}, {25, 55}, {26, 57}, {27, 59}, {28, 61},
  {29, 63}, {30, 65}, {31, 67}, {32, 69}, {33, 71},
  {34, 73}, {35, 75}, {36, 77}, {37, 79}, {38, 81},
  {39, 83}, {40, 85}, {41, 87}, {42, 89}, {43, 91},
  {44, 93}, {45, 95}, {46, 97}, {47, 99}, {48, 101},
  {49, 103});

/// Use a single function to translate each value and sum the results
pub fn iter_func(data: &[i32], func: fn(i32) -> i32) -> i32 {
  data.iter().map(|v| func(*v)).sum()
}

/// Use a single function passed as a template parameter to translate
/// each value and sum the results.
pub fn iter_func_template<F>(data: &[i32], func: F) -> i32
  where F: Fn(i32) -> i32 {
  data.iter().map(|v| func(*v)).sum()
}

/// Use an array of functions to translate each value and sum the
/// results. The value in the input is an index into the funcs array.
pub fn iter_lambdas(data: &[i32], funcs: &[fn() -> i32]) -> i32 {
  data.iter().map(|v| funcs[*v as usize]()).sum()
}

/// Translate each value using virtual dispatch via the trait
/// and sum the results.
pub fn iter_objs(data: &[Box<dyn Processor>]) -> i32 {
  data.iter().map(|v| v.process()).sum()
}

/// Template implementation for arrays with a single type.
pub fn template_objs<T: Processor>(data: &[T]) -> i32 {
  data.iter().map(|v| v.process()).sum()
}

/// Template implementation over Boxes with a single target type.
///
/// This allows us to measure the cost of the Box and non-local access.
pub fn template_box_objs<T: Processor>(data: &[Box<T>]) -> i32 {
  data.iter().map(|v| v.process()).sum()
}

/// Dispatch benchmark driver
///
/// # Single functions
/// * **dispatch lambda:** Test a single lambda passed as a function.
/// * **dispatch func:** Test a passed in function reference.
/// * **dispatch templ lambda:** Test a passed in function using templating.
///
/// # Multiple functions
/// * **dispatch lambdas:** Test dispatching through an array of lambdas.
/// * **dispatch sort lambdas:** Test dispatching through an array of lambdas with sorted data.
/// * **dispatch templ objs:** Test with arrays of a single 10 value enum type using templates.
/// * **dispatch templ box objs:** Test with arrays of Box's of a single 10 value enum type using
///     templates, so that we can determine the performance cost of using Box.
/// * **dispatch enum N:** Test with an enum class with N values.
/// * **dispatch random objs N:** Test with an array of Box&lt;dyn [Processor]&gt; with N classes.
/// * **dispatch sorted objs N:** Test with a sorted array of Box&lt;dyn [Processor]&gt; with N
///     classes.
/// * **dispatch even objs N:** Test with an even distribution such that each N * 50 elements
///     is in the same random order and contains exactly N of each of the 50 classes.
///
/// | name | N | lower | expected | upper |
/// | ---- | - | ----- | -------- | ----- |
/// | dispatch lambda |  |   8.5299 µs | 8.5416 µs | 8.5530 µs |
/// | dispatch func |  |   8.6223 µs | 8.6357 µs | 8.6496 µs |
/// | dispatch templ lambda |  |   3.2878 µs | 3.2925 µs | 3.2974 µs |
/// | dispatch lambdas |  |   58.149 µs | 58.264 µs | 58.391 µs |
/// | dispatch sort lambdas |  |   8.3311 µs | 8.3509 µs | 8.3746 µs |
/// | dispatch templ objs |  |   363.76 ns | 364.76 ns | 365.70 ns |
/// | dispatch templ box objs |  |   2.8970 µs | 2.9007 µs | 2.9044 µs |
/// | dispatch enum  | 3 |   2.8224 µs | 2.8299 µs | 2.8369 µs |
/// | dispatch enum  | 10 |   2.8176 µs | 2.8243 µs | 2.8322 µs |
/// | dispatch enum  | 50 |   2.8596 µs | 2.8663 µs | 2.8730 µs |
/// | dispatch random objs  | 1 |   8.3185 µs | 8.3309 µs | 8.3444 µs |
/// | dispatch random objs  | 2 |   39.025 µs | 39.100 µs | 39.177 µs |
/// | dispatch random objs  | 3 |   47.833 µs | 47.960 µs | 48.082 µs |
/// | dispatch random objs  | 4 |   50.158 µs | 50.302 µs | 50.464 µs |
/// | dispatch random objs  | 5 |   52.978 µs | 53.153 µs | 53.336 µs |
/// | dispatch random objs  | 6 |   54.857 µs | 54.977 µs | 55.110 µs |
/// | dispatch random objs  | 7 |   56.111 µs | 56.219 µs | 56.329 µs |
/// | dispatch random objs  | 8 |   56.784 µs | 56.938 µs | 57.101 µs |
/// | dispatch random objs  | 9 |   57.243 µs | 57.393 µs | 57.552 µs |
/// | dispatch random objs  | 10 |   58.528 µs | 58.643 µs | 58.772 µs |
/// | dispatch random objs  | 11 |   58.831 µs | 58.950 µs | 59.081 µs |
/// | dispatch random objs  | 12 |   58.552 µs | 58.658 µs | 58.774 µs |
/// | dispatch random objs  | 50 |   62.118 µs | 62.298 µs | 62.490 µs |
/// | dispatch sorted objs  | 50 |   9.1633 µs | 9.1738 µs | 9.1845 µs |
/// | dispatch even objs  | 1 |   21.854 µs | 21.878 µs | 21.904 µs |
/// | dispatch even objs  | 10 |   21.561 µs | 21.590 µs | 21.622 µs |
/// | dispatch even objs  | 20 |   21.616 µs | 21.645 µs | 21.676 µs |
/// | dispatch even objs  | 30 |   21.668 µs | 21.704 µs | 21.741 µs |
/// | dispatch even objs  | 40 |   26.408 µs | 27.712 µs | 29.121 µs |
/// | dispatch even objs  | 50 |   42.148 µs | 42.815 µs | 43.437 µs |
/// | dispatch even objs  | 60 |   39.908 µs | 40.091 µs | 40.273 µs |
/// | dispatch even objs  | 80 |   46.963 µs | 47.149 µs | 47.343 µs |
/// | dispatch even objs  | 100 |   55.568 µs | 55.721 µs | 55.894 µs |
/// | dispatch even objs  | 120 |   54.197 µs | 54.325 µs | 54.479 µs |
/// | dispatch even objs  | 140 |   54.188 µs | 54.367 µs | 54.564 µs |
/// | dispatch even objs  | 160 |   53.646 µs | 53.841 µs | 54.059 µs |
/// | dispatch even objs  | 180 |   55.096 µs | 55.195 µs | 55.303 µs |
/// | dispatch even objs  | 200 |   62.384 µs | 62.453 µs | 62.523 µs |
pub fn benchmark(c: &mut Criterion) {
  let array10: [i32; SIZE] = rust_bench::random_array(0..10, 0);
  let lambdas: [fn() -> i32; 10] = [|| 1, || 4, || 3, || 5, || 7, || 11, || 13, || 17, || 19, || 23];

  // Single functions
  c.bench_function("dispatch lambda", |b| b.iter(|| iter_func(black_box(&array10), black_box(|i| map_digit(i)))));
  c.bench_function("dispatch func", |b| b.iter(|| iter_func(black_box(&array10), black_box(map_digit))));
  c.bench_function("dispatch templ lambda", |b| b.iter(|| iter_func_template(black_box(&array10), black_box(map_digit))));

  // Multiple functions
  c.bench_function("dispatch lambdas", |b| b.iter(|| iter_lambdas(black_box(&array10), black_box(&lambdas))));
  let mut sorted10 = array10;
  sorted10.sort_unstable();
  c.bench_function("dispatch sort lambdas", |b| b.iter(|| iter_lambdas(black_box(&sorted10), black_box(&lambdas))));
  let obj10= array10.map(|x| GeneralProcessor::from(x));
  c.bench_function("dispatch templ objs", |b| b.iter(|| template_objs(&obj10)));
  let obj10= array10.map(|x| Box::new(GeneralProcessor::from(x)));
  c.bench_function("dispatch templ box objs", |b| b.iter(|| template_box_objs(&obj10)));

  // Generate SIZE values with a wider range.
  let big_array: [i32; SIZE] = rust_bench::random_array(0..100_000, 0);

  // Try different sized enums
  let enums: [Enum3; SIZE] = big_array.map(|x| num_traits::FromPrimitive::from_i32(x % 3)
      .expect("bad value {x}"));
  c.bench_function("dispatch enum 3",
                   |b| b.iter(|| template_objs(black_box(&enums))));
  let enums: [Enum10; SIZE] = big_array
      .map(|x| num_traits::FromPrimitive::from_i32(x % 10).expect("bad digit"));
  c.bench_function("dispatch enum 10", |b| b.iter(|| template_objs(black_box(&enums))));
  let enums: [Enum50; SIZE] = big_array.map(|x| num_traits::FromPrimitive::from_i32(x % 50)
      .expect("bad value {x}"));
  c.bench_function("dispatch enum 50",
                   |b| b.iter(|| template_objs(black_box(&enums))));

  // Try different numbers of classes with random distributions
  for number_of_classes in (1..=12).chain(50..=50) {
    // Create an array with the right number of classes.
    let random_objs = big_array
        .map(|x| processor_from_i32(x % number_of_classes));
    c.bench_function(format!("dispatch random objs {}", number_of_classes).as_str(),
                     |b| b.iter(|| iter_objs(black_box(&random_objs))));
  }

  // Generate a sorted array
  let sorted_objs: [Box<dyn Processor>; SIZE] = core::array::from_fn(
      |x| processor_from_i32((x / (big_array.len() / 50)) as i32));
  c.bench_function("dispatch sorted objs 50",
                   |b| b.iter(|| iter_objs(black_box(&sorted_objs))));

  // Try different multiples of 50 for round robin
  for multiple in [1, 10, 20, 30, 40, 50, 60, 80, 100, 120, 140, 160, 180, 200] {
    let mut order = (0..50*multiple).map(|x| x / multiple).collect::<Vec<i32>>();
    let mut rng: ChaChaRng = SeedableRng::seed_from_u64(0);
    order.shuffle(&mut rng);
    let random_objs: [Box<dyn Processor>; SIZE] = core::array::from_fn(
      |x| processor_from_i32(order[x % order.len()]));
    c.bench_function(format!("dispatch even objs {}", multiple).as_str(),
                       |b| b.iter(|| iter_objs(black_box(&random_objs))));
  }
}
