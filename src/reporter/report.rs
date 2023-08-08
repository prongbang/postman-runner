use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::{config, date, executor, filex};
use crate::reporter::template::dashboard;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reporter {
    #[serde(skip)]
    pub report_url: String,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionReport {
    pub report_url: String,
    pub collection_name: String,
    pub interactions: i64,
    pub assertions: i64,
    pub failed: i64,
    pub skipped: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardReport {
    pub title: String,
    pub created_at: String,
    pub total_collection: i64,
    pub total_iterations: i64,
    pub total_assertions: i64,
    pub total_failed_tests: i64,
    pub total_skipped_tests: i64,
    pub collections: Vec<CollectionReport>,
}

pub fn load(report_path: &str, name: &str) -> Reporter {
    let filename = format!("{}/.{}.json", report_path, name);
    let file = File::open(filename.as_str()).expect("Failed to open the file");

    // Deserialize the JSON string into a Reporter struct
    let reporter: Reporter = serde_json::from_reader(file).expect("Failed to deserialize JSON");

    reporter
}

pub async fn gen(config: &config::conf::Config) {
    if config.report.filename.is_empty() {
        return;
    }

    let report_path = filex::get_path(config.report.filename.as_str());
    let mut test_reporters: Vec<Reporter> = Vec::new();

    // Prepare report
    for cmd in &config.commands {
        if cmd.command.contains(executor::execute::NEWMAN_CLI) {
            let mut report = load(report_path.as_str(), cmd.name.as_str());
            report.report_url = format!("{}.html", &cmd.name);
            test_reporters.push(report);
        }
    }

    // Check test reporters
    if test_reporters.is_empty() {
        return;
    }

    println!("→ Generating");
    println!("↳ Report {}", &config.report.filename);

    // Generate report
    let mut collection_report: Vec<CollectionReport> = Vec::new();
    let total_collection: i64 = test_reporters.len() as i64;
    let mut total_iterations: i64 = 0;
    let mut total_assertions: i64 = 0;
    let mut total_failed_tests: i64 = 0;
    let mut total_skipped_tests: i64 = 0;
    for report in test_reporters {
        let iterations = report.run.stats.iterations.total;
        let assertions = report.run.stats.assertions.total;
        let failed_tests = report.run.stats.iterations.failed
            + report.run.stats.items.failed
            + report.run.stats.scripts.failed
            + report.run.stats.prerequests.failed
            + report.run.stats.requests.failed
            + report.run.stats.tests.failed
            + report.run.stats.assertions.failed
            + report.run.stats.test_scripts.failed
            + report.run.stats.prerequest_scripts.failed;
        let mut skipped_tests: i64 = 0;
        for exe in report.run.executions.iter() {
            for assertion in exe.assertions.iter() {
                if assertion.skipped {
                    skipped_tests += 1;
                }
            }
        }

        collection_report.push(CollectionReport {
            report_url: report.report_url.to_string(),
            collection_name: report.collection.info.name,
            interactions: iterations.clone(),
            assertions: assertions.clone(),
            failed: failed_tests.clone(),
            skipped: skipped_tests.clone(),
        });

        total_iterations += iterations;
        total_assertions += assertions;
        total_failed_tests += failed_tests;
        total_skipped_tests += skipped_tests;
    }

    println!("\t↳ Total collection: {}", &total_collection);
    println!("\t↳ Total iterations: {}", &total_iterations);
    println!("\t↳ Total assertions: {}", &total_assertions);
    println!("\t↳ Total failed tests: {}", &total_failed_tests);
    println!("\t↳ Total skipped tests: {}", &total_skipped_tests);

    // Remove cache files
    for cmd in &config.commands {
        if cmd.command.contains(executor::execute::NEWMAN_CLI) {
            let filename = format!("{}/.{}.json", report_path.as_str(), cmd.name.as_str());
            filex::remove(filename.as_str());
        }
    }

    let mut title = "Postman Runner Dashboard";
    if !config.report.name.is_empty() {
        title = config.report.name.as_str();
    }

    let dashboard_report = DashboardReport {
        title: title.to_string(),
        created_at: date::chrono::current(),
        total_collection,
        total_iterations,
        total_assertions,
        total_failed_tests,
        total_skipped_tests,
        collections: collection_report,
    };
    let html = dashboard(dashboard_report);
    if let Ok(_) = filex::write(config.report.filename.as_str(), html.as_str()) {
        println!("\t↳ File created at: {}", config.report.filename);
    } else {
        println!("\t↳ Cannot create file at: {}", config.report.filename);
    }
}