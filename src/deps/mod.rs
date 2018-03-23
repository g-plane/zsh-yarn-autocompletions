use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

mod packages;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Pkg {
    dependencies: Option<HashMap<String, String>>,
    devDependencies: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
struct UserCustomDeps {
    dependencies: Option<Vec<String>>,
    dev_dependencies: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
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

pub fn return_dependencies(path: Option<PathBuf>) -> String {
    let path = match path {
        Some(path) => path,
        None => default_custom_deps_file_path()
    };

    let dependencies = packages::dependencies();
    let mut dependencies: Vec<String> = dependencies
        .iter()
        .map(|&dep| String::from(dep))
        .collect();
    let custom = fetch_custom_dependencies(&path);
    dependencies.extend(custom);
    
    let exclude = fetch_exclude_dependencies(&path);
    let dependencies: Vec<String> = dependencies
        .into_iter()
        .filter(|dep| !exclude.contains(dep))
        .collect();

    dependencies.join("\n")
}

pub fn return_dev_dependencies(path: Option<PathBuf>) -> String {
    let path = match path {
        Some(path) => path,
        None => default_custom_deps_file_path()
    };

    let dev_dependencies = packages::dev_dependencies();
    let mut dev_dependencies: Vec<String> = dev_dependencies
        .iter()
        .map(|&dep| String::from(dep))
        .collect();
    let custom = fetch_custom_dev_dependencies(&path);
    dev_dependencies.extend(custom);

    let exclude = fetch_exclude_dependencies(&path);
    let dev_dependencies: Vec<String> = dev_dependencies
        .into_iter()
        .filter(|dep| !exclude.contains(dep))
        .collect();

    dev_dependencies.join("\n")
}

fn default_custom_deps_file_path() -> PathBuf {
    match env::home_dir() {
        Some(path) => path.join(".yarn-autocompletions.yml"),
        None => PathBuf::new()
    }
}

fn fetch_custom_dependencies(path: &PathBuf) -> Vec<String> {
    if let Ok(file) = File::open(path) {
        match serde_yaml::from_reader(file) {
            Ok(custom) => {
                let custom: UserCustomDeps = custom;
                match custom.dependencies {
                    Some(deps) => deps,
                    None => vec![]
                }
            }
            Err(_) => vec![]
        }
    } else {
        vec![]
    }
}

fn fetch_custom_dev_dependencies(path: &PathBuf) -> Vec<String> {
    if let Ok(file) = File::open(path) {
        match serde_yaml::from_reader(file) {
            Ok(custom) => {
                let custom: UserCustomDeps = custom;
                match custom.dev_dependencies {
                    Some(deps) => deps,
                    None => vec![]
                }
            }
            Err(_) => vec![]
        }
    } else {
        vec![]
    }
}

fn fetch_exclude_dependencies(path: &PathBuf) -> Vec<String> {
    if let Ok(file) = File::open(path) {
        match serde_yaml::from_reader(file) {
            Ok(custom) => {
                let custom: UserCustomDeps = custom;
                match custom.exclude {
                    Some(deps) => deps,
                    None => vec![]
                }
            }
            Err(_) => vec![]
        }
    } else {
        vec![]
    }
}

#[test]
fn test_fetch_installed_packages() {
    let output = fetch_installed_packages();
    let output = output.trim();
    let mut packages: Vec<&str> = output.split('\n').collect();
    packages.sort();
    assert_eq!(packages, ["a", "b", "c", "d"]);
}

#[test]
fn test_return_dependencies() {
    let path = PathBuf::from("yarn-autocompletions.example.yml");
    let output = return_dependencies(Some(path));
    assert!(output.contains("vue"));
    assert!(!output.contains("axios"));
}

#[test]
fn test_return_dev_dependencies() {
    let path = PathBuf::from("yarn-autocompletions.example.yml");
    let output = return_dev_dependencies(Some(path));
    assert!(output.contains("babel-core"));
    assert!(!output.contains("typescript"));
}
