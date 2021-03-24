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
}

fn write_prologue<T: Write>(buf: &mut T, local_variable_size: usize) {
    writeln!(buf, "main:").unwrap();
    writeln!(buf, "    push rbp").unwrap();
    writeln!(buf, "    mov rbp, rsp").unwrap();
    if local_variable_size == 0 {
        // 何もしない
    } else if local_variable_size  % 16 != 0 {
        writeln!(buf, "    sub rsp, {}", local_variable_size + 8).unwrap();
    } else  {
        writeln!(buf, "    sub rsp, {}", local_variable_size).unwrap();
    } 
}

fn write_epilogue<T: Write>(buf: &mut T) {
    writeln!(buf, "    mov rsp, rbp").unwrap();
    writeln!(buf, "    pop rbp").unwrap();
}

fn write_footer<T: Write>(buf: &mut T) {
    writeln!(buf, "    ret").unwrap();
}

fn write_operation<T: Write>(buf: &mut T, instruction: String) {
    writeln!(buf, "{}", instruction).unwrap();
}

pub fn output_asembly(input_text: &str) {
    let mut file = BufWriter::new(fs::File::create("tmp.s").unwrap());
    write_header(&mut file);

    let token_list_vec = tokenizer::text_tokenizer(input_text);
    for mut token_list in token_list_vec {
        let ast_vec = ast::ASTVec::make_ast_vec(&mut token_list);
        let instruction_vec = compiler::compile_astvec(ast_vec);
        instruction_vec
        .into_iter()
        .for_each(|instruction| write_operation(&mut file, instruction));
        write_epilogue(&mut file);
        write_footer(&mut file);
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
