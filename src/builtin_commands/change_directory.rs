use anyhow::{Context, Result, bail};
use std::{
    env::{home_dir, set_current_dir},
    path::Path,
    sync::mpsc::Sender,
};

pub fn change_directory(arguments: &[String], standard_error: &mut Sender<String>) -> Result<()> {
    let Some(home_directory) = home_dir() else {
        bail!("Error: you don't seem to have a home directory");
    };
    let Some(target_path) = arguments.first() else {
        std::env::set_current_dir(home_directory).context("changing to home directory")?;
        return Ok(());
    };
    let target_path = Path::new(&target_path);

    if target_path.is_dir() {
        set_current_dir(target_path).context("Changing to target directory")?;
    } else {
        standard_error
            .send(format!(
                "cd: {}: No such file or directory",
                target_path.to_str().unwrap_or_default()
            ))
            .context("Sending error for changing home directory")?;
    }

    Ok(())
}
