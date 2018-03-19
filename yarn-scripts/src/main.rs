use std::env;
use std::fs::File;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Pkg {
    scripts: Option<HashMap<String, String>>
}

fn fetch_npm_scripts() {
    let mut path = String::new();
    match env::var("PWD") {
        Ok(pwd) => {
            path.push_str(&pwd);
            path.push_str("/package.json");
        },
        Err(_) => {
            return;
        }
    }
    match File::open(path) {
        Ok(file) => {
            match serde_json::from_reader(file) {
                Ok(package) => {
                    let package: Pkg = package;
                    match package.scripts {
                        Some(scripts) => scripts.keys().for_each(|script| println!("{}", script)),
                        None => ()
                    }
                },
                Err(_) => ()
            }
        },
        Err(_) => ()
    };
}

fn main() {
    fetch_npm_scripts();
}
