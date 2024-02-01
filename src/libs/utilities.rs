use std::io;

pub fn get_user_input(prompt: &str) -> Result<String, io::Error> {
    println!("{prompt}");
    let mut buffer = String::from("");
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
