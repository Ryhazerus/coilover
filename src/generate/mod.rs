//! Generate module
//! 
//! Generate module is the custom module that generates files based on the 
//! provided markdown files. This module also converts the markdown files
//! into HTML files.

use std::fs;
use yaml_rust::{YamlLoader};

pub struct Generate {}

/// Generate implementation for the base functionality
/// methods.
impl Generate {

    /// Create a new instance of the Generate struct to
    /// access its functionality.
    pub fn new() -> Generate {
        Generate {}
    }

    /// Build function is a accessor method to generate the 
    /// body of the generated HTML page and implement the markdown
    /// posts into the HTML file as HTML articles.
    pub fn build(&self,config: &str) {
        Generate::include_metadata_in_template(&self, config);
        Generate::generate_template(&self, config);
    }

    /// Generate_template function will loop through the specified 
    /// posts directory and converts all the markdown posts to HTML
    /// articles that can be implemented in a single HTML page.
    pub fn generate_template(&self, config: &str) {
        let mut result: String = String::from("");
        if let Ok(entries) = fs::read_dir(config) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(_metadata) = entry.metadata() {
                        let path: String =
                            String::from(entry.path().into_os_string().into_string().unwrap());
                        result.push_str(
                            Generate::include_in_template(
                                &Generate::read_raw_files(&path),
                                &Generate::get_html_template(),
                            )
                            .as_str(),
                        );
                    } else {
                        println!("Couldn't get metadata for {:?}", entry.path());
                    }
                }
            }
        }
        // Save the generated template to the dist folder.
        Generate::save_template(&String::from("./dist"), &result);
    }

    /// Read_raw_files will real in a file to string,
    /// if the file cannot be read we catch the error thrown 
    /// by the filesystem and print it to the console.
    fn read_raw_files(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    /// Include_in_template will include a given string into a HTML template
    /// file.
    fn include_in_template(file: &String, template: &String) -> String {
        let search_param = "{{%elements%}}";
        let mut html: String = markdown::to_html(file);
        html.push_str("<hr>");
        let build: String = html.as_str().to_owned() + search_param;
        let result = template.replace(search_param, build.as_str());
        result
    }

    /// Include_metadata_in_template function will include all the 
    /// metadata from the config file into the HTML template 
    /// that needs to be generated. 
    fn include_metadata_in_template(&self, config: &str) -> String {
        // Load the yaml config file
        let config = YamlLoader::load_from_str(&Generate::read_raw_files(config)).unwrap();

        // Load all the variables from the yaml file.
        let title: &str = config[0]["title"][0].as_str().unwrap();
        let description: &str = config[0]["description"][0].as_str().unwrap();
        let author: &str = config[0]["author"][0].as_str().unwrap();

        // Parse the loaded yml file into the template file.
        let mut buffer = Generate::get_html_template().replace("{{%author%}}", author);
        buffer = buffer.replace("{{%title%}}", title);
        buffer = buffer.replace("{{%description%}}", description);

        // Save the newly generated file to disk.
        buffer
    }

    /// Save_template function will save a given file to a given path
    /// File will throw an error if the directory already exists.
    fn save_template(path: &String, file_contents: &String) {
        // Create the directory id it doesn't exist
        fs::create_dir_all(path).expect("error cannot create dir");
        // Create the full path with a filename
        let filename: String = path.as_str().to_owned() + "/index.htm";
        // Write the post to a file
        let cleaned = Generate::clean_up(file_contents);
        fs::write(filename, cleaned).expect("Unable to write file");
    }

    /// Get_html_template will return the base template for the
    /// blog that needs to be generated.
    pub fn get_html_template() -> String {
        let html: &str = "
                <!doctype html>
                <html lang='en'>
                <head>
                <meta charset='utf-8'>

                <title>{{%title%}}</title>
                <meta name='description' content='{{%description%}}'>
                <meta name='author' content='{{%author%}}'>
                <style type='text/css'>body{margin:40px
                auto;max-width:650px;line-height:1.6;font-size:18px;color:#444;padding:0
                10px}h1,h2,h3{line-height:1.2}</style>
                </head>

                <body>
                    {{%elements%}}
                </body>
                </html>";
        String::from(html)
    }

    /// Clean_up function wil remove any trace of the left over
    /// variables we might of missed during the markdown HTML 
    /// file generation.
    pub fn clean_up(html: &String) -> String {
        let search_param = "{{%elements%}}";
        let result = html.replace(search_param, "");
        result
    }
}
