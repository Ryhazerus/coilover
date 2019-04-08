extern crate clap;
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
        .get_matches();

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    if matches.is_present("generate") {
        let test = generate::Generate::include_metadata_in_template(
            &String::from("./src/mock/config/wastegate.yml"),
            &String::from("./src/templates/index.htm"),
        );
    }
}
