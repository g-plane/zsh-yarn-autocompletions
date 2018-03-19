use std::env;
use std::fs::File;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Pkg {
    scripts: HashMap<String, String>
}

fn fetch_npm_scripts() {
    let file = File::open(
        env::var("PWD").expect("") + &String::from("/package.json")
    );
    match file {
        Ok(file) => {
            let pkg: Pkg = serde_json::from_reader(file).expect("JSON parsing error.");
            pkg.scripts.keys().for_each(|script| println!("{}", script));
        },
        Err(_) => ()
    };
}

fn main() {
    fetch_npm_scripts();
}
