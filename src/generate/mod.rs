use crate::config::Config;
use std::fs;
use yaml_rust::{YamlEmitter, YamlLoader};

pub struct Generate {
    pub config: Config,
}

impl Generate {
    pub fn new(config: Config) -> Generate {
        Generate { config: config }
    }

    fn read_raw_files(path: &String) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    pub fn generate_template(posts_path: &String, template_path: &String) {
        if let Ok(entries) = fs::read_dir(posts_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        let path: String =
                            String::from(entry.path().into_os_string().into_string().unwrap());
                        println!("{}", path);

                        Generate::include_in_template(
                            &Generate::read_raw_files(&path),
                            &template_path,
                        )
                    } else {
                        println!("Couldn't get metadata for {:?}", entry.path());
                    }
                }
            }
        }
    }

    fn include_in_template(file: &String, template_path: &String) {
        // Load the template file we want to manipulate
        let template = Generate::read_raw_files(&template_path);

        let search_param = "{{%elements%}}";

        let html: String = markdown::to_html(file);

        let build: String = html.as_str().to_owned() + search_param;

        let result = template.replace(search_param, build.as_str());

        Generate::save_template(&String::from("./dist"), &result);
    }

    pub fn include_metadata_in_template(config_path: &String, template_path: &String) {
        // Load the yaml config file
        let config = YamlLoader::load_from_str(&Generate::read_raw_files(config_path)).unwrap();

        // Load the template file we want to manipulate
        let template = Generate::read_raw_files(&template_path);

        // Load all the variables from the yml file
        let title: &str = config[0]["title"][0].as_str().unwrap();
        let description: &str = config[0]["description"][0].as_str().unwrap();
        let author: &str = config[0]["author"][0].as_str().unwrap();

        // Parse the loaded yml file into the template file
        let mut buffer = template.replace("{{%author%}}", author);
        buffer = buffer.replace("{{%title%}}", title);
        buffer = buffer.replace("{{%description%}}", description);

        // Save the newly generated file to disk
        Generate::save_template(&String::from("./dist"), &buffer);
    }

    fn save_template(path: &String, file_contents: &String) {
        // Create the directory id it doesn't exist
        fs::create_dir_all(path).expect("error cannot create dir");
        // Create the full path with a filename
        let filename: String = path.as_str().to_owned() + "/index.htm";
        // Write the post to a file
        fs::write(filename, file_contents).expect("Unable to write file");
    }
}
