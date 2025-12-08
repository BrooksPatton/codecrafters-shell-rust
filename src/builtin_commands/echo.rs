use std::{
    fmt::Display,
    io::{PipeWriter, Write},
    sync::mpsc::Sender,
};

pub fn echo(user_input: &[impl Display], output: &mut Sender<String>) {
    let mut inputs = user_input.iter();
    let mut buffer = vec![];
    if let Some(input) = inputs.next() {
        // print!("{input}");
        write!(&mut buffer, "{input}");
    }

    for input in inputs {
        // print!(" {input}");
        write!(buffer, " {input}");
    }

    output.send(String::from_utf8(buffer).unwrap()).unwrap();
}
