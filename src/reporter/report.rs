use regex::Regex;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
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
    pub response_average: i64,
    pub response_min: i64,
    pub response_max: i64,
    pub response_sd: i64,
    pub dns_average: i64,
    pub dns_min: i64,
    pub dns_max: i64,
    pub dns_sd: i64,
    pub first_byte_average: i64,
    pub first_byte_min: i64,
    pub first_byte_max: i64,
    pub first_byte_sd: i64,
    pub started: i64,
    pub completed: i64,
}

