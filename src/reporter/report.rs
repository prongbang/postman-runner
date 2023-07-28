use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::config;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reporter {
    pub collection: Collection,
    pub run: Run,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(rename = "_postman_id")]
    pub postman_id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub stats: Stats,
    pub timings: Timings,
    pub executions: Vec<Execution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub iterations: Iterations,
    pub items: Items,
    pub scripts: Scripts,
    pub prerequests: Prerequests,
    pub requests: Requests,
    pub tests: Tests,
    pub assertions: Assertions,
    pub test_scripts: TestScripts,
    pub prerequest_scripts: PrerequestScripts,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iterations {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scripts {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prerequests {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requests {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tests {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assertions {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScripts {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrerequestScripts {
    pub total: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timings {
    pub response_average: f64,
    pub response_min: f64,
    pub response_max: f64,
    pub response_sd: f64,
    pub dns_average: f64,
    pub dns_min: f64,
    pub dns_max: f64,
    pub dns_sd: f64,
    pub first_byte_average: f64,
    pub first_byte_min: f64,
    pub first_byte_max: f64,
    pub first_byte_sd: f64,
    pub started: f64,
    pub completed: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Execution {
    pub assertions: Vec<Assertion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assertion {
    pub assertion: String,
    pub skipped: bool,
}

pub fn load(name: &str) -> Reporter {
    let file = File::open(format!("reporter/.{}.json", name)).expect("Failed to open the file");

    // Deserialize the JSON string into a Reporter struct
    let reporter: Reporter = serde_json::from_reader(file).expect("Failed to deserialize JSON");

    reporter
}

pub async fn gen(config: &config::conf::Config) {
    println!("→ Generating");
    println!("Report {}", &config.report);

    let mut test_reporters: Vec<Reporter> = Vec::new();

    // Prepare report
    for cmd in &config.commands {
        test_reporters.push(load(cmd.name.as_str()));
    }

    // Generate report
    let total_collection = test_reporters.len();
    let mut total_iterations: i64 = 0;
    let mut total_assertions: i64 = 0;
    let mut total_failed_tests: i64 = 0;
    let mut total_skipped_tests: i64 = 0;
    for report in test_reporters {
        total_iterations += report.run.stats.iterations.total;
        total_assertions += report.run.stats.assertions.total;
        total_failed_tests += report.run.stats.iterations.failed
            + report.run.stats.items.failed
            + report.run.stats.scripts.failed
            + report.run.stats.prerequests.failed
            + report.run.stats.requests.failed
            + report.run.stats.tests.failed
            + report.run.stats.assertions.failed
            + report.run.stats.test_scripts.failed
            + report.run.stats.prerequest_scripts.failed;
        for exe in report.run.executions.iter() {
            for assertion in exe.assertions.iter() {
                if assertion.skipped {
                    total_skipped_tests += 1;
                }
            }
        }
    }

    println!("↳ Total collection: {}", total_collection);
    println!("↳ Total iterations: {}", total_iterations);
    println!("↳ Total assertions: {}", total_assertions);
    println!("↳ Total failed tests: {}", total_failed_tests);
    println!("↳ Total skipped tests: {}", total_skipped_tests);
}