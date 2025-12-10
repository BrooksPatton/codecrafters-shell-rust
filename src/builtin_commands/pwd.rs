use crate::builtin_commands::echo::echo;
use anyhow::{Context, Result};

pub fn pwd(stdout: &mut Vec<String>, stderr: &mut Vec<String>) -> Result<()> {
    let path = std::env::current_dir().context("Getting current directory")?;
    let stringified_path = path.as_os_str().to_str().unwrap_or_default();

    echo(&[stringified_path], stdout, stderr)?;

    Ok(())
}
