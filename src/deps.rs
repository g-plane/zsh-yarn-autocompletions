use std::env;
use std::fs::File;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Pkg {
    dependencies: Option<HashMap<String, String>>,
    devDependencies: Option<HashMap<String, String>>,
}

pub fn fetch_installed_packages() -> String {
    let mut path = String::new();
    match env::var("PWD") {
        Ok(pwd) => {
            path.push_str(&pwd);
            path.push_str("/package.json");
        }
        Err(_) => {
            return String::new();
        }
    }

    match File::open(path) {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(pkg) => {
                let mut packages: Vec<String> = Vec::new();
                let pkg: Pkg = pkg;

                match pkg.dependencies {
                    Some(dependencies) => {
                        dependencies
                            .keys()
                            .for_each(|package| packages.push(package.to_string()));
                    }
                    None => (),
                };
                match pkg.devDependencies {
                    Some(dev_dependencies) => {
                        dev_dependencies
                            .keys()
                            .for_each(|package| packages.push(package.to_string()));
                    }
                    None => (),
                };

                packages
                    .iter()
                    .fold(String::new(), |acc, package| acc + &package + "\n")
            }
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    }
}

pub fn return_dependencies() -> String {
    let dependencies = vec!["vue", "vuex"];
    dependencies.join("\n")
}

pub fn return_dev_dependencies() -> String {
    let dev_dependencies = vec!["typescript", "eslint", "eslint-config-gplane"];
    dev_dependencies.join("\n")
}

#[test]
fn test_fetch_installed_packages() {
    let output = fetch_installed_packages();
    let output = output.trim();
    let mut packages: Vec<&str> = output.split('\n').collect();
    packages.sort();
    assert_eq!(packages, ["a", "b", "c", "d"]);
}
