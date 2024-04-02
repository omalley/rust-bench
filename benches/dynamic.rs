// Copyright by Owen O'Malley 2024

use std::collections::HashMap;
use criterion::{black_box, Criterion};

fn dynamic_match(data: &[i32]) -> Result<i32,String> {
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

fn dynamic_match_noerr(data: &[i32]) -> i32 {
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

fn dynamic_array(data: &[i32]) -> Result<i32,String> {
  let trans = [1, 2, 3, 5, 7, 11, 13, 17, 19, 23];
  data.iter().map(|v| {
    if (0..trans.len() as i32).contains(v) {
      Ok(trans[*v as usize])
    } else {
      Err(format!("Bad digit {v}"))
    }}).sum()
}

fn dynamic_array_noerr(data: &[i32]) -> i32 {
  let trans = [1, 2, 3, 5, 7, 11, 13, 17, 19, 23];
  data.iter().map(|v| trans[*v as usize]).sum()
}

fn dynamic_if(data: &[i32]) -> Result<i32,String> {
  data.iter().map(|v| {
    if *v == 0 {
      Ok(1)
    } else if *v == 1 {
      Ok(2)
    } else if *v == 2 {
      Ok(3)
    } else if *v == 3 {
      Ok(5)
    } else if *v == 4 {
      Ok(7)
    } else if *v == 5 {
      Ok(11)
    } else if *v == 6 {
      Ok(13)
    } else if *v == 7 {
      Ok(17)
    } else if *v == 8 {
      Ok(19)
    } else if *v == 9 {
      Ok(23)
    } else {
      Err(format!("Bad digit {v}"))
    }}).sum()
}

fn dynamic_funcs(data: &[i32], funcs: &[fn() -> i32]) -> Result<i32,String> {
  data.iter().map(|v| {
    if (0..funcs.len() as i32).contains(v) {
      Ok(funcs[*v as usize]())
    } else {
      Err(format!("Bad digit {v}"))
    }
  }).sum()
}

fn dynamic_map(data: &[i32], map: &HashMap<i32,i32>) -> Result<i32,String> {
  data.iter().map(|v| map.get(v).ok_or_else(|| format!("Bad digit {v}"))).sum()
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

fn dynamic_func_noerr(data: &[i32], func: fn(i32) -> i32) -> i32 {
  data.iter().map(|v| func(*v)).sum()
}

fn dynamic_template_noerr<F>(data: &[i32], func: F) -> i32
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

fn dynamic_objs(data: &[i32], objs: &[Box<dyn Processor>]) -> Result<i32,String> {
  data.iter().map(|v| {
    if (0..objs.len() as i32).contains(v) {
      Ok(objs[*v as usize].process())
    } else {
      Err(format!("Bad digit {v}"))
    }
  }).sum()
}

pub fn benchmark(c: &mut Criterion) {
  let array: [i32; 10_000] = rust_bench::random_array(0..10);
  let funcs: [fn() -> i32; 10] = [|| 1, || 2, || 3, || 5, || 7, || 11, || 13, || 17, || 19, || 23];
  let map = HashMap::from([(0,1), (1, 2), (2, 3), (3, 5), (4, 7), (5, 11),
    (6, 13), (7, 17), (8, 19), (9, 23)]);
  let objs: [Box<dyn Processor>; 10] = [Box::new(ProcessorImpl{x: 1}),
    Box::new(ProcessorImpl{x: 2}), Box::new(ProcessorImpl{x: 3}),
    Box::new(ProcessorImpl{x: 5}), Box::new(ProcessorImpl{x: 6}),
    Box::new(ProcessorImpl{x: 11}), Box::new(ProcessorImpl{x: 13}),
    Box::new(ProcessorImpl{x: 17}), Box::new(ProcessorImpl{x: 19}),
    Box::new(ProcessorImpl{x: 23})];
  c.bench_function("dynamic array", |b| b.iter(|| dynamic_array(black_box(&array))));
  c.bench_function("dynamic array no err", |b| b.iter(|| dynamic_array_noerr(black_box(&array))));
  c.bench_function("dynamic match", |b| b.iter(|| dynamic_match(black_box(&array))));
  c.bench_function("dynamic match no err", |b| b.iter(|| dynamic_match_noerr(black_box(&array))));
  c.bench_function("dynamic if", |b| b.iter(|| dynamic_if(black_box(&array))));
  c.bench_function("dynamic funcs", |b| b.iter(|| dynamic_funcs(black_box(&array), &funcs)));
  c.bench_function("dynamic map", |b| b.iter(|| dynamic_map(black_box(&array), black_box(&map))));
  c.bench_function("dynamic func no err", |b| b.iter(|| dynamic_func_noerr(black_box(&array), black_box(map_digit))));
  c.bench_function("dynamic template no err", |b| b.iter(|| dynamic_template_noerr(black_box(&array), black_box(map_digit))));
  c.bench_function("dynamic objs", |b| b.iter(|| dynamic_objs(black_box(&array), black_box(&objs))));
}
