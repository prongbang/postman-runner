mod command;
mod reporter;
mod result;
mod executor;
mod config;

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    println!("                  __
   ___  ___  ___ / /___ _  ___ ____  __________ _____  ___  ___ ____
  / _ \\/ _ \\(_-</ __/  ' \\/ _ `/ _ \\/___/ __/ // / _ \\/ _ \\/ -_) __/
 / .__/\\___/___/\\__/_/_/_/\\_,_/_//_/   /_/  \\_,_/_//_/_//_/\\__/_/ v0.1.0
/_/\n");

    // Load configurations
    let config = config::conf::load();

    // Execute
    executor::execute::run(config).await;
}
