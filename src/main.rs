#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;
extern crate handlebars;

use rocket_contrib::Template;
use rocket::response::{NamedFile, Redirect};
use std::collections::HashMap;
use serde_json::Value as Json;
use std::io;
use std::path::{Path, PathBuf};

mod custom_io;

//main function
fn main()
{
    start_server();
}

//start rocket server, mount routes
fn start_server() {

    rocket::ignite()
        .mount("/", routes![main_page, files])
        .launch();
}

//main page -> show articles in grid
#[get("/")]
fn main_page() -> Template {
    get_template("index", "projects.txt").unwrap()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

//template load
fn get_template(template_name: &str, json_file_name: &str) -> Result<Template, String> {
    let mut result: Result<Template, String>;
    match get_json_string(json_file_name) {
        Ok(json_string) => {
            let json_parsing = serde_json::from_str(&json_string);
            match json_parsing {
                Ok(parsed_json) => {
                    let mut context: HashMap<String, Json> = HashMap::new();
                    context.insert("projects".to_string(), parsed_json);
                    let template = Template::render(template_name, &context);
                    result = Ok(template);
                }
                Err(_) => result = Err("failed to parse json".to_string()),
            }
        }
        Err(_) => result = Err("failed to load json file".to_string()),

    }
    result

}

//load json from file as string
fn get_json_string(file_path: &str) -> Result<String, io::Error> {
    let json_string: String = custom_io::load_from_file(&file_path)?;
    Ok(json_string)
}