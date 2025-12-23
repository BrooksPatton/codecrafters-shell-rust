use std::io::Write;

use crate::{
    command::{Command, CommandIO},
    errors::ErrorExitCode,
};

#[derive(Debug)]
pub struct History {
    pub commands: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        let commands = vec![];

        Self { commands }
    }

    pub fn add(&mut self, command: &Command) {
        let history_item = command.builtin_command.to_string();

        self.commands.push(history_item);
    }

    pub fn print(&self, mut command_io: CommandIO) -> Result<(), ErrorExitCode> {
        for (index, command) in self.commands.iter().enumerate() {
            let history_number = index + 1;

            writeln!(command_io.stdout, "\t{history_number}  {command}")?;
        }

        Ok(())
    }
}
