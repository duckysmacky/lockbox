use std::io;

// TODO: hide user input
pub fn prompt_password(msg: &str) -> String {
    println!("{}", msg);
    get_input().unwrap_or_else(|err| panic!("Unable to prompt password: {}", err)).to_string()
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let input = buffer.trim_end().to_string();
    Ok(input)
}