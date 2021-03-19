use std::process::exit;
use crate::tokenizer::PROGRAM_TEXT;

pub fn error_exit(error_text: &str, error_pos: usize) -> ! {
    let (error_line_string, error_line, error_column) = PROGRAM_TEXT.get().unwrap().get_error_line(error_pos);
    let line_string = format!("line{}: ", error_line);
    let error_space = " ".repeat(line_string.len() + error_column);
    println!("{}{}", line_string, error_line_string);
    let error_string = format!("{}^{}", error_space, error_text);
    println!("{}", error_string);
    exit(1);
}
