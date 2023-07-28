use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use crate::{command, config, reporter, result};

pub async fn run(config: &config::conf::Config) {
    let mut test_collections: Vec<result::parser::TestCollection> = Vec::new();

    let report = false;

    // Run command
    for cmd in &config.commands {
        let mut command = cmd.command.to_string();
        if !config.report.is_empty() {
            command += &format!(
                " -r json,htmlextra --reporter-json-export reporter/{}.json --reporter-htmlextra-export reporter/{}.html",
                &cmd.name,
                &cmd.name
            );
        }
        println!("{}:\n↳ {}", &cmd.name, command);

        let stream = command::cmd::run_stream(&command);
        pin_mut!(stream); // needed for iteration
        let mut output = String::from("");
        while let Some(value) = stream.next().await {
            println!("{}", value.output);

            if report {
                output.push_str(value.output.as_str());

                // Parse test to struct
                if let Some(test_name) = result::parser::parse_test_name(value.output.as_str()) {
                    // TODO set test name to struct
                } else {
                    if let Some(test_request) = result::parser::parse_test_request(value.output.as_str()) {
                        // TODO set test request to struct
                    } else {
                        result::parser::parse_test_function(value.output.as_str());
                    }
                }

                // Parse result to struct
                if value.success {
                    let result = result::parser::parse_result(format!("{}", output).as_str());
                    test_collections.push(result::parser::TestCollection { name: cmd.name.to_string(), test_result: result });
                }
            }
        }
    }

    // Report
    reporter::report::gen(&config).await;
}
