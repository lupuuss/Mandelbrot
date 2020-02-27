extern crate num_cpus;
extern crate palette;
extern crate serde;
extern crate clap;
extern crate sdl2;

pub mod fractal;
pub mod utils;
pub mod user;

use fractal::config::Config;
use user::Mode;

use clap::{Arg, App};

fn main() {

    let matches = App::new("Mandelbrot")
                    .version("1.0")
                    .author("github.com/lupuuss")
                    .about("Generates mandelbrot and julia sets!")
                    .arg(Arg::with_name("real")
                            .takes_value(true)
                            .short("r")
                            .long("real")
                            .validator(utils::numeric_validator)
                            .required(false))
                    .arg(Arg::with_name("imag")
                            .short("i")
                            .takes_value(true)
                            .long("imag")
                            .validator(utils::numeric_validator)
                            .required(false))
                    .arg(Arg::with_name("cli")
                             .short("c")
                             .takes_value(false)
                             .required(false))
                    .get_matches();

    let config = Config::read_form_file_or_default("config.json");
    let is_cli = matches.is_present("cli");

    let julia_c = utils::parse_julia_c(&matches);

    let mut mode_runner = Mode::new_runner(if is_cli { Mode::CliStatic(julia_c) } else { Mode::GuiDynamic });

    mode_runner.start(&config);

}
