use std::process::exit;
use super::tokenizer::*;

pub fn error_exit(error_text: &str) -> ! {
    println!("{}", error_text);
    exit(1);
}

pub fn expect_number(token_list: &mut TokenList) -> i32 {
    let token = token_list.pop_head();
    match token {
        Some(valid_token) => {
            match valid_token.token_kind {
                TokenKind::Number(num) => {return num;},
                _ => {error_exit("expect number token");}
            }
        },
        None => {
            error_exit("expect number token");
        },
    }
}

pub fn expect_operation(token_list: &mut TokenList) -> TokenKind {
    let token = token_list.pop_head();
    match token {
        Some(valid_token) => {
            match valid_token.token_kind {
                TokenKind::Add => {return TokenKind::Add;},
                TokenKind::Sub => {return TokenKind::Sub;},
                _ => {error_exit("expect operation token");}
            }
        },
        None => {
            error_exit("expect operation token");
        },
    }
}