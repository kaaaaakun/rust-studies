use std::io::Write;
use diff::lines;

const CURRENT_RESULT_FILE: &str = "/tmp/.watch_stdout_current";
const FIRST_RESULT_FILE: &str = "/tmp/.watch_stdout";

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";


pub fn process_output(output: &std::process::Output) {
    if !output.stdout.is_empty() {
        write_output_file(CURRENT_RESULT_FILE, &output.stdout);
    }
    if std::fs::metadata(FIRST_RESULT_FILE).is_ok() {
        let previous_output = std::fs::read_to_string(FIRST_RESULT_FILE).unwrap_or_default();
        let current_output = std::fs::read_to_string(CURRENT_RESULT_FILE).unwrap_or_default();
        print_diff(&previous_output, &current_output);
    } else {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        print!("{}", stdout_str);
        write_output_file(FIRST_RESULT_FILE, &output.stdout);
    }
}

fn write_output_file(path: &str, data: &[u8]) {
    let mut file = std::fs::File::create(path).expect("Failed to create file");
    file.write_all(data).expect("Failed to write file");
}

fn print_diff(previous: &str, current: &str) {
    for diff in lines(previous, current) {
        match diff {
            diff::Result::Left(l) => println!("{}{}{}", RED, l, RESET),
            diff::Result::Right(r) => println!("{}{}{}", GREEN, r, RESET),
            diff::Result::Both(b, _) => println!("{}", b),
        }
    }
}