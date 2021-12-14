use rand::Rng;
use std::time::Instant;
use sysinfo::SystemExt;

const CASE1BIAS: f64 = 1000.0;

pub struct Bench {}

impl Bench {
  // cpu
  pub fn case1() -> i64 {
    // pow 10 ^ 6
    let n = 1000000;

    // Fill the array
    let mut array = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
      array.push(rng.gen::<i32>());
    }

    // Sort
    let now = Instant::now();
    array.sort();
    let elapsed = now.elapsed();
    ((CASE1BIAS / ((elapsed.subsec_nanos() / 1_000_000) as f64)) * 1000.0) as i64
  }
  // memory
  pub fn case2() -> i64 {
    let system = sysinfo::System::new_all();
    let total_memory = system.get_total_memory();
    (total_memory / 100000) as i64
  }
  // network
  pub fn case3() -> i64 {
    todo!("TODO: network non checked");
  }
}
