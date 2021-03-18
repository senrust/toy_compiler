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
    writeln!(buf, "").unwrap();
    writeln!(buf, "main:").unwrap();
}

fn write_footer<T: Write>(buf: &mut T) {
    writeln!(buf, "    pop rax").unwrap();
    writeln!(buf, "    ret").unwrap();
}

fn write_operation<T: Write>(buf: &mut T, instruction: String) {
    writeln!(buf, "{}", instruction).unwrap();
}

pub fn output_asembly(input_text: &str) {
    let mut file = BufWriter::new(fs::File::create("tmp.s").unwrap());
    write_header(&mut file);

    let mut token_list = tokenizer::text_tokenizer(input_text);
    let mut ast = ast::AST::new(&mut token_list);
    let instruction_vec = compiler::compile_ast(&mut ast);
    instruction_vec
        .into_iter()
        .for_each(|instruction| write_operation(&mut file, instruction));
    write_footer(&mut file);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let input_text = &args[1];
    let mut file = BufWriter::new(fs::File::create("tmp.s").unwrap());
    write_header(&mut file);

    let mut token_list = tokenizer::text_tokenizer(input_text);
    let mut ast = ast::AST::new(&mut token_list);
    let instruction_vec = compiler::compile_ast(&mut ast);
    instruction_vec
        .into_iter()
        .for_each(|instruction| write_operation(&mut file, instruction));
    write_footer(&mut file);
}
