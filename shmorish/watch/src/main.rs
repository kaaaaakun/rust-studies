use std::{io, io::Write};
use clap::{Parser, CommandFactory};
use std::process::exit;
use diff::lines;

mod args;
use crate::args::Args;

mod firstline;
use crate::firstline::print_first_line;

const DEFAULT_INTERVAL: f64 = 2.0;

const BEEP_SOUND: &str = "\x07";
const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";

const CURRENT_RESULT_FILE: &str = "/tmp/.watch_stdout_current";
const FIRST_RESULT_FILE: &str = "/tmp/.watch_stdout";

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn handle_exit_status(exit_status: i32) {
    let args = Args::parse();
    if exit_status != 0 {
        if args.beep {
            print!("{}", BEEP_SOUND);
        }
        if args.errexit {
            exit(exit_status);
        }
    }
}

fn execute_command(command: String) -> std::process::Output {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command")
        .wait_with_output()
        .expect("Failed to wait for command")
}

fn execute_command_loop(command: String) {
    let args = Args::parse();
    let interval = args.interval.unwrap_or(DEFAULT_INTERVAL);
    loop {
        print!("{}", CLEAR_SCREEN);
        if !args.no_title {
            print_first_line(&command, interval);
        }

        let output = execute_command(command.clone());

        if !output.stdout.is_empty() {
            let mut file = std::fs::File::create(CURRENT_RESULT_FILE).expect("Failed to create file");
            file.write_all(&output.stdout).expect("Failed to write file");
        }

        if std::fs::metadata(FIRST_RESULT_FILE).is_ok() {
            let previous_output = std::fs::read_to_string(FIRST_RESULT_FILE).unwrap_or_default();
            let current_output = std::fs::read_to_string(CURRENT_RESULT_FILE).unwrap_or_default();
            for diff in lines(&previous_output, &current_output) {
                match diff {
                    diff::Result::Left(l) => println!("{}{}{}", RED, l, RESET),
                    diff::Result::Right(r) => println!("{}{}{}", GREEN, r, RESET),
                    diff::Result::Both(b, _) => println!("{}", b),
                }
            }
        } else {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            print!("{}", stdout_str);
            let mut file = std::fs::File::create(FIRST_RESULT_FILE).expect("Failed to create file");
            file.write_all(&output.stdout).expect("Failed to write file");
        }


        let exit_status = output.status.code().unwrap_or(0);
        handle_exit_status(exit_status);

        std::thread::sleep(std::time::Duration::from_secs_f64(interval));
    }
}

fn remove_tmp_file() {
    if std::fs::metadata(CURRENT_RESULT_FILE).is_ok() {
        std::fs::remove_file(CURRENT_RESULT_FILE).expect("Failed to remove file");
    }
    if std::fs::metadata(FIRST_RESULT_FILE).is_ok() {
        std::fs::remove_file(FIRST_RESULT_FILE).expect("Failed to remove file");
    }
}

fn main() -> io::Result<()> {
    remove_tmp_file();
    let args = Args::parse();

    if let Some(execute_command) = args.command {
        dbg!("{:?}", &execute_command);
        execute_command_loop(execute_command);
    } else {
        Args::command().print_help().unwrap();
        println!();
        exit(1);
    }
    Ok(())
}
