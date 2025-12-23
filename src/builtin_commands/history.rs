use std::{io::Write, usize};

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

    pub fn print(
        &self,
        mut command_io: CommandIO,
        arguments: Vec<String>,
    ) -> Result<(), ErrorExitCode> {
        let how_many_to_show = if let Some(count) = arguments.first() {
            match count.parse::<usize>() {
                Ok(count) => count,
                Err(error) => {
                    writeln!(command_io.stderr, "{error:?}")?;
                    return Err(ErrorExitCode::new_const::<1>());
                }
            }
        } else {
            self.commands.len()
        };

        for (index, command) in self
            .commands
            .iter()
            .enumerate()
            .skip(self.commands.len() - how_many_to_show)
        {
            let history_number = index + 1;

            writeln!(command_io.stdout, "\t{history_number}  {command}")?;
        }

        Ok(())
    }
}
