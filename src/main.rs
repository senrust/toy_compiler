use std::fs;
use std::io::{BufWriter, Write};

pub mod compiler;
pub mod tokenizer;

fn write_header<T: Write>(buf:&mut T) {
    writeln!(buf, ".intel_syntax noprefix").unwrap();
    writeln!(buf, ".globl main").unwrap();
    writeln!(buf, "\n").unwrap();
    writeln!(buf, "main:").unwrap();
}

fn write_footer<T: Write>(buf:&mut T) {
    writeln!(buf, "    ret").unwrap();
}

fn main() {
    let mut token_lkist = tokenizer::text_tokenizer("a12 + 8 * 6126");
    compiler::expect_number(&mut token_lkist);
    compiler::compile_operation(&mut token_lkist);
    compiler::compile_operation(&mut token_lkist);
    let mut file = BufWriter::new(fs::File::create("tmp.s").unwrap());
    write_header(&mut file);
    writeln!(file, "    mov rax, 10").unwrap();
    write_footer(&mut file);
}
