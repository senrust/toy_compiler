use std::{collections::VecDeque, iter::FromIterator};

use super::compiler::*;

#[derive(Debug,PartialEq,Eq)] 
pub enum OperationKind {
    Add,
    Sub
}

#[derive(Debug,PartialEq,Eq)] 
pub enum TokenKind {
    Number(i32),
    Operation(OperationKind),
    InvalidToken,
}

#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub token_pos: usize,
    next: Option<Box<Token>>,
}

impl Token {
    fn new(token_pos: usize) -> Token {
        Token{
            token_kind: TokenKind::InvalidToken,
            token_pos,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct TokenList {
    pub head: Option<Box<Token>>,
    pub raw_text: String,
}

impl TokenList {
    fn new(text: &str) -> TokenList {
        TokenList {
            raw_text: String::from(text),
            head: None,
        }
    }

    pub fn pop_head(&mut self) -> Option<Box<Token>> {
        if let Some(mut token) = self.head.take() {
            self.head = token.next.take();
            return Some(token);
        } else {
            return None;
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.head.is_none()
    }
}

impl Drop for TokenList {
    fn drop(&mut self) {
        let mut token = self.head.take();
        loop {
            match token {
                Some(mut valid_token) => {
                    let next_token = valid_token.next.take();
                    token = next_token;
                },
                None => {break;},
            }
        }
    }
}

// 入力テキストのトークン連結リストを作成する
pub fn text_tokenizer(text: &str) -> TokenList {
    let mut char_queue = VecDeque::from_iter(text.chars());
    let mut tokenlist = TokenList::new(text);
    let mut current_token = &mut tokenlist.head;
    let mut token_pos = 0;
    
    while !char_queue.is_empty() {
        let mut new_token = Token::new(token_pos);
        token_pos += 1;
        let ch = char_queue.pop_front().unwrap();

        if ch == ' ' {
            continue;
        }

        if ch == '+' {
            new_token.token_kind = TokenKind::Operation(OperationKind::Add);
        } 
        
        if ch == '-' {
            new_token.token_kind = TokenKind::Operation(OperationKind::Sub);
        }
        
        if ch.is_digit(10) {
            let mut num: i32 = ch.to_digit(10).unwrap() as i32;
            loop {
                let next = char_queue.front();
                if let Some(digit) = next {
                    if digit.is_digit(10) {
                        let next_digit = char_queue.pop_front().unwrap();
                        num = num*10 + next_digit.to_digit(10).unwrap() as i32;
                        token_pos += 1; 
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            new_token.token_kind = TokenKind::Number(num);
        }

        if new_token.token_kind == TokenKind::InvalidToken {
            error_exit("unsupported token", new_token.token_pos, &text)
        }
        
       
        match current_token {
            Some(token) => {
                token.next = Some(Box::new(new_token));
                current_token = &mut token.next;
            }
            None => {
                *current_token = Some(Box::new(new_token)); 
            }
        }
    }
    tokenlist
}