use std::env;

#[macro_use]
extern crate serde_derive;

mod scripts;
mod deps;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "scripts" => print!("{}", scripts::fetch_npm_scripts()),
        "add" => println!("{}", deps::return_dependencies(None)),
        "add-dev" => println!("{}", deps::return_dev_dependencies(None)),
        "remove" => print!("{}", deps::fetch_installed_packages()),
        _ => ()
    };
}
