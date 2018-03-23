pub fn dependencies<'a>() -> Vec<&'a str> {
    vec![
        "react",
        "vue",
        "axios",
    ]
}

pub fn dev_dependencies<'a>() -> Vec<&'a str> {
    vec![
        "babel-core",
        "babel-preset-env",
        "eslint",
        "typescript",
        "webpack",
    ]
}
