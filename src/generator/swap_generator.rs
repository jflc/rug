extern crate rand;

use rand::prelude::*;
use super::*;

pub struct SwapGenerator {
    rng: Box<RngCore>
}

impl SwapGenerator {
    /// create a swap generator instance with default rng
    pub fn new() -> Self {
        SwapGenerator::new_with_rng(default_rng())
    }

    /// create a swap generator instance
    pub fn new_with_rng<R: RngCore + 'static>(rng: R) -> Self {
        SwapGenerator {
            rng: Box::new(rng)
        }
    }
}

impl Generator for SwapGenerator {

    fn run(&mut self, config: &Config) -> Vec<i32> {
        let mut result: Vec<i32> = (config.min..config.max).collect();
        let size = result.len();
        for i in 0..config.num {
            let rand_index = self.rng.gen_range(i, size);
            result.swap(i, rand_index);
        }
        return result[..config.num].to_vec();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::tests::*;

    #[test]
    fn test_non_repeated_values() {
        assert_non_repeated_values(SwapGenerator::new);
    }

    #[test]
    fn test_size() {
        assert_size(SwapGenerator::new);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(SwapGenerator::new);
    }

}