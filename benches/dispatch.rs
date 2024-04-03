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

#[derive(Debug)]
struct ProcessorImpl {
  x: i32,
}

impl ProcessorImpl {
  fn from(i: i32) -> Self {
    ProcessorImpl{x: map_digit(i)}
  }
}
impl Processor for ProcessorImpl {
  fn process(&self) -> i32 {
    self.x
  }
}

macro_rules! define_struct {
    ( $name:ident, $value:literal) => {
        #[derive(Default)]
        struct $name {
          // nothing
        }

        impl Processor for $name {
          fn process(&self) -> i32 {
            $value
          }
        }
    }
}

define_struct!(Processor0, 1);
define_struct!(Processor1, 2);
define_struct!(Processor2, 3);
define_struct!(Processor3, 5);
define_struct!(Processor4, 7);
define_struct!(Processor5, 11);
define_struct!(Processor6, 13);
define_struct!(Processor7, 17);
define_struct!(Processor8, 19);
define_struct!(Processor9, 23);

fn processor_from_i32(i: i32) -> Box<dyn Processor> {
  match i {
    0 => Box::new(Processor0::default()) as Box<dyn Processor>,
    1 => Box::new(Processor1::default()) as Box<dyn Processor>,
    2 => Box::new(Processor2::default()) as Box<dyn Processor>,
    3 => Box::new(Processor3::default()) as Box<dyn Processor>,
    4 => Box::new(Processor4::default()) as Box<dyn Processor>,
    5 => Box::new(Processor5::default()) as Box<dyn Processor>,
    6 => Box::new(Processor6::default()) as Box<dyn Processor>,
    7 => Box::new(Processor7::default()) as Box<dyn Processor>,
    8 => Box::new(Processor8::default()) as Box<dyn Processor>,
    9 => Box::new(Processor9::default()) as Box<dyn Processor>,
    _ => panic!("Bad digit {i}"),
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
      .map(|x| Box::new(ProcessorImpl::from(x)) as Box<dyn Processor>);
  let varied_objs = array.map(|x| processor_from_i32(x));
  let enums = array
      .map(|x| num_traits::FromPrimitive::from_i32(x).expect("bad digit"));
  c.bench_function("dispatch lambdas", |b| b.iter(|| iter_lambdas(black_box(&array), black_box(&lambdas))));
  c.bench_function("dispatch lambda", |b| b.iter(|| iter_func(black_box(&array), black_box(|i| map_digit(i)))));
  c.bench_function("dispatch func", |b| b.iter(|| iter_func(black_box(&array), black_box(map_digit))));
  c.bench_function("dispatch func template", |b| b.iter(|| iter_func_template(black_box(&array), black_box(map_digit))));
  c.bench_function("dispatch iter objs", |b| b.iter(|| iter_objs(black_box(&objs))));
  c.bench_function("dispatch iter varied objs", |b| b.iter(|| iter_objs(black_box(&varied_objs))));
  c.bench_function("dispatch iter enums", |b| b.iter(|| iter_enums(black_box(&enums))));
  for limit in 0..10 {
    let tmp_objs = array
        .map(|x| if x < limit { processor_from_i32(x)}
        else { Box::new(ProcessorImpl{x}) as Box<dyn Processor>});
    c.bench_function(format!("dispatch varied {}", limit + 1).as_str(),
                     |b| b.iter(|| iter_objs(black_box(&tmp_objs))));
  }
}
