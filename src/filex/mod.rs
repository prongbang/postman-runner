use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::Path;

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