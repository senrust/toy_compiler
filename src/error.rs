use std::process::exit;

pub fn error_exit(error_text: &str, space_count: usize, input_text: &str) -> ! {
    println!("{}", input_text);
    let error_space = " ".repeat(space_count);
    let error_string = format!("{}^ {}", error_space, error_text);
    println!("{}", error_string);
    exit(1);
}