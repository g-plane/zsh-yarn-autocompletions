use std::env;
use std::fs::File;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Pkg {
    dependencies: Option<HashMap<String, String>>,
    devDependencies: Option<HashMap<String, String>>
}

fn print_installed_packages() {
    let file = File::open(
        env::var("PWD").expect("") + &String::from("/package.json")
    );
    match file {
        Ok(file) => {
            let pkg: Pkg = serde_json::from_reader(file).expect("JSON parsing error.");
            match pkg.dependencies {
                Some(dependencies) => {
                    dependencies.keys().for_each(|package| println!("{}", package));
                },
                None => (),
            }
            match pkg.devDependencies {
                Some(dev_dependencies) => {
                    dev_dependencies.keys().for_each(|package| println!("{}", package));
                },
                None => (),
            }
        },
        Err(_) => {
            return;
        }
    };
}

fn main() {
    let dependencies = [
        "vue",
        "vuex"
    ];

    let dev_dependencies = [
        "typescript",
        "eslint",
        "eslint-config-gplane"
    ];

    match env::args().find(|arg| arg == "remove") {
        Some(_) => {
            print_installed_packages();
            return;
        },
        None => (),
    };

    match env::args().find(|arg| arg == "dev") {
        Some(_) => {
            dev_dependencies.iter().for_each(|&dep| println!("{}", dep));
        },
        None => {
            dependencies.iter().for_each(|&dep| println!("{}", dep));
        },
    };
}
