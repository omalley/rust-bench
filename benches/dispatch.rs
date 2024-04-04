// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};
use num_derive::FromPrimitive;
use paste::paste;

/// A utility function to translate the digits via a match
fn map_digit(v: i32) -> i32 {
  match v {
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

/// Use a passed in function to translate
fn iter_func(data: &[i32], func: fn(i32) -> i32) -> i32 {
  data.iter().map(|v| func(*v)).sum()
}

/// Use templates to remove the function dispatch costs.
fn iter_func_template<F>(data: &[i32], func: F) -> i32
  where F: Fn(i32) -> i32 {
  data.iter().map(|v| func(*v)).sum()
}

/// Iterate and dispatch via an array of lambdas.
fn iter_lambdas(data: &[i32], funcs: &[fn() -> i32]) -> i32 {
  data.iter().map(|v| funcs[*v as usize]()).sum()
}

trait Processor {
  fn process(&self) -> i32;
}

/// Define a class that implements the trait and can
/// handle all the inputs.
#[derive(Debug)]
struct GeneralProcessor {
  x: i32,
}

impl GeneralProcessor {
  fn from(i: i32) -> Self {
    GeneralProcessor {x: map_digit(i)}
  }
}

impl Processor for GeneralProcessor {
  fn process(&self) -> i32 {
    self.x
  }
}

macro_rules! define_structs {
    ( $({$name:literal, $value:literal}),* ) => {
      paste!{
        $(struct [<Processor $name>] {
            // nothing
          }

          impl Processor for [<Processor $name>] {
            fn process(&self) -> i32 {
              $value
            }
        })*

        fn processor_from_i32(i: i32) -> Box<dyn Processor> {
          match i {
            $($name => Box::new([<Processor $name>]{}) as Box<dyn Processor>,)*
            _ => panic!("Bad name {i}"),
          }
        }
      }
    }
}

/// Define Processor0 to Processor23 using a macro.
define_structs!({0, 1}, {1, 2}, {2, 3}, {3, 5},
  {4, 7}, {5, 11}, {6, 13}, {7, 17}, {8, 19},
  {9, 23}, {10, 25}, {11, 27}, {12, 29}, {13, 31},
  {14, 33}, {15, 35}, {16, 37}, {17, 39}, {18, 41},
  {19, 43}, {20, 45}, {21, 47}, {22, 49}, {23, 51});

/// Use virtual dispatch through the trait.
fn iter_objs(data: &[Box<dyn Processor>]) -> i32 {
  data.iter().map(|v| v.process()).sum()
}

#[derive(FromPrimitive)]
enum ProcessorEnum {
  Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
}

impl ProcessorEnum {
  fn process(&self) -> i32 {
    match self {
      ProcessorEnum::Zero => 1,
      ProcessorEnum::One => 2,
      ProcessorEnum::Two => 3,
      ProcessorEnum::Three => 5,
      ProcessorEnum::Four => 7,
      ProcessorEnum::Five => 11,
      ProcessorEnum::Six => 13,
      ProcessorEnum::Seven => 17,
      ProcessorEnum::Eight => 19,
      ProcessorEnum::Nine => 23,
    }
  }
}

/// Test the speed going through using an enum non-virtual method.
fn iter_enums(data: &[ProcessorEnum]) -> i32 {
  data.iter().map(|x| x.process()).sum()
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(0..10, 0);
  let lambdas: [fn() -> i32; 10] = [|| 1, || 2, || 3, || 5, || 7, || 11, || 13, || 17, || 19, || 23];
  let objs = array
      .map(|x| Box::new(GeneralProcessor::from(x)) as Box<dyn Processor>);
  let enums = array
      .map(|x| num_traits::FromPrimitive::from_i32(x).expect("bad digit"));
  c.bench_function("dispatch lambda", |b| b.iter(|| iter_func(black_box(&array), black_box(|i| map_digit(i)))));
  c.bench_function("dispatch func", |b| b.iter(|| iter_func(black_box(&array), black_box(map_digit))));
  c.bench_function("dispatch func template", |b| b.iter(|| iter_func_template(black_box(&array), black_box(map_digit))));
  c.bench_function("dispatch lambdas", |b| b.iter(|| iter_lambdas(black_box(&array), black_box(&lambdas))));
  c.bench_function("dispatch iter objs", |b| b.iter(|| iter_objs(black_box(&objs))));
  c.bench_function("dispatch iter enums", |b| b.iter(|| iter_enums(black_box(&enums))));
  // Generate 10,000 values with a wider range.
  let array: [i32; 10_000] = rust_bench::random_array(0..100_000, 0);
  for number_of_classes in 1..=24 {
    // Create an array with the right number of classes.
    let tmp_objs = array
        .map(|x| processor_from_i32(x % number_of_classes));
    c.bench_function(format!("dispatch varied {}", number_of_classes).as_str(),
                     |b| b.iter(|| iter_objs(black_box(&tmp_objs))));
  }
}
