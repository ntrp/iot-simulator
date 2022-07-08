use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::vec_deque::*;

use iot_simulator_api::generator::*;

#[no_mangle]
pub fn new_instance(
    min: f32,
    max: f32,
    precision: u8,
    buffer_size: usize,
) -> Box<dyn StatefulGeneratorPlugin<(), f32>> {
    SMAGenerator::new(min, max, precision, buffer_size)
}

pub struct SMAGenerator {
    min: f32,
    max: f32,
    precision: u8,
    buffer: VecDeque<i32>,
    rng: ThreadRng,
}

impl SMAGenerator {
    fn new(min: f32, max: f32, precision: u8, buffer_size: usize) -> Box<SMAGenerator> {
        Box::new(SMAGenerator {
            min,
            max,
            precision,
            buffer: VecDeque::with_capacity(buffer_size),
            rng: rand::thread_rng(),
        })
    }
}

impl GeneratorPlugin<(), f32> for SMAGenerator {
    fn generate(&mut self, _: ()) -> f32 {
        let val: f32 = self.rng.gen_range(self.min..self.max);
        val
    }
}

impl StatefulGeneratorPlugin<(), f32> for SMAGenerator {}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;
