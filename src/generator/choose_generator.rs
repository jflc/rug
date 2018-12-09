extern crate rand;

use rand::prelude::*;
use super::*;

pub struct ChooseGenerator {
    rng: Box<RngCore>
}

impl ChooseGenerator {
    /// create an choose generator instance with default rng
    pub fn new() -> Self {
        ChooseGenerator::new_with_rng(default_rng())
    }

    /// create an choose generator instance
    pub fn new_with_rng<R: RngCore + 'static>(rng: R) -> Self {
        ChooseGenerator {
            rng: Box::new(rng)
        }
    }
}

impl Generator for ChooseGenerator {

    fn run(&mut self, config: &Config) -> Vec<i32> {
        (config.min..config.max).choose_multiple(&mut self.rng, config.num)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::tests::*;

    #[test]
    fn test_non_repeated_values() {
        assert_non_repeated_values(ChooseGenerator::new);
    }

    #[test]
    fn test_size() {
        assert_size(ChooseGenerator::new);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(ChooseGenerator::new);
    }

}