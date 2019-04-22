//! Coilover is a static website generator built purely in Rust.
//! The app uses the clap crate to easily build a cli application
//! and parse arguments. 
//! 
//! The main file consists of the cli initialization process
//! and the actions for the currently supported features.
//! 
//! Author: Zahyr Seferina
//! Version: 1.0.0

extern crate clap;
extern crate dirs;
extern crate markdown;
extern crate yaml_rust;

mod config;
mod generate;

use clap::{App, Arg}; 

fn main() {
    let matches = App::new("Coilover")
        .version("1.0.0")
        .author("Zahyr S. <zahyr.seferina@gmail.com>")
        .about("Rust based static site generator")
        .arg(
            Arg::with_name("init")
                .short("i")
                .long("init")
                .help("will initialize the coilover application"),
        )
        .arg(
            Arg::with_name("generate")
                .short("g")
                .long("generate")
                .help("will generate a new static site"),
        )
        .get_matches();

    // Auto-generate the configuration files
    let configuration : config::Config = config::Config::new();
    if matches.is_present("init") {
        &configuration.generate_config();
    }

    // Generate command will generate the posts based on the provided markdown
    // posts in the designated folder.
    if matches.is_present("generate") {
        generate::Generate::new()
        .build(&configuration.get_full_config_file());;
    }
}