use std::env;
use std::fs;
use std::io::{BufWriter, Write};

mod ast;
mod compiler;
mod error;
mod tests;
mod tokenizer;

fn write_header<T: Write>(buf: &mut T) {
    writeln!(buf, ".intel_syntax noprefix").unwrap();
    writeln!(buf, ".globl main").unwrap();
    writeln!(buf, "\n").unwrap();
    writeln!(buf, "main:").unwrap();
}

fn write_footer<T: Write>(buf: &mut T) {
    writeln!(buf, "    ret").unwrap();
}

fn write_number<T: Write>(buf: &mut T, num: i32) {
    writeln!(buf, "    mov rax, {}", num).unwrap();
}

fn write_operation<T: Write>(buf: &mut T, token_list: &mut tokenizer::TokenList) {
    let operation_string = compiler::compile_operation(token_list);
    writeln!(buf, "{}", operation_string).unwrap();
}

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("unsupported input text");
        return;
    }
    let input_text = &args[1];
    let mut file = BufWriter::new(fs::File::create("tmp.s").unwrap());
    write_header(&mut file);
    */
    
    // let mut token_list = tokenizer::text_tokenizer(input_text);
    let mut token_list = tokenizer::text_tokenizer("10 )");
    ast::AST::new(&mut token_list);
    /*
    let num = compiler::expect_number(&mut token_list);
    write_number(&mut file, num);
    while !token_list.is_empty() {
        write_operation(&mut file, &mut token_list);
    }
    write_footer(&mut file);
    */
}
