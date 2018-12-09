extern crate rand;

use rand::prelude::*;
use super::*;

pub struct AuxGenerator {
    rng: Box<RngCore>
}


impl AuxGenerator {

    /// create an aux generator instance
    pub fn new<R: RngCore + 'static>(rng: R) -> AuxGenerator {
        AuxGenerator {
            rng: Box::new(rng)
        }
    }

    /// generate next random number
    fn generate_next(&mut self, aux: &mut [Option<usize>]) -> usize  {
        let size= aux.len();
        let last = size - 1;
        let index= self.rng.gen_range(0, size);
        let result = aux[index].unwrap_or(index);
        aux[index] = aux[last].or_else(|| Some(last));
        result
    }

}

impl Default for AuxGenerator {
    /// create an aux generator instance with default rng
    fn default() -> Self {
        AuxGenerator::new(default_rng())
    }
}

impl Generator for AuxGenerator {

    fn run(&mut self, config: &Config) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(config.num);
        let range_size = (config.max - config.min + 1) as usize;
        let aux: &mut[Option<usize>] = &mut vec![None; range_size];

        for i in 0..config.num {
            let size = range_size - i;
            let mut num = self.generate_next(&mut aux[..size]) as i32;
            num += config.min;
            result.push(num);
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
        assert_non_repeated_values(AuxGenerator::default);
    }

    #[test]
    fn test_size() {
        assert_size(AuxGenerator::default);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(AuxGenerator::default);
    }

}