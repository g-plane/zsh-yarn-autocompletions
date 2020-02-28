use async_std::fs;
use async_std::io::Result;
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
struct Pkg {
    scripts: Option<HashMap<String, String>>,
}

pub async fn fetch_npm_scripts() -> Result<String> {
    let mut path = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    path.push("package.json");
    let content = fs::read(path).await?;
    let package = serde_json::from_slice::<Pkg>(&content)?;

    Ok(package
        .scripts
        .map(|scripts| scripts.keys().join("\n"))
        .unwrap_or_default())
}

#[async_std::test]
async fn test_fetch_scripts() {
    let output = fetch_npm_scripts().await.unwrap();
    let output = output.trim();
    let mut scripts: Vec<&str> = output.split('\n').collect();
    scripts.sort();
    assert_eq!(scripts, ["build", "commit", "dev", "lint"]);
}
