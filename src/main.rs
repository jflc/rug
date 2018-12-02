extern crate rug;
#[macro_use]
extern crate clap;

use clap::App;
use rug::generator::Config;
use rug::generator::Generator;
use rug::generator::aux_generator::AuxGenerator;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = Config::new(
        value_t_or_exit!(matches.value_of("min"), i32),
        value_t_or_exit!(matches.value_of("max"), i32),
        value_t_or_exit!(matches.value_of("num"), u32)
    ).expect("Invalid configuration");
    let mut generator = AuxGenerator::new(config);
    let result = generator.run();
    println!("Result: {:?}", result);
}
