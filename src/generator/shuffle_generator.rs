extern crate rand;

use rand::prelude::*;
use rand::seq::SliceRandom;
use super::Config;
use super::Generator;

pub struct ShuffleGenerator {
    config: Config,
    rng: ThreadRng
}

impl Generator for ShuffleGenerator {

    fn run(&mut self) -> Vec<i32> {
        let mut sample: Vec<i32> = (self.config.min..self.config.max).collect();
        let (result, _) = sample.partial_shuffle(&mut self.rng, self.config.num as usize);
        result.to_vec()
    }

}

impl ShuffleGenerator {
    /// create an shuffle generator instance
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
        assert_non_repeated_values(ShuffleGenerator::new);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(ShuffleGenerator::new);
    }

}