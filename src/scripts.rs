use std::env;
use std::fs::File;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

#[derive(Deserialize)]
struct Pkg {
    scripts: Option<HashMap<String, String>>,
}

pub fn fetch_npm_scripts() -> String {
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
            Ok(package) => {
                let package: Pkg = package;
                match package.scripts {
                    Some(scripts) => scripts
                        .keys()
                        .fold(String::new(), |acc, script| acc + &script + "\n"),
                    None => String::new(),
                }
            }
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    }
}

#[test]
fn test_fetch_scripts() {
    let output = fetch_npm_scripts();
    let output = output.trim();
    let mut scripts: Vec<&str> = output.split('\n').collect();
    scripts.sort();
    assert_eq!(scripts, ["build", "commit", "dev", "lint"]);
}
