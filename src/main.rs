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
}

fn write_operation<T: Write>(buf: &mut T, instruction: String) {
    writeln!(buf, "{}", instruction).unwrap();
}

pub fn output_asembly(input_text: &str) {
    let mut file = BufWriter::new(fs::File::create("tmp.s").unwrap());
    write_header(&mut file);

    let token_list_vec = tokenizer::text_tokenizer(input_text);
    for mut token_list in token_list_vec {
        let function_ast = ast::FunctionAST::make_function_ast(&mut token_list);
        let instruction_vec = compiler::compile_function_ast(function_ast);
        instruction_vec
            .into_iter()
            .for_each(|instruction| write_operation(&mut file, instruction));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let input_text = &args[1];
    output_asembly(input_text);
}
