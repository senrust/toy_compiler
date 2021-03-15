use std::fs;
use std::io::{BufWriter, Write};

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
    let mut file = BufWriter::new(fs::File::create("tmp.s").unwrap());
    write_header(&mut file);
    writeln!(file, "    mov rax, 10").unwrap();
    write_footer(&mut file);
}

#[cfg(test)]
mod tests {
    use std::process::Command;

  #[test]
  fn binary_test() {
    let status = Command::new("sh")
        .arg("-c")
        .arg("./a.out")
        .status()
        .expect("failed to execute mkdir").code().unwrap();
    assert_eq!(status, 10);
  }
}