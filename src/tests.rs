#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_arch="x86")]
    fn binary_test() {
        use std::process::Command;
        let status = Command::new("sh")
            .arg("-c")
            .arg("./a.out")
            .status()
            .expect("failed to execute mkdir")
            .code()
            .unwrap();
        assert_eq!(status, 21);
    }
}
