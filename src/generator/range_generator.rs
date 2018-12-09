extern crate rand;

use rand::prelude::*;
use super::*;

pub struct RangeGenerator {
    rng: Box<RngCore>
}

impl RangeGenerator {
    /// create a range generator instance
    pub fn new<R: RngCore + 'static>(rng: R) -> Self {
        RangeGenerator {
            rng: Box::new(rng)
        }
    }
}

impl Default for RangeGenerator {
    /// create a range generator instance with default rng
    fn default() -> Self {
        RangeGenerator::new(default_rng())
    }
}

impl Generator for RangeGenerator {

    fn run(&mut self, config: &Config) -> Vec<i32> {
        let mut result = Vec::new();
        let mut sample: Vec<i32> = (config.min..config.max+1).collect();
        for _ in 0..config.num {
            let size = sample.len();
            let rand_index = self.rng.gen_range(0, size);
            result.push(sample.remove(rand_index));
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::tests::*;

    #[test]
    fn test_non_repeated_values() {
        assert_non_repeated_values(RangeGenerator::default);
    }

    #[test]
    fn test_size() {
        assert_size(RangeGenerator::default);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(RangeGenerator::default);
    }

}