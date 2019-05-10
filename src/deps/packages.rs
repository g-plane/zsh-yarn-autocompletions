use std::collections::HashSet;

pub fn dependencies() -> HashSet<String> {
    let names = [
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
    ];
    names.iter().map(|s| s.to_string()).collect::<HashSet<_>>()
}

pub fn dev_dependencies() -> HashSet<String> {
    let names = [
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
    ];
    names.iter().map(|s| s.to_string()).collect::<HashSet<_>>()
}
