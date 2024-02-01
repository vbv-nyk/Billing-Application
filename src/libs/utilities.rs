use std::io;

pub fn get_user_string_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut buffer = String::from("");
    loop {
        let read_result = io::stdin().read_line(&mut buffer);
        if read_result.is_ok() {
            return buffer.trim().to_owned();
        } else {
            println!("Error Accepting Input, try again!");
        }
    }
}

pub fn get_user_number_input(prompt: &str) -> u32 {
    loop {
        let input = get_user_string_input(prompt);
        if let Ok(number) = input.trim().parse::<u32>() {
            return number;
        } else {
            println!("Entered amount is not an integer");
        }
    }
}

pub fn convert_to_usize(number: u32) -> usize {
    number as usize
}
