use rand::prelude::*;

pub mod aux_generator;
pub mod range_generator;
pub mod choose_generator;
pub mod shuffle_generator;
pub mod swap_generator;

pub struct Config {
    min: i32,
    max: i32,
    num: usize
}

impl Config {

    pub fn new(min: i32, max: i32, num: usize) -> Result<Self, String>{
        Config {
            min,
            max,
            num
        }.validate()
    }

    /// validate Config values
    fn validate(self) -> Result<Self, String> {
        match (self.min, self.max, self.num) {
            (min, max, _) if max < min => Err(String::from("max value needs to be bigger than min value")),
            (min, max, num) if ((max - min + 1) as usize) < num => Err(String::from("num cannot be bigger than the min..max range")),
            _ => Ok(self)
        }
    }
}

pub trait Generator {

    /// generate random number sequence
    fn run(&mut self, config: &Config) -> Vec<i32>;

}

fn default_rng() -> impl RngCore {
    thread_rng()
}


#[cfg(test)]
pub mod tests {

    use super::*;

    use std::collections::HashSet;
    use std::collections::HashMap;
    use std::time::Duration;
    use std::time::Instant;


    trait TestGenerator {
        fn run_and_time(&mut self, config: &Config) -> (Vec<i32>, Duration);
    }

    impl<G: Generator> TestGenerator for G {
        fn run_and_time(&mut self, config: &Config) -> (Vec<i32>, Duration) {
            let start = Instant::now();
            let result = self.run(config);
            (result, start.elapsed())
        }
    }

    #[test]
    fn test_validate_min_bigger_than_max() {
        let result = Config::new(
            20,
            10,
            1
        );

        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap(), String::from("max value needs to be bigger than min value"));
    }

    #[test]
    fn test_validate_num_bigger_than_max_min_range() {
        let result = Config::new(
            10,
            20,
            30
        );

        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap(), String::from("num cannot be bigger than the min..max range"));
    }

    #[test]
    fn test_validate_valid_config() {
        let result = Config::new(
            10,
            20,
            10
        );

        assert_eq!(result.is_ok(), true);
        let config = result.unwrap();
        assert_eq!(config.min, 10);
        assert_eq!(config.max, 20);
        assert_eq!(config.num, 10);
    }

    #[test]
    fn test_validate_all_equal() {
        let result = Config::new(
            1,
            1,
            1
        );

        assert_eq!(result.is_ok(), true);
        let config = result.unwrap();
        assert_eq!(config.min, 1);
        assert_eq!(config.max, 1);
        assert_eq!(config.num, 1);
    }

    pub fn assert_non_repeated_values<F, R>(constructor: F) where F: Fn() -> R, R: Generator {
        let config = Config::new(
            1000,
            2000,
            500
        ).expect("Unable to create config");

        let mut generator = constructor();

        let result = generator.run(&config);

        let mut set = HashSet::new();

        assert!(result.iter().all(|e| set.insert(e)));
    }

    pub fn assert_size<F, R>(constructor: F) where F: Fn() -> R, R: Generator {
        let config = Config::new(
            10,
            100,
            25
        ).expect("Unable to create config");

        let mut generator = constructor();

        let result = generator.run(&config);

        assert_eq!(result.len(), config.num);
    }

    pub fn assert_values_probability<F, R>(constructor: F) where F: Fn() -> R, R: Generator {
        let iterations: u32 = 100000;
        let config = Config::new(
            1,
            100,
            25
        ).expect("Unable to create config");

        let mut generator = constructor();

        let mut values: HashMap<i32, u32> = (config.min..config.max).map(|i| (i,0)).collect();
        let mut times: Vec<Duration> = Vec::new();

        (0..iterations).map(|_| generator.run_and_time(&config))
            .flat_map(|(r, t)| {times.push(t); r})
            .for_each(|v| {values.entry(v).and_modify(|e|  *e += 1 );});

        let min = *values.values().min().unwrap() as f64 / (iterations as f64 / 100.0);
        let max = *values.values().max().unwrap() as f64 / (iterations as f64 / 100.0);
        let diff = max - min;

        let fastest = times.iter().min().unwrap();
        let slowest = times.iter().max().unwrap();
        let average: Duration = times.iter().sum::<Duration>() / iterations;

        println!();
        println!("min: {:?}%; max: {:?}%; diff: {:?}%", min, max, diff);
        println!("fastest: {:?}; slowest: {:?}; average: {:?};", fastest, slowest, average);
        println!();

        assert!(diff <= 1.1);
    }

}