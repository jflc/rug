extern crate rand;

use rand::prelude::*;
use rand::rngs::JitterRng;
use super::Config;
use super::Generator;

pub struct AuxGenerator {
    config: Config,
    rng: ThreadRng
}

impl Generator for AuxGenerator {

    fn run(&mut self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(self.config.num as usize);
        let range_size = (self.config.max - self.config.min + 1) as usize;
        let aux: &mut[Option<usize>] = &mut vec![None; range_size];

        for i in 0..self.config.num {
            let size = range_size - i as usize;
            let mut num = self.generate_next(&mut aux[..size]) as i32;
            num += self.config.min;
            result.push(num);
        }

        result
    }
}

impl AuxGenerator {

    /// create an aux generator instance
    pub fn new(config: Config) -> AuxGenerator {
        AuxGenerator {
            config,
            rng: thread_rng()
        }
    }

    /// generate next random number
    fn generate_next(&mut self, aux: &mut [Option<usize>]) -> usize  {
        let size= aux.len();
        let last = size - 1;
        let index= self.rng.gen_range(0, size);
        let result = aux[index].unwrap_or(index);
        aux[index] = aux[last].or(Some(last));
        result
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::tests::*;

    #[test]
    fn test_non_repeated_values() {
        assert_non_repeated_values(AuxGenerator::new);
    }

    #[test]
    fn test_values_probability() {
        assert_values_probability(AuxGenerator::new);
    }

}