use anyhow::{Context, Result, bail};
use std::{
    env::{home_dir, set_current_dir},
    path::Path,
};

pub fn change_directory(arguments: &[String]) -> Result<()> {
    let Some(target_path) = arguments.first() else {
        let Some(home_directory) = home_dir() else {
            bail!("Error: you don't seem to have a home directory");
        };

        std::env::set_current_dir(home_directory).context("changing to home directory")?;
        return Ok(());
    };

    let target_path = Path::new(target_path);

    if target_path.is_dir() {
        set_current_dir(target_path).context("Changing to target directory")?;
    }

    Ok(())
}
