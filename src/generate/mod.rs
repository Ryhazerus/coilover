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

    fn read_raw_files(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    pub fn generate_template(posts_path: &String, template: &String) {
        print!("{}", posts_path);
        if let Ok(entries) = fs::read_dir(posts_path) {
            println!("test");
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        let path: String =
                            String::from(entry.path().into_os_string().into_string().unwrap());
                        println!("{}", path);
                        Generate::include_in_template(&Generate::read_raw_files(&path), &template)
                    } else {
                        println!("Couldn't get metadata for {:?}", entry.path());
                    }
                }
            }
        }
    }

    pub fn include_in_template(file: &String, template: &String) {
        let search_param = "{{%elements%}}";

        let html: String = markdown::to_html(file);

        let build: String = html.as_str().to_owned() + search_param;

        let result = template.replace(search_param, build.as_str());

        Generate::save_template(&String::from("./dist"), &result);
    }

    pub fn include_metadata_in_template(
        config_path: &str,
        template_path: &str,
        template: &String,
    ) -> String {
        // Load the yaml config file
        let config = YamlLoader::load_from_str(&Generate::read_raw_files(config_path)).unwrap();

        // Load all the variables from the yml file
        let title: &str = config[0]["title"][0].as_str().unwrap();
        let description: &str = config[0]["description"][0].as_str().unwrap();
        let author: &str = config[0]["author"][0].as_str().unwrap();

        // Parse the loaded yml file into the template file
        let mut buffer = template.replace("{{%author%}}", author);
        buffer = buffer.replace("{{%title%}}", title);
        buffer = buffer.replace("{{%description%}}", description);

        // Save the newly generated file to disk
        buffer
    }

    fn save_template(path: &String, file_contents: &String) {
        // Create the directory id it doesn't exist
        fs::create_dir_all(path).expect("error cannot create dir");
        // Create the full path with a filename
        let filename: String = path.as_str().to_owned() + "/index.htm";
        // Write the post to a file
        let cleaned = Generate::clean_up(file_contents);
        fs::write(filename, cleaned).expect("Unable to write file");
    }

    pub fn get_html_template() -> String {
        let html = "
                <!doctype html>
                <html lang='en'>
                <head>
                <meta charset='utf-8'>

                <title>{{%title%}}</title>
                <meta name='description' content='{{%description%}}'>
                <meta name='author' content='{{%author%}}'>

                </head>

                <body>
                    {{%elements%}}
                </body>
                </html>";
        String::from(html)
    }

    pub fn clean_up(html: &String) -> String {
        let search_param = "{{%elements%}}";
        let result = html.replace(search_param, "");
        result
    }
}
