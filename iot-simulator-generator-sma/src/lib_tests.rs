use crate::avg;
use std::collections::VecDeque;

#[cfg(test)]
mod lib_tests {
    use super::super::*;
    use crate::lib_tests::std_deviation;

    #[test]
    fn it_should_generate_in_the_range() {
        let mut plugin = new_instance(10.0, 20.0, 2, 20);

        for _ in 1..150 {
            let val = plugin.generate(());
            assert!(val > 10.0 && val < 20.0);
        }
    }

    #[test]
    fn it_should_have_lower_std_when_the_bucket_is_bigger() {
        let mut plugin_small = new_instance(10.0, 20.0, 2, 5);
        let mut plugin_large = new_instance(10.0, 20.0, 2, 20);

        let mut small_vals: VecDeque<f32> = (1..150).map(|_| plugin_small.generate(())).collect();
        let mut large_vals: VecDeque<f32> = (1..150).map(|_| plugin_large.generate(())).collect();

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
