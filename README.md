# Wastegate

Wastegate is a Rust based cli application for generating static websites. With Wastegate you can write your posts in markdown and run the wastegate application. The program supports both statically generated files and an internal web server to serve the files directly from the application.

## Prerequisites

To run the application you need Rust, Cargo and the Rust Nightly toolchain installed.


Install Rust Nightly directly
- [Rust Nightly](https://doc.rust-lang.org/1.2.0/book/nightly-rust.html)
- [Cargo](https://github.com/rust-lang/cargo)


## Installation

Use the build tool [cargo](https://github.com/rust-lang/cargo) to install, build and run the application.

```bash
- cargo build
- cargo run 
```

## Usage

```
Wastegate 1.0
Zahyr S. <zahyr.seferina@gmail.com>
Rust based static site generator, with no extra fat.

USAGE:
    wastegate [FLAGS] [OPTIONS]

FLAGS:
    -g, --generate    will generate a new static site
    -h, --help        Prints help information
    -i, --init        will initialize the wastegate application
    -V, --version     Prints version information

OPTIONS:
```

### Posts
To generate a new static site, put your markdown files in posts folder and run the application with ```./generate --generate``` or ```./generate -g```.

## Contributing
For contributions and change requests, please open an issue first to discuss what you would like to change.

And furthermore please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)