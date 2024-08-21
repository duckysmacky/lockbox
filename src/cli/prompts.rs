use std::io;
use crate::log_fatal;

// TODO: hide user input
pub fn prompt_password(msg: &str) -> String {
    println!("{}", msg);
    get_input().unwrap_or_else(|err|log_fatal!("Error prompting password: {}", err))
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let input = buffer.trim_end().to_string();
    Ok(input)
}