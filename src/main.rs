mod deps;
mod scripts;

fn main() {
    let mut args = std::env::args();
    match args.next().unwrap_or(String::new()).as_str() {
        "scripts" => print!("{}", scripts::fetch_npm_scripts().unwrap_or(String::new())),
        "add" => println!(
            "{}",
            deps::return_dependencies(None).unwrap_or(String::new())
        ),
        "add-dev" => println!(
            "{}",
            deps::return_dev_dependencies(None).unwrap_or(String::new())
        ),
        "remove" => print!(
            "{}",
            deps::fetch_installed_packages().unwrap_or(String::new())
        ),
        "why" => print!("{}", deps::list_node_modules().unwrap_or(String::new())),
        _ => (),
    };
}
