extern crate rand;

use rand::prelude::*;
use super::Config;
use super::Generator;

pub struct ChooseGenerator {
    config: Config,
    rng: ThreadRng
}

impl Generator for ChooseGenerator {

    fn run(&mut self) -> Vec<i32> {
        (self.config.min..self.config.max).choose_multiple(&mut self.rng, self.config.num as usize)
    }
}

impl ChooseGenerator {
    /// create an choose generator instance
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
        assert_non_repeated_values(ChooseGenerator::new);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(ChooseGenerator::new);
    }

}