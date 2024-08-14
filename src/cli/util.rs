use std::io;

pub fn prompt_input() -> io::Result<String> {
    println!("Please enter a password: ");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let input = buffer.trim_end().to_string();
    Ok(input)
}