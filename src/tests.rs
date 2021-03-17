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

    fn compare_output() -> i32{
        let status = Command::new("sh")
            .arg("-c")
            .arg("./a.out")
            .status()
            .expect("failed to execute binary")
            .code()
            .unwrap();
        status
    }

    #[test]
    fn compiler_test() {
        let f = File::open("testset.txt").unwrap();
        for line_result in BufReader::new(f).lines() {
            let test_set = line_result.unwrap();
            let test_vec: Vec<&str> = test_set.split(",").collect();
            let correct_output: i32 = test_vec[0].parse().unwrap();
            let input_text: &str = test_vec[1].trim();
            println!("assert {} = {}", test_vec[0], test_vec[1]);
            output_asembly(input_text);
            make_binary_from_asm();
            let result = compare_output();
            if correct_output == result {
                println!("suceeded!");
            } else {
                println!("test failed! expected {} but {} retuend", correct_output, result);
                panic!();
            }
        }
    }
}
