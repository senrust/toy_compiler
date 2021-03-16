#[cfg(test)]
mod tests {
    use crate::compiler::{compile_operation, expect_number, expect_operation};
    use crate::tokenizer::{text_tokenizer, OperationKind};
    use crate::ast::{AST};
    use std::process::Command;

    #[test]
    fn binary_test() {
        let status = Command::new("sh")
            .arg("-c")
            .arg("./a.out")
            .status()
            .expect("failed to execute mkdir")
            .code()
            .unwrap();
        assert_eq!(status, 21);
    }

    #[test]
    fn tokenizer_test() {
        let input_text = "18 + 21 - 8";
        let mut token_list = text_tokenizer(input_text);
        assert_eq!(expect_number(&mut token_list), 18);
        assert_eq!(expect_operation(&mut token_list), OperationKind::Add);
        assert_eq!(expect_number(&mut token_list), 21);
        assert_eq!(expect_operation(&mut token_list), OperationKind::Sub);
        assert_eq!(expect_number(&mut token_list), 8);
        assert!(token_list.is_empty());
    }

    #[test]
    fn formula_test() {
        let input_text = "+ 21 - 8";
        let mut token_list = text_tokenizer(input_text);
        assert_eq!(compile_operation(&mut token_list), "    add rax, 21");
        assert_eq!(compile_operation(&mut token_list), "    sub rax, 8");
        assert!(token_list.is_empty());
    }

}
