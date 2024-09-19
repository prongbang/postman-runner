use std::error::Error;
use regex::Regex;
use url::Url;
use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::path::Path;

pub fn extract_url(command: &str) -> Option<String> {
    let re = Regex::new(r"(https?://\S+)").unwrap();
    if let Some(caps) = re.captures(&command) {
        let url = &caps[1];
        return Some(url.to_string());
    }
    None
}

pub fn extract_path(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).unwrap();
    let path = parsed_url.path();
    if path.is_empty() {
        return None;
    }
    Some(path.to_string())
}

pub fn extract_name(url: &str) -> Option<String> {
    if let Some(file_path) = extract_path(url) {
        let path = Path::new(file_path.as_str());
        return path.file_name()
            .map(|name| name.to_string_lossy().into_owned());
    }
    None
}

pub fn download_file(url: &str, dest_path: &str) -> Result<(), Box<dyn Error>> {
    let response = get(url)?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }
    let mut content = Cursor::new(response.bytes()?);
    let mut file = File::create(dest_path)?;
    copy(&mut content, &mut file)?;

    Ok(())
}