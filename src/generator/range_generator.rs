extern crate rand;

use rand::prelude::*;
use super::Config;
use super::Generator;

pub struct RangeGenerator {
    config: Config,
    rng: ThreadRng
}

impl Generator for RangeGenerator {

    fn run(&mut self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut sample: Vec<i32> = (self.config.min..self.config.max).collect();
        for _ in 0..self.config.num {
            let size = sample.len();
            let rand_index = self.rng.gen_range(0, size);
            result.push(sample.remove(rand_index));
        }
        return result;
    }
}

impl RangeGenerator {
    /// create an swap generator instance
    pub fn new(config: Config) -> Self {
        Self {
            config,
            rng: thread_rng()
        }
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
    fn test_values_probability() {
        assert_values_probability(RangeGenerator::new);
    }

}