mod command;
mod reporter;
mod result;

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.yml")]
    config: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    commands: Vec<Commands>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Commands {
    name: String,
    command: String,
}

fn main() {
    println!("                  __
   ___  ___  ___ / /___ _  ___ ____  __________ _____  ___  ___ ____
  / _ \\/ _ \\(_-</ __/  ' \\/ _ `/ _ \\/___/ __/ // / _ \\/ _ \\/ -_) __/
 / .__/\\___/___/\\__/_/_/_/\\_,_/_//_/   /_/  \\_,_/_//_/_//_/\\__/_/ v.1.0.0
/_/\n");

    let args = Args::parse();

    // Parse yml to struct
    let f = std::fs::File::open(args.config).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    // Run command
    for cmd in config.commands {
        let command = cmd.command.as_str();
        println!("{}:\n↳ {}", cmd.name, command);
        match command::cmd::run(command) {
            Ok(output) => {
                println!("{}", output)
            }
            Err(error) => {
                println!("\tCommand execution failed:\n{}", error);
            }
        }
    }
}
