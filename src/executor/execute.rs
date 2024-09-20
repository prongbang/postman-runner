use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use crate::{common, config, filex, reporter};

pub const NEWMAN_CLI: &str = "newman";
pub const COLLECTION_LOCAL: &str = "local";

pub async fn exec(config: &mut config::conf::Config) {
    if config.sync {
        sync(config).await
    } else {
        run(config).await
    }
}

pub async fn sync(config: &mut config::conf::Config) {
    for cmd in &mut config.commands {
        if let Some(url) = common::extract_url(cmd.command.as_str()) {
            if let Some(file_path) = common::extract_path(url.as_str()) {
                let file_path = format!(".{}", file_path);
                match common::download_file(url.as_str(), file_path.as_str()) {
                    Ok(_) => {
                        println!("→ File downloaded and saved to {}", file_path);
                    }
                    Err(err) => {
                        println!("→ {:?}", err)
                    }
                }
            }
        }
    }
}

pub async fn run(config: &mut config::conf::Config) {
    println!("→ Running");

    // Logger
    let mut cli = "";
    if config.logger {
        cli = "cli,";
    }

    // Reporter
    let mut reporter = "html";
    if !config.report.reporter.is_empty() {
        reporter = &config.report.reporter.as_str();
    }

    let mut report_path = String::new();
    if !config.report.filename.is_empty() {
        report_path = filex::get_path(config.report.filename.as_str());
    }

    // Run command
    for cmd in &mut config.commands {

        // run by command name
        if let Some(name) = &config.command_name {
            if *name == cmd.name {
                cmd.skipped = Some(false);
            } else {
                cmd.skipped = Some(true);
            }
        }

        let mut command = cmd.command.to_string();
        if !report_path.is_empty() && command.contains(NEWMAN_CLI) {

            // convert URL to directory path
            if let Some(collection) = &cmd.collection {
                if collection == COLLECTION_LOCAL {
                    if let Some(url) = common::extract_url(cmd.command.as_str()) {
                        if let Some(path) = common::extract_path(url.as_str()) {
                            command = command.replace(url.as_str(), format!(".{}", path).as_str())
                        }
                    }
                }
            }

            let standalone = cmd.standalone.unwrap_or(false);
            if !standalone {
                command += &format!(
                    " -r {}json,{} --reporter-json-export {}/.{}.json --reporter-{}-export {}/{}.html",
                    cli,
                    reporter,
                    &report_path,
                    &cmd.name,
                    reporter,
                    &report_path,
                    &cmd.name,
                );
            }
        }

        // Check skipped test collection
        if cmd.is_skipped() {
            continue;
        }

        println!("{}:\n↳ {}", &cmd.name, command);

        let stream = cmdwrap::run_stream(&command);
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
