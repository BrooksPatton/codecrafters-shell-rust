use std::{
    collections::VecDeque,
    fs,
    io::{BufRead, BufReader, Write},
    path::Path,
    usize,
};

use crate::{
    command::{Command, CommandIO},
    errors::ErrorExitCode,
};

#[derive(Debug)]
pub struct History {
    pub commands: Vec<String>,
    lookback_index: usize,
}

impl History {
    pub fn new() -> Self {
        let commands = vec![];
        let lookback_index = 0;

        Self {
            commands,
            lookback_index,
        }
    }

    pub fn add(&mut self, command: &Command) {
        let history_item = command.builtin_command.to_string().trim().to_owned();

        self.commands.push(history_item);
    }

    fn print(&self, mut command_io: CommandIO) -> Result<(), ErrorExitCode> {
        for (index, command) in self.commands.iter().enumerate() {
            let history_number = index + 1;

            writeln!(command_io.stdout, "\t{history_number}  {command}")?;
        }

        Ok(())
    }

    fn print_n(&self, mut command_io: CommandIO, count: usize) -> Result<(), ErrorExitCode> {
        for (index, command) in self
            .commands
            .iter()
            .enumerate()
            .skip(self.commands.len() - count)
        {
            let history_number = index + 1;

            writeln!(command_io.stdout, "\t{history_number}  {command}")?;
        }

        Ok(())
    }

    pub fn get_previous_prompt(&mut self) -> Option<&str> {
        self.lookback_index += 1;
        let index = self.commands.len().checked_sub(self.lookback_index)?;

        self.commands
            .get(index)
            .map(|command_prompt| command_prompt.as_str())
    }

    pub fn get_next_prompt(&mut self) -> Option<&str> {
        self.lookback_index = self.lookback_index.checked_sub(1)?;
        let index = self.commands.len().checked_sub(self.lookback_index)?;

        self.commands
            .get(index)
            .map(|command_prompt| command_prompt.as_str())
    }

    pub fn reset_lookback(&mut self) {
        self.lookback_index = 0;
    }

    pub fn controller(
        &mut self,
        mut command_io: CommandIO,
        mut arguments: VecDeque<String>,
    ) -> Result<(), ErrorExitCode> {
        let Some(first_argument) = arguments.pop_front() else {
            return self.print(command_io);
        };

        if let Some(count) = first_argument.as_str().parse::<usize>().ok() {
            return self.print_n(command_io, count);
        }

        match first_argument.as_str() {
            "-r" => {
                let Some(filename) = arguments.pop_front() else {
                    writeln!(command_io.stderr, "Error, history -r requires a path")?;
                    return Err(ErrorExitCode::new_const::<1>());
                };

                self.load_history_from_file(filename, command_io)
            }
            "-w" => {
                let Some(filename) = arguments.pop_front() else {
                    writeln!(command_io.stderr, "Error: history -w requires a path")?;
                    return Err(ErrorExitCode::new_const::<9>());
                };

                self.write_history_to_file(command_io, filename)
            }
            _ => {
                writeln!(command_io.stderr, "Error: Unkown option {first_argument}")?;
                writeln!(command_io.stderr, "history [count]|[-r path]")?;
                Err(ErrorExitCode::new_const::<5>())
            }
        }
    }

    fn load_history_from_file(
        &mut self,
        filename: String,
        mut command_io: CommandIO,
    ) -> Result<(), ErrorExitCode> {
        let path = Path::new(&filename);

        if !path.is_file() {
            writeln!(command_io.stderr, "Error: {filename} is not a valid file")?;
            return Err(ErrorExitCode::new_const::<2>());
        }

        let history_file = match fs::File::options().read(true).open(path) {
            Ok(file) => file,
            Err(error) => {
                writeln!(command_io.stderr, "{error:?}")?;
                return Err(ErrorExitCode::new_const::<3>());
            }
        };
        let history_file_reader = BufReader::new(history_file);

        for history_command in history_file_reader.lines() {
            match history_command {
                Ok(command) => self.commands.push(command),
                Err(error) => {
                    writeln!(command_io.stderr, "{error:?}")?;
                    return Err(ErrorExitCode::new_const::<4>());
                }
            }
        }

        Ok(())
    }

    fn write_history_to_file(
        &self,
        mut command_io: CommandIO,
        filename: String,
    ) -> Result<(), ErrorExitCode> {
        let path = Path::new(&filename);

        if path.is_dir() {
            writeln!(
                command_io.stderr,
                "Error: When writing history to file, given filename must not be a directory."
            )?;
            return Err(ErrorExitCode::new_const::<6>());
        }

        let mut file = match fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
        {
            Ok(file) => file,
            Err(error) => {
                writeln!(command_io.stderr, "{error:?}")?;
                return Err(ErrorExitCode::new_const::<7>());
            }
        };

        for command in self.commands.iter() {
            writeln!(file, "{command}")?;
        }

        Ok(())
    }
}
