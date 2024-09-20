use std::fs::File;
use std::path::Path;
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
    pub stats: Option<Stats>,
    pub timings: Option<Timings>,
    pub executions: Vec<Execution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub iterations: Option<Iterations>,
    pub items: Option<Items>,
    pub scripts: Option<Scripts>,
    pub prerequests: Option<Prerequests>,
    pub requests: Option<Requests>,
    pub tests: Option<Tests>,
    pub assertions: Option<Assertions>,
    pub test_scripts: Option<TestScripts>,
    pub prerequest_scripts: Option<PrerequestScripts>,
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
    pub assertions: Option<Vec<Assertion>>,
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
    pub run_duration: f64,
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
    pub total_run_durations: f64,
    pub collections: Vec<CollectionReport>,
}

pub fn load(report_path: &str, name: &str) -> Result<Reporter, ()> {
    let filename = format!("{}/.{}.json", report_path, name);
    let path = Path::new(filename.as_str());

    if path.exists() {
        let file = File::open(filename.as_str()).expect("Failed to open the file");

        // Deserialize the JSON string into a Reporter struct
        let reporter: Reporter = serde_json::from_reader(file).expect("Failed to deserialize JSON");

        return Ok(reporter);
    }

    Err(())
}

pub async fn gen(config: &config::conf::Config) {
    println!("→ Test results");

    let mut filename = "reporter/index.html".to_string();
    if !config.report.filename.is_empty() {
        filename = config.report.filename.clone()
    }

    // Prepare report
    let report_path = filex::get_path(filename.as_str());
    let mut test_reporters: Vec<Reporter> = Vec::new();
    for cmd in &config.commands {
        // Check skipped test collection
        if cmd.is_skipped() {
            continue;
        }

        if cmd.command.contains(executor::execute::NEWMAN_CLI) {
            if let Ok(mut report) = load(report_path.as_str(), cmd.name.as_str()) {
                report.report_url = format!("{}.html", &cmd.name);
                test_reporters.push(report);
            }
        }
    }

    // Check test reporters
    if test_reporters.is_empty() {
        println!("  x No test results");
        return;
    }

    // Generate report
    let mut collection_report: Vec<CollectionReport> = Vec::new();
    let total_collection: i64 = test_reporters.len() as i64;
    let mut total_iterations: i64 = 0;
    let mut total_assertions: i64 = 0;
    let mut total_failed_tests: i64 = 0;
    let mut total_skipped_tests: i64 = 0;
    let mut total_run_durations: f64 = 0.0;
    for report in test_reporters {
        // Total run duration per seconds
        let mut run_duration: f64 = 0.0;
        if let Some(timings) = report.run.timings {
            run_duration = (timings.completed - timings.started) / 1000.0;
        }

        // Calculate iterations, assertions and failed test
        let mut iterations: i64 = 0;
        let mut assertions: i64 = 0;
        let mut failed_tests: i64 = 0;
        if let Some(stats) = report.run.stats {
            if let Some(iteration) = stats.iterations {
                iterations = iteration.total;
                failed_tests += iteration.failed;
            }
            if let Some(assertion) = stats.assertions {
                assertions = assertion.total;
                failed_tests += assertion.failed;
            }
            if let Some(items) = stats.items {
                failed_tests += items.failed;
            }
            if let Some(scripts) = stats.scripts {
                failed_tests += scripts.failed;
            }
            if let Some(prerequests) = stats.prerequests {
                failed_tests += prerequests.failed;
            }
            if let Some(requests) = stats.requests {
                failed_tests += requests.failed;
            }
            if let Some(tests) = stats.tests {
                failed_tests += tests.failed;
            }
            if let Some(test_scripts) = stats.test_scripts {
                failed_tests += test_scripts.failed;
            }
            if let Some(prerequest_scripts) = stats.prerequest_scripts {
                failed_tests += prerequest_scripts.failed;
            }
        }

        // Calculate skipped test
        let mut skipped_tests: i64 = 0;
        for exe in report.run.executions {
            if let Some(assertions) = &exe.assertions {
                for assertion in assertions {
                    if assertion.skipped {
                        skipped_tests += 1;
                    }
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
            run_duration: run_duration.clone(),
        });

        total_iterations += iterations;
        total_assertions += assertions;
        total_failed_tests += failed_tests;
        total_skipped_tests += skipped_tests;
        total_run_durations += run_duration;
    }

    println!("  ✓ Total run durations: {}s", &total_run_durations);
    println!("  ✓ Total collection: {}", &total_collection);
    println!("  ✓ Total iterations: {}", &total_iterations);
    println!("  ✓ Total assertions: {}", &total_assertions);
    println!("  ✓ Total failed tests: {}", &total_failed_tests);
    println!("  ✓ Total skipped tests: {}", &total_skipped_tests);

    println!("\n→ Generate a report");

    // Remove cache files
    for cmd in &config.commands {
        // Check skipped test collection
        if cmd.is_skipped() {
            continue;
        }

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
        total_run_durations,
        collections: collection_report,
    };
    let html = dashboard(dashboard_report);
    if let Ok(_) = filex::write(filename.as_str(), html.as_str()) {
        println!("  ✓ Generated the file at: {}", filename);
    } else {
        println!("  x Cannot generate the file at: {}", filename);
    }
}