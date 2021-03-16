use std::process::exit;
use super::tokenizer::*;

pub fn error_exit(error_text: &str, space_count : usize, input_text: &str) -> ! {
    println!("{}", input_text);
    let error_space = " ".repeat(space_count);
    let error_string = format!("{}^ {}", error_space, error_text);
    println!("{}", error_string);
    exit(1);
}

pub fn expect_number(token_list: &mut TokenList) -> i32 {
    let token = token_list.pop_head();
    match token {
        Some(valid_token) => {
            match valid_token.token_kind {
                TokenKind::Number(num) => {return num;},
                _ => {error_exit("expect number token", valid_token.token_pos, &token_list.raw_text);}
            }
        },
        // Noneの場合はトークン終了を意味する
        None => {
            // テキスト終端の1つ後ろに要求エラーを立てる
            let tail_pos = token_list.raw_text.chars().count() + 1;
            error_exit("expect number token", tail_pos, &token_list.raw_text);
        },
    }
}

pub fn expect_operation(token_list: &mut TokenList) -> OperationKind {
    let token = token_list.pop_head();
    match token {
        Some(valid_token) => {
            match valid_token.token_kind {
                TokenKind::Operation(op) => {return op;},
                _ => {error_exit("expect operation token",  valid_token.token_pos, &token_list.raw_text);}
            }
        },
        // Noneの場合はトークン終了を意味する
        None => {
            // テキスト終端の1つ後ろに要求エラーを立てる
            let tail_pos = token_list.raw_text.chars().count() + 1;
            error_exit("expect operation token", tail_pos, &token_list.raw_text);
        },
    }
}

pub fn compile_operation(token_list: &mut TokenList) -> String {
    let mut instruction = "    ".to_string();
    match expect_operation(token_list) {
        OperationKind::Add => {instruction += "add rax,";}
        OperationKind::Sub => {instruction += "sub rax,";}
    }
    let num = expect_number(token_list);
    instruction = format!("{} {}", instruction, num);
    instruction
}