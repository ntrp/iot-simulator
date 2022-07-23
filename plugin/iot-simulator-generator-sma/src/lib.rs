use abi_stable::std_types::RHashMap;
use std::collections::vec_deque::*;
use std::sync::{Arc, RwLock};

use rand::prelude::ThreadRng;
use rand::Rng;

use iot_simulator_api::export_plugin;
use iot_simulator_api::generator::{
    unwrap_arg, GenerationResult, GeneratorPlugin, GeneratorPointer,
};

unsafe extern "C" fn new_instance(args: RHashMap<String, String>) -> GeneratorPointer {
    let min = unwrap_arg("min", &args);
    let max = unwrap_arg("max", &args);
    let precision = unwrap_arg("precision", &args);
    let buffer_size = unwrap_arg("buffer_size", &args);
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

unsafe impl Sync for SMAGenerator {}
unsafe impl Send for SMAGenerator {}

impl SMAGenerator {
    fn new(min: f32, max: f32, precision: u32, buffer_size: usize) -> Arc<RwLock<SMAGenerator>> {
        Arc::new(RwLock::new(SMAGenerator {
            min,
            max,
            precision,
            buffer_size: buffer_size as u8,
            buffer: VecDeque::with_capacity(buffer_size),
            rng: rand::thread_rng(),
        }))
    }
}

impl GeneratorPlugin for SMAGenerator {
    fn generate(&mut self) -> GenerationResult {
        let val: f32 = self.rng.gen_range(self.min..self.max);
        if self.buffer.is_empty() {
            for _ in 1..=self.buffer_size {
                self.buffer.push_front(val)
            }
        }
        self.buffer.pop_back();
        self.buffer.push_front(val);
        GenerationResult::Float(round(avg(&mut self.buffer), self.precision))
    }
}

fn avg(buffer: &mut VecDeque<f32>) -> f32 {
    buffer.iter().sum::<f32>() / buffer.len() as f32
}

fn round(n: f32, precision: u32) -> f32 {
    let p = 10i32.pow(precision) as f32;
    (n * p).round() / p
}

export_plugin!("sma", new_instance);

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;
