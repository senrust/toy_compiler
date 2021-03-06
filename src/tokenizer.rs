use crate::error::error_exit;
use std::{collections::VecDeque, iter::FromIterator};

use crate::ast::PrimaryNodeKind;

use once_cell::sync::OnceCell;

pub struct ProgramText {
    pub text: Vec<char>,
}

impl ProgramText {
    fn new(text: Vec<char>) -> Self {
        ProgramText { text }
    }

    pub fn get_tail_pos(&self) -> usize {
        self.text.len() - 1
    }

    pub fn get_error_line(&self, error_pos: usize) -> (String, usize, usize) {
        let mut error_text = format!("");
        let mut pos = error_pos;

        // エラー発生行の終端位置を取得
        while self.text[pos] != '\n' {
            // エラー発生業が最終行の場合
            if pos != self.text.len() - 1 {
                pos += 1;
            } else {
                break;
            }
        }
        // posはエラー発生行の改行を指しているので, 1つ前に戻す
        pos -= 1;
        // エラー発生行の文字列を取得
        while self.text[pos] != '\n' {
            error_text = format!("{}{}", self.text[pos], error_text);

            if pos != 0 {
                pos -= 1;
            } else {
                break;
            }
        }
        // posはエラー発生行の改行を指しているので, 1つ後ろに戻す
        // ただし先頭行の場合はposが0を指している
        if pos != 0 {
            pos += 1;
        }

        let mut curpos = 0;
        let mut line = 1;
        while curpos != pos {
            if self.text[curpos] == '\n' {
                line += 1;
            }
            curpos += 1;
        }
        (error_text, line, error_pos - pos)
    }
}

pub static PROGRAM_TEXT: OnceCell<ProgramText> = OnceCell::new();

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
pub enum BracesKind {
    LeftBraces,
    RightBraces,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Number(i32),
    Operation(OperationKind),
    Parentheses(ParenthesesKind),
    Braces(BracesKind),
    Comma,
    LocalVariableDefinition(usize),
    LocalVariable(usize),
    FucntionDefinition(String),
    FucntionCall(String),
    Assign,
    Return,
    While,
    If,
    Else,
    For,
    StateMentEnd,
    Reference,
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
    pub local_stack_size: usize,
}

impl TokenList {
    fn new() -> TokenList {
        TokenList {
            head: None,
            local_stack_size: 0,
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

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn is_operation(&mut self, op: OperationKind) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Operation(op) {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
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
            }
            None => {
                return false;
            }
        }
    }

