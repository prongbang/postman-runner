use regex::{Captures, Regex};

#[derive(Debug, PartialEq)]
pub struct TestData {
    pub executed: u32,
    pub failed: u32,
}

impl TestData {
    #[allow(dead_code)]
    pub fn new(executed: u32, failed: u32) -> Self {
        Self { executed, failed }
    }
}

pub fn parse(name: &str, inline: &str) {
    // Strip escapes from text
    let stripped_inline = strip_ansi_escapes::strip(inline).unwrap_or_default();
    let stripped_text = String::from_utf8(stripped_inline).unwrap();

    // Create a regular expression pattern to match the labels and values
    let regx = Regex::new(r"(?m)│\s+(\w+(?:-\w+)*)\s+│\s+(\d+)\s+│\s+(\d+)\s+│(?m)").unwrap();
    let result = regx.captures_iter(stripped_text.as_str());
    for captures in result {
        if captures.len() == 4 {
            let label = get_value(&captures, 1);
            let executed = get_value(&captures, 2);
            let failed = get_value(&captures, 3);

            println!("Label: {}", label);
            println!("Executed: {}", executed);
            println!("Failed: {}", failed);
        }
    }
}

fn get_value(captures: &Captures, index: usize) -> String {
    captures.get(index).map_or("", |m| m.as_str().trim()).to_string()
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

    parse("hello", string);
}