![logo](doc/logo.png)

# Coilover

Coilover is a simple, Markdown, static site generator perfect for personal projects. Think of it like "I just want to write markdown and have the rest done for me" type of system. Coilover takes your Markdown and generates a web-page containing the corresponding HTML code. The website is then ready to be served by Apache, Nginx or another web server. Coilover is a personal tool developed to learn the rust programming language.

## Getting Started

- Install [Rust](https://www.rust-lang.org/)
- Read up about its [Usage](https://www.rust-lang.org/learn) and [Configuration](https://www.rust-lang.org/tools)
- [Fork](https://github.com/Ryhazerus/Coilover/fork) and Contribute your own modifications
- Have questions? Check out our official forum community Coilover

### Prerequisites

To get started with building Coilover you need the following installed.


- [Rust-lang Nightly](https://doc.rust-lang.org/1.2.0/book/nightly-rust.html)
- [Cargo](https://doc.rust-lang.org/cargo/)


### Installing

Installing and running the development environment.

```
cargo build
cargo run -- {OPTION}
```

Example for running the build.

```
 cargo run -- --help
```

## Deployment

To get a release or debug build you can run the following command.

```
Debug
- cargo build

Release
- cargo build --release
```

## Built With

* [Rust](https://www.rust-lang.org/)- The programming language used
* [Cargo](https://doc.rust-lang.org/cargo/) Rust build tool
* [Clap](https://github.com/clap-rs/clap) A full featured, fast Command Line Argument Parser for Rust 
* [Markdown](https://github.com/johannhof/markdown.rs) Rust Markdown parsing library
* [Yaml-rust](https://github.com/chyh1990/yaml-rust) A pure rust YAML implementation.

## Contributing

Please read [CONTRIBUTING.md](https://github.com/Ryhazerus/Coilover/blob/master/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/Ryhazerus/Coilover/releases). 

## Authors

* **Zahyr Seferina** - *Initial work* - [Ryhazerus](https://github.com/Ryhazerus)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments
* Rust docs
* Stackoverflow
