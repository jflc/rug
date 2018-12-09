extern crate rand;

use rand::prelude::*;
use super::*;
use std::prelude::v1::Vec;

pub struct ShuffleGenerator {
    rng: Box<RngCore>
}

impl ShuffleGenerator {
    /// create a shuffle generator instance
    pub fn new<R: RngCore + 'static>(rng: R) -> Self {
        ShuffleGenerator {
            rng: Box::new(rng)
        }
    }
}

impl Default for ShuffleGenerator {
    /// create a shuffle generator instance with default rng
    fn default() -> Self {
        ShuffleGenerator::new(default_rng())
    }
}

impl Generator for ShuffleGenerator {

    fn run(&mut self, config: &Config) -> Vec<i32> {
        (config.min..config.max+1).collect::<Vec<i32>>()
            .partial_shuffle(&mut self.rng, config.num)
            .0
            .to_vec()
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::tests::*;

    #[test]
    fn test_non_repeated_values() {
        assert_non_repeated_values(ShuffleGenerator::default);
    }

    #[test]
    fn test_size() {
        assert_size(ShuffleGenerator::default);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(ShuffleGenerator::default);
    }

}