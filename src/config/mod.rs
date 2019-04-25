//! Config module
//!
//! The config module is meant to hold all
//! the configuration data and functionality
//! to generate the necessary config files.

use std::env;
use std::fs;
use std::path::Path;
use yaml_rust::{YamlEmitter, YamlLoader};

#[derive(Debug, Clone)]
pub struct Config {
    pub directory_path: String,
    config_file_path: String,
    posts_path: String,
}

/// Config implementation is meant to hold
/// all of the configuration functionality
/// and object.
impl Config {

    /// new function will get the current directory
    /// and generate the correct paths based on the
    /// gotten directory path.
    pub fn new() -> Config {
        let root_path = env::current_dir().expect("Cannot get current directory");

        let home_path: String = root_path.display().to_string();
        let config_path: String = home_path.to_owned() + "/coilover_config.yml";
        let post_config: String = home_path.to_owned() + "/posts";

        Config {
            directory_path: home_path,
            config_file_path: config_path,
            posts_path: post_config,
        }
    }

    /// generate_config() will check if the folders already
    /// exists before creating the configuration files.
    pub fn generate_config(&self) {
        if !Config::check_directory_exists(&self.directory_path) {
            Config::generate_config_directories(&self.directory_path);
            Config::generate_config_file(&self.directory_path);
        } else {
            println!("This coilover application is already initiated");
        }
    }

    /// get_full_config_file will return a copy of the
    /// fully configured configuration file path.
    pub fn get_full_config_file(&self) -> &str {
        self.config_file_path.as_str()
    }

    /// get_full_posts_file will return a copy of the
    /// full path to the posts directory.
    pub fn get_full_posts_file(&self) -> &str {
        self.posts_path.as_str()
    }

    pub fn get_base_path(&self) -> &str {
        self.directory_path.as_str()
    }

    pub fn get_dist_path(&self) -> String {
        self.directory_path.clone().to_owned() + "/dist"
    }

    /// generate_config_file function will generate
    /// a YAML configuration file with the base variables defined.
    /// Then save the file to the currently configured base directory.
    fn generate_config_file(path: &str) {
        let example_yml_config = "
                author:
                    - Example
                title: 
                    - Example
                description: 
                    - This is a example config file
                ";

        let config_yml = YamlLoader::load_from_str(example_yml_config).unwrap(); // Convert the example yml to a actual yml object
        let buffer = &config_yml[0]; // Create a buffer to hold the first items in the yml example

        let mut out_str = String::new(); // output string we want to dump the yml file in and save
        {
            let mut emitter = YamlEmitter::new(&mut out_str); // Create an emitter callback
            emitter.dump(buffer).unwrap(); // dump the YAML object to a buffer String
        }

        let full_path: String = path.to_owned() + "/coilover_config.yml"; // Generate the full path as to where to save the config
        fs::write(full_path, out_str).expect("unable to write file"); // Save the file
    }

    /// generate_config_directories will generate all the necessary
    /// directories for the application.
    fn generate_config_directories(path: &str) {
        let sub_folders = vec!["/template", "/posts", "/dist"];
        for x in &sub_folders {
            let full_path = path.to_owned() + x;
            if !Config::check_directory_exists(&full_path) {
                fs::create_dir_all(full_path).expect("Error cannot create dir");
            } else {
                println!("Coilover is already configured");
                return;
            }
        }
    }

    /// check_directory_exists is a helper function to
    /// check wither a given directory exists or not.
    fn check_directory_exists(path: &String) -> bool {
        let full_path = path.to_owned() + "/coilover_config.yml";
        Path::new(&full_path).exists()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}