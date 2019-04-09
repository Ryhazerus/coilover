extern crate clap;
extern crate markdown;
extern crate yaml_rust;

mod config;
mod generate;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Wastegate")
        .version("1.0")
        .author("Zahyr S. <zahyr.seferina@gmail.com>")
        .about("Rust based static site generator, with no extra fat.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("generate")
                .short("g")
                .long("generate")
                .help("will generate a new static site"),
        )
        .arg(
            Arg::with_name("init")
                .short("i")
                .long("init")
                .help("will initialize the wategate application"),
        )
        .get_matches();

    if matches.is_present("init") {
        let configuration = config::Config::new();
        config::Config::generate_config_directories(configuration.get_directory_path());
        config::Config::generate_config_file(configuration.get_directory_path());
    }
}
