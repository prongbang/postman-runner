use regex::Regex;

#[allow(dead_code)]
pub fn extract_url(text: &str) -> Option<String> {
    let re = Regex::new(r"https?://go.postman.co/\S+").unwrap();
    if let Some(captures) = re.captures(text) {
        Some(captures[0].to_string())
    } else {
        None
    }
}