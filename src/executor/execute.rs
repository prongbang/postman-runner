use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use crate::{command, config, reporter};

pub async fn run(config: &config::conf::Config) {
    println!("→ Running");

    // Logger
    let mut cli = "";
    if config.logger {
        cli = "cli,";
    }

    // Run command
    for cmd in &config.commands {
        let mut command = cmd.command.to_string();
        if !config.report.is_empty() {
            command += &format!(
                " -r {}json,htmlextra --reporter-json-export reporter/.{}.json --reporter-htmlextra-export reporter/{}.html",
                cli,
                &cmd.name,
                &cmd.name,
            );
        }
        println!("{}:\n↳ {}", &cmd.name, command);

        let stream = command::cmd::run_stream(&command);
        pin_mut!(stream); // needed for iteration
        while let Some(value) = stream.next().await {
            if config.logger {
                println!("{}", value.output);
            }
        }
    }

    // Report
    reporter::report::gen(&config).await;
}
