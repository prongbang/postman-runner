mod command;
mod reporter;
mod result;

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

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

#[tokio::main]
async fn main() {
    println!("                  __
   ___  ___  ___ / /___ _  ___ ____  __________ _____  ___  ___ ____
  / _ \\/ _ \\(_-</ __/  ' \\/ _ `/ _ \\/___/ __/ // / _ \\/ _ \\/ -_) __/
 / .__/\\___/___/\\__/_/_/_/\\_,_/_//_/   /_/  \\_,_/_//_/_//_/\\__/_/ v0.1.0
/_/\n");

    let args = Args::parse();

    // Parse yml to struct
    let file = std::fs::File::open(args.config).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(file).expect("Could not read values.");

    // Result
    let mut test_collections: Vec<result::parser::TestCollection> = Vec::new();

    // Run command
    for cmd in config.commands {
        let command = cmd.command.as_str();
        println!("{}:\n↳ {}", cmd.name, command);

        let stream = command::cmd::run_stream(command);
        pin_mut!(stream); // needed for iteration
        let mut output = String::from("");
        while let Some(value) = stream.next().await {
            output.push_str(value.output.as_str());
            println!("{}", value.output);

            // Parse test to struct
            result::parser::parse_test_name(value.output.as_str());
            result::parser::parse_test_req(value.output.as_str());

            // Parse result to struct
            if value.success {
                let result = result::parser::parse_result(format!("{}", output).as_str());
                test_collections.push(result::parser::TestCollection { name: cmd.name.to_string(), test_result: result });
            }
        }
    }

    println!("Result: {:?}", test_collections)
}
