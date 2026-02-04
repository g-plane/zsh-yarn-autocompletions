mod deps;
mod scripts;

#[async_std::main]
async fn main() {
    let mut args = std::env::args();
    args.next(); // The first argument is the program itself.

    match args.next().unwrap_or_default().as_str() {
        "scripts" => print!("{}", scripts::fetch_npm_scripts().await.unwrap_or_default()),
        "add" => println!(
            "{}",
            deps::return_dependencies(None).await.unwrap_or_default()
        ),
        "add-dev" => println!(
            "{}",
            deps::return_dev_dependencies(None)
                .await
                .unwrap_or_default()
        ),
        "remove" => print!(
            "{}",
            deps::fetch_installed_packages().await.unwrap_or_default()
        ),
        "why" => print!("{}", deps::list_node_modules().await.unwrap_or_default()),
        _ => (),
    };
}
