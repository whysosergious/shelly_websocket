use tokio::process::Command;

/// spawn a process to execute shell command
pub async fn execute_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: binary input/uotput
    // TODO: look into keepalive for e.g. sqeel operations

    let output = Command::new("nu").arg("-c").arg(command).output().await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Nushell error: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}
