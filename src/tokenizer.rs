use crate::error::error_exit;
use std::{collections::VecDeque, iter::FromIterator};

#[derive(Debug, PartialEq, Eq)]
pub enum OperationKind {
    Gt,
    Ge,
    Eq,
    Not,
    Le,
    Lt,
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

fn pop_digit(char_queue: &mut VecDeque<char>) -> i32 {
    let mut num = char_queue.pop_front().unwrap().to_digit(10).unwrap() as i32;
    loop {
        let next = char_queue.front();
        if let Some(next_ch) = next {
            if next_ch.is_digit(10) {
                let next_digit = char_queue.pop_front().unwrap().to_digit(10).unwrap() as i32;
                num = num * 10 + next_digit;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    num
}

fn pop_ascii(char_queue: &mut VecDeque<char>) -> TokenKind {
    let ch = char_queue.pop_front().unwrap();
    let mut op_string = ch.to_string();
    // <、<=、>、>=、==、!= に対応するため, 次が=ならばそれも取り出す
    if let Some(next_ch_ref) = char_queue.front() {
        if *next_ch_ref == '=' {
            let next_ch = char_queue.pop_front().unwrap();
            op_string.push(next_ch);
        }
    }

    if op_string == ">" {
        TokenKind::Operation(OperationKind::Gt)
    } else if op_string == ">=" { 
        TokenKind::Operation(OperationKind::Ge)
    } else if op_string == "==" { 
        TokenKind::Operation(OperationKind::Eq)
    } else if op_string == "!=" { 
        TokenKind::Operation(OperationKind::Not)
    } else if op_string == "<=" { 
        TokenKind::Operation(OperationKind::Le)
    } else if op_string == "<" { 
        TokenKind::Operation(OperationKind::Lt)
    } else if op_string == "+" { 
        TokenKind::Operation(OperationKind::Add)
    } else if op_string == "-" { 
        TokenKind::Operation(OperationKind::Sub)
    } else if op_string == "*" { 
        TokenKind::Operation(OperationKind::Mul)
    } else if op_string == "/" { 
        TokenKind::Operation(OperationKind::Div)
    } else if op_string == "(" { 
        TokenKind::Parentheses(ParenthesesKind::LeftParentheses)
    } else if op_string == ")" { 
        TokenKind::Parentheses(ParenthesesKind::RightParentheses)
    } else {
        TokenKind::InvalidToken
    }
}

// 入力テキストのトークン連結リストを作成する
pub fn text_tokenizer(text: &str) -> TokenList {
    // veqdeque よりはpeekableなイテレータで良いかも
    let mut char_queue = VecDeque::from_iter(text.chars());
    let mut tokenlist = TokenList::new(text);
    let mut current_token = &mut tokenlist.head;
    let text_len = char_queue.len();

    while !char_queue.is_empty() {
        // 解析トークンの位置はテキストの長さ-未処理文字数で求まる
        let token_pos = text_len - char_queue.len();
        let mut new_token = Token::new(token_pos);
        let ch = char_queue.front().unwrap();

        if *ch == ' ' {
            char_queue.pop_front();
            continue;
        }
       
        if ch.is_digit(10) {
            let num = pop_digit(&mut char_queue);
            new_token.token_kind = TokenKind::Number(num);
        } else {
            new_token.token_kind = pop_ascii(&mut char_queue);
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
