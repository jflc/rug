extern crate rug;
#[macro_use]
extern crate clap;

use clap::App;
use rug::generator::Config;
use rug::generator::Generator;
use rug::generator::aux_generator::AuxGenerator;
use rug::generator::choose_generator::ChooseGenerator;
use rug::generator::range_generator::RangeGenerator;
use rug::generator::shuffle_generator::ShuffleGenerator;
use rug::generator::swap_generator::SwapGenerator;

arg_enum!{
    #[derive(PartialEq, Debug)]
    pub enum AlgType {
        Aux,
        Choose,
        Range,
        Shuffle,
        Swap,
    }
}

impl AlgType {
    fn create_generator(&self) -> Box<Generator> {
        println!("Algorithm: {:?}", self);
        match *self {
            AlgType::Aux => Box::new(AuxGenerator::new()),
            AlgType::Choose => Box::new(ChooseGenerator::new()),
            AlgType::Range => Box::new(RangeGenerator::new()),
            AlgType::Shuffle => Box::new(ShuffleGenerator::new()),
            AlgType::Swap => Box::new(SwapGenerator::new())
        }
    }
}



fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = Config::new(
        value_t_or_exit!(matches.value_of("min"), i32),
        value_t_or_exit!(matches.value_of("max"), i32),
        value_t_or_exit!(matches.value_of("num"), usize)
    ).expect("Invalid configuration");
    let alg_type = value_t_or_exit!(matches.value_of("alg"), AlgType);
    let sort =  matches.is_present("sort");

    let mut generator = alg_type.create_generator();

    let mut result = generator.run(&config);

    if sort {
        result.sort();
    }

    println!("Result: {:?}", result);
}
