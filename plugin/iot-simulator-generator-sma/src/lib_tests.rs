use std::collections::VecDeque;

use crate::avg;

#[cfg(test)]
mod tests {
    use crate::lib_tests::std_deviation;
    use std::collections::HashMap;

    use super::super::*;

    #[test]
    fn it_should_generate_in_the_range() {
        let plugin = unsafe {
            new_instance(
                HashMap::from([
                    ("min".to_string(), "10.0".to_string()),
                    ("max".to_string(), "20.0".to_string()),
                    ("precision".to_string(), "2".to_string()),
                    ("buffer_size".to_string(), "10".to_string()),
                ])
                .into(),
            )
        };

        for _ in 1..150 {
            let val = match plugin.write().expect("Write lock failed").generate() {
                GenerationResult::Float(val) => val,
                _ => unreachable!("This plugin generates f32"),
            };
            assert!(val > 10.0 && val < 20.0);
        }
    }

    #[test]
    fn it_should_have_lower_std_when_the_bucket_is_bigger() {
        let plugin_small = unsafe {
            new_instance(
                HashMap::from([
                    ("min".to_string(), "10.0".to_string()),
                    ("max".to_string(), "20.0".to_string()),
                    ("precision".to_string(), "2".to_string()),
                    ("buffer_size".to_string(), "5".to_string()),
                ])
                .into(),
            )
        };
        let plugin_large = unsafe {
            new_instance(
                HashMap::from([
                    ("min".to_string(), "10.0".to_string()),
                    ("max".to_string(), "20.0".to_string()),
                    ("precision".to_string(), "2".to_string()),
                    ("buffer_size".to_string(), "20".to_string()),
                ])
                .into(),
            )
        };

        let mut small_vals: VecDeque<f32> = (1..150)
            .map(
                |_| match plugin_small.write().expect("Write lock failed").generate() {
                    GenerationResult::Float(res) => res,
                    _ => unreachable!("This plugin generates f32"),
                },
            )
            .collect();

        let mut large_vals: VecDeque<f32> = (1..150)
            .map(
                |_| match plugin_large.write().expect("Write lock failed").generate() {
                    GenerationResult::Float(res) => res,
                    _ => unreachable!("This plugin generates f32"),
                },
            )
            .collect();

        assert!(std_deviation(&mut small_vals) > std_deviation(&mut large_vals));
    }
}

fn std_deviation(data: &mut VecDeque<f32>) -> f32 {
    let avg = avg(data);
    let count = data.len();
    let variance = data
        .iter()
        .map(|value| {
            let diff = avg - (*value as f32);
            diff * diff
        })
        .sum::<f32>()
        / count as f32;
    variance.sqrt()
}
