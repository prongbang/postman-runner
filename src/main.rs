mod reporter;
mod result;
mod executor;
mod config;
mod date;
mod filex;
mod common;

#[tokio::main]
async fn main() {
    let version = "v0.3.0";
    println!("{}", format!("                  __
   ___  ___  ___ / /___ _  ___ ____  __________ _____  ___  ___ ____
  / _ \\/ _ \\(_-</ __/  ' \\/ _ `/ _ \\/___/ __/ // / _ \\/ _ \\/ -_) __/
 / .__/\\___/___/\\__/_/_/_/\\_,_/_//_/   /_/  \\_,_/_//_/_//_/\\__/_/ {}
/_/\n", version));

    // Load configurations
    let mut config = config::conf::load();

    // Execute
    executor::execute::exec(&mut config).await;
}
