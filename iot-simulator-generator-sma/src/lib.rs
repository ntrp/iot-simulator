use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::vec_deque::*;

use iot_simulator_api::generator::*;

#[no_mangle]
pub fn new_instance(
    min: f32,
    max: f32,
    precision: u32,
    buffer_size: usize,
) -> Box<dyn StatefulGeneratorPlugin<(), f32>> {
    SMAGenerator::new(min, max, precision, buffer_size)
}

pub struct SMAGenerator {
    min: f32,
    max: f32,
    precision: u32,
    buffer_size: u8,
    buffer: VecDeque<f32>,
    rng: ThreadRng,
}

impl SMAGenerator {
    fn new(min: f32, max: f32, precision: u32, buffer_size: usize) -> Box<SMAGenerator> {
        Box::new(SMAGenerator {
            min,
            max,
            precision,
            buffer_size: buffer_size as u8,
            buffer: VecDeque::with_capacity(buffer_size),
            rng: rand::thread_rng(),
        })
    }
}

impl GeneratorPlugin<(), f32> for SMAGenerator {
    fn generate(&mut self, _: ()) -> f32 {
        let val: f32 = self.rng.gen_range(self.min..self.max);
        if self.buffer.is_empty() {
            for _ in 1..=self.buffer_size {
                self.buffer.push_front(val)
            }
        }
        self.buffer.pop_back();
        self.buffer.push_front(val);
        round(avg(&mut self.buffer), self.precision)
    }
}

fn avg(buffer: &mut VecDeque<f32>) -> f32 {
    buffer.iter().sum::<f32>() / buffer.len() as f32
}

fn round(n: f32, precision: u32) -> f32 {
    let p = 10i32.pow(precision) as f32;
    (n * p).round() / p
}

impl StatefulGeneratorPlugin<(), f32> for SMAGenerator {}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;
