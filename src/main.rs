use std::env;

mod scripts;
mod deps;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "scripts" => print!("{}", scripts::fetch_npm_scripts().unwrap_or(String::new())),
        "add" => println!("{}", deps::return_dependencies(None)),
        "add-dev" => println!("{}", deps::return_dev_dependencies(None)),
        "remove" => print!("{}", deps::fetch_installed_packages()),
        "why" => print!("{}", deps::list_node_modules()),
        _ => ()
    };
}
