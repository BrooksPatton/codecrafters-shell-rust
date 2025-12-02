pub fn echo(user_input: &[String]) {
    let message = user_input.join(" ");
    println!("{message}");
}
