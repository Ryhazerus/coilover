//! Generate module
//!
//! Generate module is the custom module that generates files based on the
//! provided markdown files. This module also converts the markdown files
//! into HTML files.

use std::fs;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use yaml_rust::YamlLoader;
use std::path::MAIN_SEPARATOR;

pub struct Generate {}

/// Generate implementation for the base functionality
/// methods.
impl Generate {

    /// Create a new instance of the Generate struct to
    /// access its functionality.
    pub fn new() -> Generate {
        Generate {}
    }

    pub fn build_posts(&self, post_path: String, dist_path: String, config: String) {
        // Get all the paths in the posts folder.
        let posts: Vec<String> = Generate::get_post_files(post_path);

        // Define a base number of worker threads.
        let mut n_workers: usize = 10;

        // Create as many workers as there are posts if the amount of posts is under 100.
        if posts.len() <= 100 && posts.len() > 0 {
            n_workers = posts.len();
        }

        println!("number of threads: {}", n_workers);

        // Create a thread pool with the amount of workers
        let pool = ThreadPool::new(n_workers);

        // Loop over the defined paths and spawn a thread to handle the mark down conversions per post.
        for post in posts {
            let filename_split: Vec<&str> = post.split(MAIN_SEPARATOR).collect();

            let filename : String = String::from(filename_split[filename_split.len() -1]);

            let owned_path = dist_path.to_owned();
            let owned_config = config.to_owned();

            pool.execute(move || {
                Generate::generate_posts(post.clone(), owned_path, filename.clone(), owned_config);
            });
        }

        pool.join();
    }

    fn generate_index(dist_path: String){
        
    }

    fn generate_posts(post_path: String, dist_path: String, filename: String, config: String) {
        let metatdata = Generate::include_metadata_in_template(config);
        let raw_file: String = Generate::read_raw_files(post_path.as_str());
        let html = Generate::markdown_to_html(raw_file.as_str(), metatdata.as_str());
        Generate::save_file(dist_path.as_str(), html.as_str(), filename.as_str());

    }

    fn get_post_files(config: String) -> Vec<String> {
        let mut posts: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(config) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(_metadata) = entry.metadata() {
                        let path = entry.path().into_os_string().into_string().unwrap();
                        posts.push(path);
                    } else {
                        println!("Couldn't get metadata for {:?}", entry.path());
                    }
                }
            }
        }
        posts
    }

    /// Read_raw_files will real in a file to string,
    /// if the file cannot be read we catch the error thrown
    /// by the filesystem and print it to the console.
    fn read_raw_files(path: &str) -> String {
        fs::read_to_string(path).expect("Unable to read file")
    }

    /// Include_in_template will include a given string into a HTML template
    /// file.
    fn markdown_to_html(file: &str, template: &str) -> String {
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
    fn include_metadata_in_template(config: String) -> String {
        // Load the yaml config file
        let config = YamlLoader::load_from_str(&Generate::read_raw_files(config.as_str())).unwrap();

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

    /// save_file function will save a given file to a given path
    /// File will throw an error if the directory already exists.
    fn save_file(path: &str, file_contents: &str, filename: &str) {
        // Create the directory id it doesn't exist
        fs::create_dir_all(path).expect("Error cannot create dir");
        // Create the full path with a filename
        // Write the post to a file
        let cleaned = Generate::clean_up(file_contents);


        let test: String = MAIN_SEPARATOR.to_string();
        
        let full_path: String = path.to_owned() + test.as_str() + &filename.replace(".md", ".htm").as_str();

        fs::write(full_path, cleaned).expect("Unable to write file");
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
    pub fn clean_up(html: &str) -> String {
        let search_param = "{{%elements%}}";
        let result = html.replace(search_param, "");
        result
    }
}
