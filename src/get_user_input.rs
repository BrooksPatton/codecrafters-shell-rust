use std::io::Write;

use anyhow::Result;
use console::{Key, Term};

use crate::{
    builtin_commands::BuiltinCommand,
    utilities::{find_executable_files, get_path},
};

pub struct UserInput {
    ps1: &'static str,
    term: Term,
}

impl UserInput {
    pub fn new(ps1: &'static str) -> Self {
        let term = Term::stdout();

        Self { ps1, term }
    }

    pub fn readline(&self) -> Result<String> {
        let mut in_command = true;
        let mut user_input = String::new();

        self.print_prompt()?;

        loop {
            let key_code = self.term.read_key()?;

            match key_code {
                Key::Unknown => todo!(),
                Key::UnknownEscSeq(_items) => todo!(),
                Key::ArrowLeft => todo!(),
                Key::ArrowRight => todo!(),
                Key::ArrowUp => todo!(),
                Key::ArrowDown => todo!(),
                Key::Enter => {
                    self.term.write_line("")?;
                    return Ok(user_input);
                }
                Key::Backspace => {
                    self.term.clear_chars(1)?;
                    user_input.pop();
                }
                Key::Home => todo!(),
                Key::End => todo!(),
                Key::Tab => {
                    if !in_command {
                        continue;
                    }

                    if let Some(completed_command) = self.autocomplete_one_builtin(&user_input) {
                        user_input = format!("{completed_command} ");
                        in_command = false;
                        self.rewrite_line(&user_input)?;
                        continue;
                    }

                    // if no possible matching builtins or executables ring the bell
                    let possible_commands = self.get_possible_commands(&user_input)?;

                    if possible_commands.is_empty() {
                        self.print_bell()?;
                        continue;
                    }

                    if possible_commands.len() == 1 {
                        user_input = format!("{} ", possible_commands[0]);
                        in_command = false;
                        self.rewrite_line(&user_input)?;
                        continue;
                    }
                }
                Key::BackTab => todo!(),
                Key::Alt => todo!(),
                Key::Del => todo!(),
                Key::Shift => todo!(),
                Key::Insert => todo!(),
                Key::PageUp => todo!(),
                Key::PageDown => todo!(),
                Key::Char(character) => {
                    write!(&self.term, "{character}")?;
                    user_input.push(character);

                    if character == ' ' {
                        in_command = false;
                    }
                }
                Key::CtrlC => todo!(),
                _ => (),
            }

            self.term.flush()?;
        }
    }

    fn print_prompt(&self) -> Result<()> {
        write!(&self.term, "{}", self.ps1)?;

        Ok(())
    }

    fn autocomplete_one_builtin(&self, command: &str) -> Option<String> {
        let matching_builtins = BuiltinCommand::matches(command);

        if matching_builtins.len() == 1 {
            matching_builtins.first().cloned()
        } else {
            None
        }
    }

    fn rewrite_line(&self, user_input: &str) -> Result<()> {
        self.term.clear_line()?;
        self.print_prompt()?;
        write!(&self.term, "{user_input}")?;

        Ok(())
    }

    fn get_possible_commands(&self, user_input: &str) -> Result<Vec<String>> {
        let matching_builtins = BuiltinCommand::matches(user_input);

        if matching_builtins.len() > 0 {
            return Ok(matching_builtins);
        }

        let path = get_path()?;
        let external_executables = find_executable_files(user_input, &path, true)?
            .into_iter()
            .filter_map(|dir_entry| Some(dir_entry.file_name().to_str()?.to_owned()))
            .collect::<Vec<String>>();

        Ok(external_executables)
    }

    fn print_bell(&self) -> Result<()> {
        write!(&self.term, "\x07")?;
        self.term.flush()?;

        Ok(())
    }
}
