use serde::Deserialize;
use std::env;
use std::fs::File;
use std::collections::HashMap;

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

    let file = File::open(path);
    if let Err(_) = file {
        return String::new();
    }

    let package: Result<Pkg, _> = serde_json::from_reader(file.unwrap());
    if let Err(_) = package {
        return String::new();
    }
    let package = package.unwrap();

    match package.scripts {
        Some(scripts) => scripts
            .keys()
            .map(|script| script.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        None => String::new(),
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
