// Copyright by Owen O'Malley 2024

use criterion::{black_box, Criterion};
use num_derive::FromPrimitive;

fn iter_lambdas(data: &[i32], funcs: &[fn() -> i32]) -> i32 {
  data.iter().map(|v| funcs[*v as usize]()).sum()
}

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

fn iter_func(data: &[i32], func: fn(i32) -> i32) -> i32 {
  data.iter().map(|v| func(*v)).sum()
}

fn iter_func_template<F>(data: &[i32], func: F) -> i32
  where F: Fn(i32) -> i32 {
  data.iter().map(|v| func(*v)).sum()
}

trait Processor {
  fn process(&self) -> i32;
}

struct ProcessorImpl {
  x: i32,
}

impl Processor for ProcessorImpl {
  fn process(&self) -> i32 {
    self.x
  }
}

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

fn iter_enums(data: &[ProcessorEnum]) -> i32 {
  data.iter().map(|x| x.process()).sum()
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(0..10, 0);
  let lambdas: [fn() -> i32; 10] = [|| 1, || 2, || 3, || 5, || 7, || 11, || 13, || 17, || 19, || 23];
  let objs = array
      .map(|x| Box::new(ProcessorImpl{x}) as Box<dyn Processor>);
  let enums = array
      .map(|x| num_traits::FromPrimitive::from_i32(x).expect("bad digit"));
  c.bench_function("dispatch lambdas", |b| b.iter(|| iter_lambdas(black_box(&array), black_box(&lambdas))));
  c.bench_function("dispatch func", |b| b.iter(|| iter_func(black_box(&array), black_box(map_digit))));
  c.bench_function("dispatch func template", |b| b.iter(|| iter_func_template(black_box(&array), black_box(map_digit))));
  c.bench_function("dispatch iter objs", |b| b.iter(|| iter_objs(black_box(&objs))));
  c.bench_function("dispatch iter enums", |b| b.iter(|| iter_enums(black_box(&enums))));
}
