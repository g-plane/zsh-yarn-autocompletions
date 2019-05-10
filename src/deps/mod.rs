use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::io;
use std::iter::FromIterator;
use std::path::PathBuf;

mod packages;

#[derive(Deserialize)]
struct Pkg {
    dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
struct UserCustomDeps {
    dependencies: Option<Vec<String>>,
    dev_dependencies: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

pub fn fetch_installed_packages() -> std::io::Result<String> {
    let mut path = env::current_dir().unwrap_or(PathBuf::from("."));
    path.push("package.json");

    let file = File::open(path)?;
    let pkg: Pkg = serde_json::from_reader(file)?;
    let mut packages: Vec<String> = vec![];

    if let Some(dependencies) = pkg.dependencies {
        for dependency in dependencies.keys() {
            packages.push(dependency.clone());
        }
    }
    if let Some(dependencies) = pkg.dev_dependencies {
        for dependency in dependencies.keys() {
            packages.push(dependency.clone());
        }
    }
    Ok(packages.join("\n"))
}

pub fn return_dependencies(path: Option<PathBuf>) -> io::Result<String> {
    let path = path.unwrap_or(default_custom_deps_file_path());

    let mut dependencies = packages::dependencies();
    for dep in fetch_custom_dependencies(&path)? {
        dependencies.insert(dep);
    }

    let exclude = fetch_exclude_dependencies(&path)?;
    let dependencies = dependencies.difference(&exclude);
    Ok(dependencies.fold(String::new(), |mut acc, cur| {
        acc.push_str(cur);
        acc.push_str("\n");
        acc
    }))
}

pub fn return_dev_dependencies(path: Option<PathBuf>) -> io::Result<String> {
    let path = path.unwrap_or(default_custom_deps_file_path());

    let mut dev_dependencies = packages::dev_dependencies();
    for dep in fetch_custom_dev_dependencies(&path)? {
        dev_dependencies.insert(dep);
    }

    let exclude = fetch_exclude_dependencies(&path)?;
    let dev_dependencies = dev_dependencies.difference(&exclude);
    Ok(dev_dependencies.fold(String::new(), |mut acc, cur| {
        acc.push_str(cur);
        acc.push_str("\n");
        acc
    }))
}

fn default_custom_deps_file_path() -> PathBuf {
    env::home_dir()
        .map(|path| path.join(".yarn-autocompletions.yml"))
        .unwrap_or(PathBuf::new())
}

fn fetch_custom_dependencies(path: &PathBuf) -> io::Result<Vec<String>> {
    let custom = serde_yaml::from_reader(File::open(path)?).unwrap_or(UserCustomDeps {
        dependencies: None,
        dev_dependencies: None,
        exclude: None,
    });
    Ok(custom.dependencies.unwrap_or(vec![]))
}

fn fetch_custom_dev_dependencies(path: &PathBuf) -> io::Result<Vec<String>> {
    let custom = serde_yaml::from_reader(File::open(path)?).unwrap_or(UserCustomDeps {
        dependencies: None,
        dev_dependencies: None,
        exclude: None,
    });
    Ok(custom.dev_dependencies.unwrap_or(vec![]))
}

fn fetch_exclude_dependencies(path: &PathBuf) -> io::Result<HashSet<String>> {
    let custom = serde_yaml::from_reader(File::open(path)?).unwrap_or(UserCustomDeps {
        dependencies: None,
        dev_dependencies: None,
        exclude: None,
    });
    Ok(HashSet::from_iter(
        custom.exclude.unwrap_or(Vec::new()).iter().cloned(),
    ))
}

pub fn list_node_modules<'a>() -> io::Result<String> {
    let cwd = env::current_dir().unwrap_or(PathBuf::from("."));
    let directories = fs::read_dir(cwd.join("node_modules"))?;
    let items = directories
        .map(|dir| match dir {
            Ok(entry) => entry.file_name().into_string().unwrap_or(String::new()),
            Err(_) => String::new(),
        })
        .collect::<Vec<_>>()
        .join("\n");
    Ok(items)
}

#[test]
fn test_fetch_installed_packages() {
    let output = fetch_installed_packages().unwrap();
    let output = output.trim();
    let mut packages: Vec<&str> = output.split('\n').collect();
    packages.sort();
    assert_eq!(packages, ["a", "b", "c", "d"]);
}

#[test]
fn test_return_dependencies() {
    let path = PathBuf::from("yarn-autocompletions.example.yml");
    let output = return_dependencies(Some(path)).unwrap();
    assert!(output.contains("vue"));
    assert!(!output.contains("axios"));
}

#[test]
fn test_return_dev_dependencies() {
    let path = PathBuf::from("yarn-autocompletions.example.yml");
    let output = return_dev_dependencies(Some(path)).unwrap();
    assert!(output.contains("@babel/core"));
    assert!(!output.contains("gulp"));
}
