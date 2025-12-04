use std::path::Path;

use crate::builtin_commands::echo::echo;

pub fn pwd(path: &Path) {
    let stringified_path = path.as_os_str().to_str().unwrap_or_default();

    echo(&[stringified_path]);
}
