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
