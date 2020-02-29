use std::collections::HashSet;

macro_rules! hash_set {
    ($($e:expr,)*) => {{
        let mut set = ::std::collections::HashSet::new();

        $(set.insert($e.to_string());)*

        set
    }}
}

pub fn dependencies() -> HashSet<String> {
    hash_set![
        "@babel/runtime",
        "async",
        "axios",
        "body-parser",
        "chalk",
        "commander",
        "cookie-parser",
        "debug",
        "express",
        "next",
        "preact",
        "preact-compat",
        "react",
        "react-dom",
        "request",
        "styled-components",
        "vue",
        "vuex",
    ]
}

pub fn dev_dependencies() -> HashSet<String> {
    hash_set![
        "@babel/cli",
        "@babel/core",
        "@babel/preset-env",
        "@babel/preset-react",
        "@babel/preset-typescript",
        "ava",
        "babel-eslint",
        "babel-loader",
        "css-loader",
        "eslint",
        "file-loader",
        "gulp",
        "jest",
        "mocha",
        "rollup",
        "standard",
        "style-loader",
        "ts-loader",
        "typescript",
        "@typescript/eslint-parser",
        "url-loader",
        "vue-loader",
        "webpack",
        "xo",
    ]
}
