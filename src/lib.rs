pub mod tokenizer;
pub mod compiler;

#[cfg(test)]
mod tests {
    use std::process::Command;
    use super::tokenizer::*;
    use super::compiler::*;

  #[test]
  fn binary_test() {
        let status = Command::new("sh")
        .arg("-c")
        .arg("./a.out")
        .status()
        .expect("failed to execute mkdir").code().unwrap();
    assert_eq!(status, 10);
  }

  #[test]
  fn tokenizer_test() {
    let input_text = "18 + 21 - 8";
    let mut token_list = text_tokenizer(input_text);
    assert_eq!(expect_number(&mut token_list), 18);
    assert_eq!(expect_operation(&mut token_list), TokenKind::Add);
    assert_eq!(expect_number(&mut token_list), 21);
    assert_eq!(expect_operation(&mut token_list), TokenKind::Sub);
    assert_eq!(expect_number(&mut token_list), 8);
    assert!(token_list.is_empty());
  }
}