    pub fn is_reference(&mut self) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Reference {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn consume_commma(&mut self) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Comma {
                    self.pop_head();
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn consume_function_definition(&mut self) -> Option<String> {
        match self.peek_head() {
            Some(token) => {
                match token.token_kind {
                    TokenKind::FucntionDefinition(_) => {
                        // 所有権を取り出し
                        if let TokenKind::FucntionDefinition(function_name) =
                            self.pop_head().unwrap().token_kind
                        {
                            return Some(function_name);
                        }
                        return None;
                    }
                    _ => {
                        return None;
                    }
                }
            }
            None => {
                return None;
            }
        }
    }

    pub fn consume_functioncall(&mut self) -> Option<String> {
        match self.peek_head() {
            Some(token) => {
                match token.token_kind {
                    TokenKind::FucntionCall(_) => {
                        // 所有権を取り出し
                        if let TokenKind::FucntionCall(function_name) =
                            self.pop_head().unwrap().token_kind
                        {
                            return Some(function_name);
                        }
                        return None;
                    }
                    _ => {
                        return None;
                    }
                }
            }
            None => {
                return None;
            }
        }
    }

    pub fn is_parentheses(&self, parenthese: ParenthesesKind) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Parentheses(parenthese) {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn comsume_parentheses(&mut self, parenthese: ParenthesesKind) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Parentheses(parenthese) {
                    self.pop_head();
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn comsume_braces(&mut self, braces: BracesKind) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Braces(braces) {
                    self.pop_head();
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn is_assign(&self) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Assign {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn consume_return(&mut self) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::Return {
                    self.pop_head();
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    // if文かチェック
    pub fn consume_if(&mut self) -> bool {
        if let Some(first_token) = self.peek_head() {
            if first_token.token_kind == TokenKind::If {
                self.pop_head();
                return true;
            }
        }
        false
    }

    // else文かチェック
    pub fn consume_else(&mut self) -> bool {
        if let Some(first_token) = self.peek_head() {
            if first_token.token_kind == TokenKind::Else {
                self.pop_head();
                return true;
            }
        }
        false
    }

    // while文かチェック
    pub fn consume_while(&mut self) -> bool {
        if let Some(first_token) = self.peek_head() {
            if first_token.token_kind == TokenKind::While {
                self.pop_head();
                return true;
            }
        }
        false
    }

    // for文かチェック
    pub fn consume_for(&mut self) -> bool {
        if let Some(first_token) = self.peek_head() {
            if first_token.token_kind == TokenKind::For {
                self.pop_head();
                return true;
            }
        }
        false
    }

    pub fn is_statement_end(&self) -> bool {
        match self.peek_head() {
            Some(token) => {
                if token.token_kind == TokenKind::StateMentEnd {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn consume_statement_end(&mut self) -> bool {
        let token = self.pop_head();
        let error_text = "expect ; at end of statement";
        match token {
            Some(valid_token) => match valid_token.token_kind {
                TokenKind::StateMentEnd => {
                    return true;
                }
                _ => {
                    error_exit(error_text, valid_token.token_pos);
                }
            },
            // Noneの場合はトークン終了を意味する
            None => {
                // テキスト終端の1つ後ろに要求エラーを立てる
                let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                error_exit(error_text, tail_pos);
            }
        }
    }

    pub fn expect_primary(&mut self) -> PrimaryNodeKind {
        let token = self.pop_head();
        let error_text = "expect number or variable token";
        match token {
            Some(valid_token) => match valid_token.token_kind {
                TokenKind::Number(num) => {
                    return PrimaryNodeKind::Number(num);
                }
                TokenKind::LocalVariable(offset) => {
                    return PrimaryNodeKind::LocalVariable(offset);
                }
                _ => {
                    error_exit(error_text, valid_token.token_pos);
                }
            },
            // Noneの場合はトークン終了を意味する
            None => {
                // テキスト終端の1つ後ろに要求エラーを立てる
                let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                error_exit(error_text, tail_pos);
            }
        }
    }

    pub fn expect_variable_definition(&mut self) -> PrimaryNodeKind {
        let token = self.pop_head();
        let error_text = "expect variable token";
        match token {
            Some(valid_token) => match valid_token.token_kind {
                TokenKind::LocalVariableDefinition(offset) => {
                    return PrimaryNodeKind::LocalVariable(offset);
                }
                _ => {
                    error_exit(error_text, valid_token.token_pos);
                }
            },
            // Noneの場合はトークン終了を意味する
            None => {
                // テキスト終端の1つ後ろに要求エラーを立てる
                let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                error_exit(error_text, tail_pos);
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

fn is_operational_char(ch: &char) -> bool {
    if *ch == '='
        || *ch == '+'
        || *ch == '-'
        || *ch == '*'
        || *ch == '/'
        || *ch == ';'
        || *ch == '{'
        || *ch == '}'
        || *ch == '['
        || *ch == ']'
        || *ch == '('
        || *ch == ')'
        || *ch == '\n'
        || *ch == ','
        || *ch == ' '
        || *ch == '&'
    {
        true
    } else {
        false
    }
}

fn pop_digit(char_queue: &mut VecDeque<char>) -> Result<i32, ()> {
    let mut num = char_queue.pop_front().unwrap().to_digit(10).unwrap() as i32;
    loop {
        let next = char_queue.front();
        if let Some(next_ch) = next {
            if next_ch.is_digit(10) {
                let next_digit = char_queue.pop_front().unwrap().to_digit(10).unwrap() as i32;
                num = num * 10 + next_digit;
            } else if is_operational_char(next_ch) {
                break;
            } else {
                return Err(());
            }
        } else {
            break;
        }
    }
    Ok(num)
}

fn pop_operation(char_queue: &mut VecDeque<char>) -> TokenKind {
    let ch = char_queue.pop_front().unwrap();
    let mut op_string = ch.to_string();

    if op_string == "(" {
        return TokenKind::Parentheses(ParenthesesKind::LeftParentheses);
    } else if op_string == ")" {
        return TokenKind::Parentheses(ParenthesesKind::RightParentheses);
    } else if op_string == "{" {
        return TokenKind::Braces(BracesKind::LeftBraces);
    } else if op_string == "}" {
        return TokenKind::Braces(BracesKind::RightBraces);
    } else if op_string == ";" {
        return TokenKind::StateMentEnd;
    } else if op_string == "," {
        return TokenKind::Comma;
    }

    // <、<=、>、>=、==、!= に対応するため, 次が=ならばそれも取り出す
    if let Some(next_ch_ref) = char_queue.front() {
        if *next_ch_ref == '=' {
            let next_ch = char_queue.pop_front().unwrap();
            op_string.push(next_ch);
        }
    }
    if op_string == "=" {
        TokenKind::Assign
    } else if op_string == ">" {
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
    } else if op_string == "&" {
        TokenKind::Reference
    } else {
        TokenKind::InvalidToken
    }
}

fn pop_identifier(char_queue: &mut VecDeque<char>,) -> String {
    let ch = char_queue.pop_front().unwrap();
    let mut identifier = format!("{}", ch);

    // ascii, _, 0~9 が続くうちは取り出す
    loop {
        if let Some(next_ch_ref) = char_queue.front() {
            if next_ch_ref.is_ascii_alphabetic() || *next_ch_ref == '_' || next_ch_ref.is_digit(10)
            {
                let next_ch = char_queue.pop_front().unwrap();
                identifier.push(next_ch);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    identifier
}

fn pop_identifier_token(
    char_queue: &mut VecDeque<char>,
    local_variable_set: &mut Vec<String>,
    tokenizer_info: &mut TokenizerInfo
) -> TokenKind {

    let text_len = PROGRAM_TEXT.get().unwrap().get_tail_pos();
    let mut indentifier_head_pos = text_len - char_queue.len();

    let mut identifier = pop_identifier(char_queue);
    let mut is_variable_definition = false;

    if identifier == "int" {
        skip_input(char_queue);
        if char_queue.is_empty() {
            error_exit("variable definition is not correct", text_len);
        }

        let ch =char_queue.front().unwrap();
        if !ch.is_ascii_alphabetic() {
            return TokenKind::InvalidToken;
        }

        indentifier_head_pos = text_len - char_queue.len();

        identifier = pop_identifier(char_queue);
        skip_input(char_queue);
        if char_queue.is_empty() {
            error_exit("variable definition is not correct", text_len);
        }

        let ch =char_queue.front().unwrap();
        if tokenizer_info.state == TokenizerState::FucntionDefinition {
            if !(*ch == ')'  || *ch == ',') {
                error_exit("variable definition needs ','", text_len - char_queue.len());
            }
        } else {
            if *ch != ';' {
                error_exit("variable definition needs ';'", text_len - char_queue.len());
            } else {
                char_queue.pop_front();
            }
        }
        is_variable_definition = true;
    }

    if identifier == "int" {
        error_exit("cannot use int for variable name", indentifier_head_pos);
    }

    if identifier == "return" {
        return TokenKind::Return;
    } else if identifier == "while" {
        return TokenKind::While;
    } else if identifier == "if" {
        return TokenKind::If;
    } else if identifier == "else" {
        return TokenKind::Else;
    } else if identifier == "for" {
        return TokenKind::For;
    }

    skip_input(char_queue);
    if char_queue.is_empty() {
        error_exit("program is not correct", text_len);
    }

    let ch = char_queue.front().unwrap();
    if *ch == '(' {
        return TokenKind::FucntionCall(identifier);
    }

    for (index, exist_variable) in local_variable_set.iter().enumerate() {
        if *exist_variable == identifier {
            if is_variable_definition {
                error_exit(&format!("variable {} is already defined", identifier), indentifier_head_pos);
            }
            return TokenKind::LocalVariable((index + 1) * 8);
        }
    }
    if is_variable_definition {
        local_variable_set.push(identifier);
        return TokenKind::LocalVariableDefinition(local_variable_set.len() * 8);
    } else {
        error_exit(&format!("undefined variable {}", identifier), indentifier_head_pos);
    }
}

fn pop_function_definition_token(char_queue: &mut VecDeque<char>) -> Result<String, String> {
    let identifier = pop_identifier(char_queue);

    if identifier != "int" {
        return Err(format!("function must return int"));
    }

    skip_input(char_queue);

    let mut function_name = format!("");
    // ascii, _, 0~9 が続くうちは取り出す
    loop {
        if let Some(next_ch_ref) = char_queue.front() {
            if next_ch_ref.is_ascii_alphabetic() || *next_ch_ref == '_' || next_ch_ref.is_digit(10)
            {
                let next_ch = char_queue.pop_front().unwrap();
                function_name.push(next_ch);
            } else {
                break;
            }
        } else {
            return Err(format!("function definition is not correct"));
        }
    }

    if function_name == "return" {
        return Err(format!("return is invalid function name"));
    } else if function_name == "while" {
        return Err(format!("while is invalid function name"));
    } else if function_name == "if" {
        return Err(format!("if is invalid function name"));
    } else if function_name == "else" {
        return Err(format!("else is invalid function name"));
    } else if function_name == "for" {
        return Err(format!("for is invalid function name"));
    } else if function_name == "int" {
        return Err(format!("int is invalid function name"));
    } 

    return Ok(function_name);
}

// プログラム文終端までコメントの場合はOk(true)を返す
fn skip_comment(char_queue: &mut VecDeque<char>) -> Result<(), ()> {
    if let Some(ch0) = char_queue.get(0) {
        if *ch0 == '/' {
            if let Some(ch1) = char_queue.get(1) {
                if *ch1 == '/' {
                    // 一行コメント
                    char_queue.pop_front();
                    char_queue.pop_front();
                    loop {
                        if let Some(ch) = char_queue.pop_front() {
                            if ch == '\n' {
                                return Ok(());
                            }
                        } else {
                            return Ok(());
                        }
                    }
                } else if *ch1 == '*' {
                    // ブロックコメント
                    char_queue.pop_front();
                    char_queue.pop_front();
                    loop {
                        // 先頭とその次が "*/"の場合まで1文字ずつ取り出す
                        if let Some(ch0) = char_queue.get(0) {
                            if let Some(ch1) = char_queue.get(1) {
                                if *ch0 == '*' && *ch1 == '/' {
                                    char_queue.pop_front();
                                    char_queue.pop_front();
                                    return Ok(());
                                } else {
                                    // 先頭1文字を取り出す
                                    char_queue.pop_front();
                                }
                            } else {
                                // コメントが閉じられていない
                                return Err(());
                            }
                        } else {
                            // コメントが閉じられていない
                            return Err(());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn skip_input(char_queue: &mut VecDeque<char>) {
    loop {
        match skip_comment(char_queue) {
            Ok(_) => {
                if char_queue.is_empty() {
                    break;
                }
            }
            // コメントが閉じられていない場合
            Err(_) => {
                // テキスト終端に要求エラーを立てる
                let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                error_exit("parenthes is not closed", tail_pos);
            }
        }

        let ch = char_queue.front().unwrap();

        if *ch == ' ' || *ch == '\n' {
            char_queue.pop_front();
            continue;
        } else {
            break;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TokenizerState {
    Global,
    Local,
    FucntionDefinition,
}

struct TokenizerInfo {
    state: TokenizerState,
    nest_level: usize,
}

impl TokenizerInfo {
    fn new() -> Self {
        TokenizerInfo {
            state: TokenizerState::Global,
            nest_level: 0,
        }
    }
}

// 入力テキストのトークン連結リストを作成する
pub fn text_tokenizer(text: &str) -> Vec<TokenList> {
    // グローバルなPROGRAM_TEXTにミュータブルなcursorを用意して一文字ずつ参照, cursorを移動したいが,
    // グローバル変数はアクセスが面倒なので,
    // スタック内にVecDequeを用意して, トークン化はそれで行う
    let mut program_char: Vec<char> = text.chars().collect();
    if !program_char.is_empty() {
        if let Some(ch) = program_char.get(program_char.len() - 1) {
            if *ch != '\n' {
                program_char.push('\n');
            }
        }
    }
    let program_text = ProgramText::new(program_char);
    PROGRAM_TEXT.set(program_text).ok();
    let mut char_queue = VecDeque::from_iter(text.chars());
    let mut tokenlist_vec: Vec<TokenList> = vec![];

    let mut tokenlist = TokenList::new();
    let mut current_token = &mut tokenlist.head;

    let text_len = char_queue.len();
    let mut local_varibales: Vec<String> = vec![];
    let mut tokenizer_info = TokenizerInfo::new();

    while !char_queue.is_empty() {
        skip_input(&mut char_queue);
        if char_queue.is_empty() {
            break;
        }

        // 解析トークンの位置はテキストの長さ-未処理文字数で求まる
        let token_pos = text_len - char_queue.len();
        let mut new_token = Token::new(token_pos);

        let ch = char_queue.front().unwrap();

        if tokenizer_info.state == TokenizerState::Global {
            if ch.is_ascii_alphabetic() {
                match pop_function_definition_token(&mut char_queue) {
                    Ok(function_name) => {
                        new_token.token_kind = TokenKind::FucntionDefinition(function_name);
                    }
                    Err(err_string) => {
                        error_exit(&err_string, token_pos);
                    }
                }
                // ロカール変数配列初期化
                // 引数もローカル変数として扱うため, ここで初期化する
                local_varibales = vec![];
                tokenizer_info.state = TokenizerState::FucntionDefinition;
            } else {
                error_exit(
                    "only function definition is allowed in global area",
                    token_pos,
                );
            }
        } else if ch.is_digit(10) {
            match pop_digit(&mut char_queue) {
                Ok(num) => {
                    new_token.token_kind = TokenKind::Number(num);
                }
                Err(()) => {
                    let error_pos = text_len - char_queue.len();
                    error_exit("unsupported token", error_pos);
                }
            }
        } else if ch.is_ascii_punctuation() {
            new_token.token_kind = pop_operation(&mut char_queue);

            if new_token.token_kind == TokenKind::Parentheses(ParenthesesKind::RightParentheses) 
               && tokenizer_info.state == TokenizerState::FucntionDefinition {
                tokenizer_info.state = TokenizerState::Local;
            }

            // {}でネストレベルを判定
            if new_token.token_kind == TokenKind::Braces(BracesKind::LeftBraces) {
                tokenizer_info.nest_level += 1;
            } else if new_token.token_kind == TokenKind::Braces(BracesKind::RightBraces) {
                tokenizer_info.nest_level -= 1;
                if tokenizer_info.nest_level == 0 {
                    tokenizer_info.state = TokenizerState::Global;
                    // tokenlistを更新して
                    match current_token {
                        Some(token) => {
                            token.next = Some(Box::new(new_token));
                        }
                        None => {
                            *current_token = Some(Box::new(new_token));
                        }
                    }
                    tokenlist.local_stack_size = local_varibales.len() * 8;
                    tokenlist_vec.push(tokenlist);
                    tokenlist = TokenList::new();
                    current_token = &mut tokenlist.head;
                    continue;
                }
            }
        } else if ch.is_ascii_alphabetic() {
            new_token.token_kind = pop_identifier_token(&mut char_queue, &mut local_varibales, &mut tokenizer_info);
        }

        if new_token.token_kind == TokenKind::InvalidToken {
            error_exit("unsupported token", new_token.token_pos);
        } 

        if let TokenKind::LocalVariableDefinition(_) = new_token.token_kind {
            if tokenizer_info.state != TokenizerState::FucntionDefinition {
                continue;
            }
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
    tokenlist_vec
}
