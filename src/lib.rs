mod errors;
mod utilities;

#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::Result;

use crate::{
    errors::CustomError,
    utilities::{get_user_input, print_error},
};

pub fn run() -> Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let user_input = get_user_input()?;
        let error = CustomError::CommandNotFound(user_input);

        print_error(error);
    }

    #[allow(unreachable_code)]
    Ok(())
}
