use super::tokenizer::{OperationKind, TokenKind, TokenList};
use crate::error::error_exit;

pub fn expect_number(token_list: &mut TokenList) -> i32 {
    let token = token_list.pop_head();
    match token {
        Some(valid_token) => match valid_token.token_kind {
            TokenKind::Number(num) => {
                return num;
            }
            _ => {
                error_exit(
                    "expect number token",
                    valid_token.token_pos,
                    &token_list.raw_text,
                );
            }
        },
        // Noneの場合はトークン終了を意味する
        None => {
            // テキスト終端の1つ後ろに要求エラーを立てる
            let tail_pos = token_list.raw_text.chars().count();
            error_exit("expect number token", tail_pos, &token_list.raw_text);
        }
    }
}

pub fn expect_operation(token_list: &mut TokenList) -> OperationKind {
    let token = token_list.pop_head();
    match token {
        Some(valid_token) => match valid_token.token_kind {
            TokenKind::Operation(op) => {
                return op;
            }
            _ => {
                error_exit(
                    "expect operation token",
                    valid_token.token_pos,
                    &token_list.raw_text,
                );
            }
        },
        // Noneの場合はトークン終了を意味する
        None => {
            // テキスト終端に要求エラーを立てる
            let tail_pos = token_list.raw_text.chars().count();
            error_exit("expect operation token", tail_pos, &token_list.raw_text);
        }
    }
}

pub fn compile_operation(token_list: &mut TokenList) -> String {
    let mut instruction = "    ".to_string();
    match expect_operation(token_list) {
        OperationKind::Add => {
            instruction += "add rax,";
        }
        OperationKind::Sub => {
            instruction += "sub rax,";
        }
        _ => {},
    }
    let num = expect_number(token_list);
    instruction = format!("{} {}", instruction, num);
    instruction
}
