use std::collections::VecDeque;

#[derive(Debug,PartialEq,Eq)] 
pub enum TokenKind {
    Number(i32),
    Add,
    Sub,
    InvalidToken,
}

#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    next: Option<Box<Token>>,
}

impl Token {
    fn new() -> Token {
        Token{
            token_kind: TokenKind::InvalidToken,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct TokenList {
    pub head: Option<Box<Token>>,
}

impl TokenList {
    fn new() -> TokenList {
        TokenList {
            head: None,
        }
    }

    pub fn pop_head(&mut self) -> Option<Box<Token>> {
        if let Some(mut token) = self.head.take() {
            self.head= token.next.take();
            return Some(token);
        } else {
            return None;
        }
    }
}

pub fn text_tokenizer(text: &str) -> TokenList {
    let mut char_queue = text.chars().collect::<VecDeque<char>>();
    let mut tokenlist = TokenList::new();
    let mut current_token = &mut tokenlist.head;
    
    while !char_queue.is_empty() {
        let ch = char_queue.pop_front().unwrap();
        if ch == ' ' {
            continue;
        }

        let mut new_token = Token::new();

        if ch == '+' {
            new_token.token_kind = TokenKind::Add;
        }

        if ch == '-' {
            new_token.token_kind = TokenKind::Sub;
        }
        
        if ch.is_digit(10) {
            let mut num: i32 = ch.to_digit(10).unwrap() as i32;
            loop {
                let next = char_queue.front();
                if let Some(digit) = next {
                    if digit.is_digit(10) {
                        let next_digit = char_queue.pop_front().unwrap();
                        num = num*10 + next_digit.to_digit(10).unwrap() as i32;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            new_token.token_kind = TokenKind::Number(num);
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