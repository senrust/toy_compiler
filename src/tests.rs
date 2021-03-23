#[cfg(test)]
mod tests {
    #[cfg(target_arch = "x86_64")]
    use std::process::Command;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn make_binary_from_asm() {
        Command::new("cc")
            .arg("-o")
            .arg("./a.out")
            .arg("tmp.s")
            .arg("include_func.c")
            .output()
            .expect("failed to asemble binary");
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn compare_output() -> i32 {
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
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn compiler_test() {
        use crate::output_asembly;
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let f = File::open("./test/binary_test.txt").unwrap();
        let mut lines_iter = BufReader::new(f).lines();
        let correct_output: i32 = lines_iter.next().unwrap().ok().unwrap().parse().unwrap();
        let mut input_program = format!("");
        for line_result in lines_iter {
            let line = line_result.unwrap();
            input_program = format!("{}\n{}", input_program, line);
        }
        output_asembly(&input_program);
        make_binary_from_asm();
        let result = compare_output();
        if correct_output == result {
            println!("suceeded!");
        } else {
            println!(
                "test failed! expected {} but {} retuend",
                correct_output, result
            );
            panic!();
        }
    }

    #[test]
    #[cfg(any(target_arch = "arm"))]
    fn assembley_test() {
        use crate::output_asembly;
        use std::fs;
        let input_program = fs::read_to_string("./test/binary_test.txt").unwrap();
        output_asembly(&input_program);
    }
}
