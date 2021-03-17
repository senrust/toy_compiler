use crate::error::error_exit;
use std::{collections::VecDeque, iter::FromIterator};

#[derive(Debug, PartialEq, Eq)]
pub enum OperationKind {
    Add,
    Sub,
    Mul,
    Div,    
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParenthesesKind {
    LeftParentheses,
    RightParentheses,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Number(i32),
    Operation(OperationKind),
    Parentheses(ParenthesesKind),
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
        Token {
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

    pub fn peek_head(&self) -> &Option<Box<Token>> {
        &self.head
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

    pub fn consume_operation(&mut self, op: OperationKind) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Operation(op) {
                    self.pop_head();
                    return true;
                } else {
                    return false;
                }
            },
            None => {
                return false;
            }
        }
    }

    pub fn comsume_parentheses(&mut self, parenthese: ParenthesesKind)  -> bool{
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Parentheses(parenthese) {
                    self.pop_head();
                    return true;
                } else {
                    return false;
                }
            },
            None => {
                return false;
            }
        }
    }

    pub fn expect_number(&mut self) -> i32 {
        let token = self.pop_head();
        match token {
            Some(valid_token) => {
                match valid_token.token_kind {
                    TokenKind::Number(num) => {
                        return num;
                    }
                    _ => {
                        error_exit(
                            "expect number token",
                            valid_token.token_pos,
                            &self.raw_text,
                        );
                    }
                }
            },
            // Noneの場合はトークン終了を意味する
            None => {
                // テキスト終端の1つ後ろに要求エラーを立てる
                let tail_pos = self.raw_text.chars().count();
                error_exit("expect number token", tail_pos, &self.raw_text);
            }
        }
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
                }
                None => {
                    break;
                }
            }
        }
    }
}

// 入力テキストのトークン連結リストを作成する
pub fn text_tokenizer(text: &str) -> TokenList {
    // veqdeque よりはpeekableなイテレータで良いかも
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

        if ch == '*' {
            new_token.token_kind = TokenKind::Operation(OperationKind::Mul);
        }

        if ch == '/' {
            new_token.token_kind = TokenKind::Operation(OperationKind::Div);
        }

        if ch == '(' {
            new_token.token_kind = TokenKind::Parentheses(ParenthesesKind::LeftParentheses);
        }

        if ch == ')' {
            new_token.token_kind = TokenKind::Parentheses(ParenthesesKind::RightParentheses);
        }

        if ch.is_digit(10) {
            let mut num: i32 = ch.to_digit(10).unwrap() as i32;
            loop {
                let next = char_queue.front();
                if let Some(next_ch) = next {
                    if next_ch.is_digit(10) {
                        let next_digit = char_queue.pop_front().unwrap().to_digit(10).unwrap() as i32;
                        num = num * 10 + next_digit;
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
