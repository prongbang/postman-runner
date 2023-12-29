use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::Path;

pub fn get_path(filename: &str) -> String {
    let parent_dir = Path::new(filename).parent().unwrap();
    parent_dir.to_path_buf().display().to_string()
}

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

pub fn write(filename: &str, content: &str) -> io::Result<()> {
    let parent_dir = Path::new(filename).parent().unwrap();

    // Create the parent directory if it doesn't exist
    if !fs::metadata(&parent_dir).is_ok() {
        fs::create_dir_all(&parent_dir)?;
    }

    // Create the file
    let mut file = File::create(filename)?;

    // Write content to the file
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn remove(file_path: &str) {
    if file_exists(file_path) {
        let _ = fs::remove_file(file_path);
    }
}

#[test]
fn test_get_path() {
    // Given
    let filename = "reporter/feature/report.html";

    // When
    let actual = get_path(filename);

    // Then
    assert_eq!(actual, "reporter/feature")
}