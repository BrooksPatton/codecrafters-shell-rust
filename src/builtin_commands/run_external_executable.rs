use std::fs::DirEntry;

pub fn run_external_executable(executable: DirEntry, arguments: &[String]) {
    let path = executable.path();
    let mut command = std::process::Command::new(path);

    command.args(arguments);

    let Ok(mut process_child) = command.spawn() else {
        return;
    };
    let Ok(_result) = process_child.wait() else {
        return;
    };
}
