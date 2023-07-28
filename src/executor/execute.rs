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

    // Reporter
    let mut  reporter = "html";
    if !config.report.reporter.is_empty() {
        reporter = &config.report.reporter.as_str();
    }

    // Run command
    for cmd in &config.commands {
        let mut command = cmd.command.to_string();
        if !config.report.filename.is_empty() {
            command += &format!(
                " -r {}json,{} --reporter-json-export reporter/.{}.json --reporter-{}-export reporter/{}.html",
                cli,
                reporter,
                &cmd.name,
                reporter,
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
