use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;

#[derive(Deserialize)]
struct Pkg {
    scripts: Option<HashMap<String, String>>,
}

pub fn fetch_npm_scripts() -> std::io::Result<String> {
    let mut path = env::current_dir().unwrap_or(std::path::PathBuf::from("."));
    path.push("package.json");
    let file = File::open(path)?;
    let package: Pkg = serde_json::from_reader(file)?;

    let scripts = match package.scripts {
        Some(scripts) => scripts
            .keys()
            .map(|script| script.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        None => String::new(),
    };
    Ok(scripts)
}

#[test]
fn test_fetch_scripts() {
    let output = fetch_npm_scripts().unwrap();
    let output = output.trim();
    let mut scripts: Vec<&str> = output.split('\n').collect();
    scripts.sort();
    assert_eq!(scripts, ["build", "commit", "dev", "lint"]);
}
