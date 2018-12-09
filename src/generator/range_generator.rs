extern crate rand;

use rand::prelude::*;
use super::*;

pub struct RangeGenerator {
    rng: Box<RngCore>
}

impl RangeGenerator {
    /// create a range generator instance with default rng
    pub fn new() -> Self {
        RangeGenerator::new_with_rng(default_rng())
    }

    /// create a range generator instance
    pub fn new_with_rng<R: RngCore + 'static>(rng: R) -> Self {
        RangeGenerator {
            rng: Box::new(rng)
        }
    }
}

impl Generator for RangeGenerator {

    fn run(&mut self, config: &Config) -> Vec<i32> {
        let mut result = Vec::new();
        let mut sample: Vec<i32> = (config.min..config.max).collect();
        for _ in 0..config.num {
            let size = sample.len();
            let rand_index = self.rng.gen_range(0, size);
            result.push(sample.remove(rand_index));
        }
        return result;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::tests::*;

    #[test]
    fn test_non_repeated_values() {
        assert_non_repeated_values(RangeGenerator::new);
    }

    #[test]
    fn test_size() {
        assert_size(RangeGenerator::new);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(RangeGenerator::new);
    }

}