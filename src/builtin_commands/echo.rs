use std::fmt::Display;

pub fn echo(user_input: &[impl Display]) {
    let mut inputs = user_input.iter();
    if let Some(input) = inputs.next() {
        print!("{input}");
    }

    for input in inputs {
        print!(" {input}");
    }

    println!("");
}
