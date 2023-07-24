use std::process::Command;

pub fn run(command: &str) -> Result<String, String> {
    // Execute the command in the shell
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .map_err(|err| format!("Failed to execute the command: {}", err))?;

    // Check if the command executed successfully
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).map_err(|err| format!("Failed to convert stdout to UTF-8: {}", err))?;
        Ok(stdout)
    } else {
        let stderr = String::from_utf8(output.stderr).map_err(|err| format!("Failed to convert stderr to UTF-8: {}", err))?;
        Err(stderr)
    }
}