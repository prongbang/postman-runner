mod command;
mod reporter;
mod result;
mod executor;
mod config;
mod date;
mod filex;

#[tokio::main]
async fn main() {
    let version = "v0.2.0";
    println!("{}", format!("                  __
   ___  ___  ___ / /___ _  ___ ____  __________ _____  ___  ___ ____
  / _ \\/ _ \\(_-</ __/  ' \\/ _ `/ _ \\/___/ __/ // / _ \\/ _ \\/ -_) __/
 / .__/\\___/___/\\__/_/_/_/\\_,_/_//_/   /_/  \\_,_/_//_/_//_/\\__/_/ {}
/_/\n", version));

    // Load configurations
    let config = config::conf::load();

    // Execute
    executor::execute::run(&config).await;
}
