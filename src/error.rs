use crate::tokenizer::{PROGRAM_TEXT, TokenList};
use std::process::exit;

// エラー行に全角表示文字があるとその文ずれる
// (スペースは半角のため)
// utfコードから全角判断してスペースを余分に追加すれば対応できるはず
pub fn invalid_token_exit(error_text: &str, token_list: &TokenList) -> ! {
    if let Some(token) = token_list.peek_head(){
        error_exit(error_text, token.token_pos);
    } else {
        let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
        error_exit(error_text, tail_pos);
    }
}


// エラー行に全角表示文字があるとその文ずれる
// (スペースは半角のため)
// utfコードから全角判断してスペースを余分に追加すれば対応できるはず
pub fn error_exit(error_text: &str, error_pos: usize) -> ! {
    let (error_line_string, error_line, error_column) =
        PROGRAM_TEXT.get().unwrap().get_error_line(error_pos);
    let line_string = format!("line{}: ", error_line);
    let error_space = " ".repeat(line_string.len() + error_column);
    println!("{}{}", line_string, error_line_string);
    let error_string = format!("{}^{}", error_space, error_text);
    println!("{}", error_string);
    exit(1);
}
