extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Wastegate")
                        .version("1.0")
                        .author("Zahyr S. <zahyr.seferina@gmail.com>")
                        .about("Rust based static site generator, with no extra fat.")
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("FILE")
                                    .help("Sets a custom config file")
                                    .takes_value(true))
                        .get_matches();

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }


    // Continued program logic goes here...
}