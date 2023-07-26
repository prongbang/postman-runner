use regex::{Captures, Regex};

const INTERACTIONS: &str = "iterations";
const REQUESTS: &str = "requests";
const TEST_SCRIPTS: &str = "test-scripts";
const PRE_REQUEST_SCRIPTS: &str = "prerequest-scripts";
const ASSERTIONS: &str = "assertions";

#[derive(Debug, PartialEq)]
pub struct TestData {
    pub executed: u32,
    pub failed: u32,
}

impl TestData {
    pub fn new() -> Self {
        Self { executed: 0, failed: 0 }
    }
}

#[derive(Debug, PartialEq)]
pub struct TestResult {
    pub iterations: TestData,
    pub requests: TestData,
    pub test_scripts: TestData,
    pub prerequest_scripts: TestData,
    pub assertions: TestData,
}

impl TestResult {
    pub fn new() -> Self {
        Self {
            iterations: TestData::new(),
            requests: TestData::new(),
            test_scripts: TestData::new(),
            prerequest_scripts: TestData::new(),
            assertions: TestData::new(),
        }
    }
}

pub fn parse(inline: &str) -> TestResult {
    // Strip escapes from text
    let stripped_inline = strip_ansi_escapes::strip(inline).unwrap_or_default();
    let stripped_text = String::from_utf8(stripped_inline).unwrap();

    let mut test_result = TestResult::new();

    // Create a regular expression pattern to match the labels and values
    let regx = Regex::new(r"(?m)│\s+(\w+(?:-\w+)*)\s+│\s+(\d+)\s+│\s+(\d+)\s+│(?m)").unwrap();
    let result = regx.captures_iter(stripped_text.as_str());
    for captures in result {
        if captures.len() == 4 {
            let label = get_value(&captures, 1);
            if label == INTERACTIONS {
                test_result.iterations.executed = get_value_u32(&captures, 2);
                test_result.iterations.failed = get_value_u32(&captures, 3);
            } else if label == REQUESTS {
                test_result.requests.executed = get_value_u32(&captures, 2);
                test_result.requests.failed = get_value_u32(&captures, 3);
            } else if label == TEST_SCRIPTS {
                test_result.test_scripts.executed = get_value_u32(&captures, 2);
                test_result.test_scripts.failed = get_value_u32(&captures, 3);
            } else if label == PRE_REQUEST_SCRIPTS {
                test_result.prerequest_scripts.executed = get_value_u32(&captures, 2);
                test_result.prerequest_scripts.failed = get_value_u32(&captures, 3);
            } else if label == ASSERTIONS {
                test_result.assertions.executed = get_value_u32(&captures, 2);
                test_result.assertions.failed = get_value_u32(&captures, 3);
            }
        }
    }

    test_result
}

fn get_value(captures: &Captures, index: usize) -> String {
    captures.get(index).map_or("", |m| m.as_str().trim()).to_string()
}

fn get_value_u32(captures: &Captures, index: usize) -> u32 {
    let number: u32 = get_value(captures, index).parse().unwrap();
    return number;
}

#[test]
fn test_parse() {
    let string = "┌─────────────────────────┬─────────────────────┬─────────────────────┐
│                         │            executed │              failed │
├─────────────────────────┼─────────────────────┼─────────────────────┤
│              iterations │                   1 │                   0 │
├─────────────────────────┼─────────────────────┼─────────────────────┤
│                requests │                   1 │                   0 │
├─────────────────────────┼─────────────────────┼─────────────────────┤
│            test-scripts │                   1 │                   0 │
├─────────────────────────┼─────────────────────┼─────────────────────┤
│      prerequest-scripts │                   0 │                   0 │
├─────────────────────────┼─────────────────────┼─────────────────────┤
│              assertions │                   1 │                   0 │
├─────────────────────────┴─────────────────────┴─────────────────────┤
│ total run duration: 1305ms                                          │
├─────────────────────────────────────────────────────────────────────┤
│ total data received: 5.76kB (approx)                                │
├─────────────────────────────────────────────────────────────────────┤
│ average response time: 1230ms [min: 1230ms, max: 1230ms, s.d.: 0µs] │
└─────────────────────────────────────────────────────────────────────┘";

    parse(string);
}