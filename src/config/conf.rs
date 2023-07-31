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
pub struct Config {
    #[serde(default = "Report::new")]
    pub report: Report,
    #[serde(default)]
    pub logger: bool,
    pub commands: Vec<Commands>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commands {
    pub name: String,
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub name: String,
    pub filename: String,
    pub reporter: String,
}

impl Report {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            filename: String::new(),
            reporter: String::new(),
        }
    }
}

pub fn load() -> Config {
    let args = Args::parse();

    // Parse yml to struct
    let file = std::fs::File::open(args.config).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(file).expect("Could not read values.");

    config
}

