use std::fs;
use yaml_rust::{YamlEmitter, YamlLoader};

pub struct Config {
    directory_path: String, // Base path for the directory where everything is stored
}

impl Config {
    pub fn new() -> Config {
        Config {
            directory_path: String::from("./wastegate"), 
        }
    }

    pub fn generate_config_file(path: &str) {
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

        let full_path: String = path.to_owned() + "/wastegate_config.yml"; // Generate the full path as to where to save the config

        fs::write(full_path, out_str).expect("unable to write file"); // Save the file
    }

    pub fn generate_config_directories(path: &str) {
        let sub_folders = vec!["/template", "/posts", "/dist"];
        for x in &sub_folders {
            let full_path = path.to_owned() + x;
            fs::create_dir_all(full_path).expect("error cannot create dir");
        }
    }

    pub fn get_directory_path(&self) -> &str {
        self.directory_path.as_str().as_ref()
    }
}