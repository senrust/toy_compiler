#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::process::Command;
    use crate::output_asembly;

    fn make_binary_from_asm() {
        Command::new("cc")
            .arg("-o")
            .arg("./a.out")
            .arg("tmp.s")
            .output()
            .expect("failed to asemble binary");
    }

    fn compare_output(correct_result: i32){
        let status = Command::new("sh")
            .arg("-c")
            .arg("./a.out")
            .status()
            .expect("failed to execute binary")
            .code()
            .unwrap();
        assert_eq!(status, correct_result);
    }

    #[test]
    fn compiler_test() {
        let f = File::open("testset.txt").unwrap();
        for line_result in BufReader::new(f).lines() {
            let test_set = line_result.unwrap();
            let test_vec: Vec<&str> = test_set.split(",").collect();
            let correct_output: i32 = test_vec[0].parse().unwrap();
            let input_text: &str = test_vec[1].trim();
            output_asembly(input_text);
            make_binary_from_asm();
            compare_output(correct_output);
        }
    }
}
