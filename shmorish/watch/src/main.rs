use std::io;
use clap::{Parser, CommandFactory};
use std::process::exit;

mod utils;
use utils::args::Args;
use utils::firstline::print_first_line;
use utils::output::{process_output, remove_tmp_file};

const DEFAULT_INTERVAL: f64 = 2.0;
const BEEP_SOUND: &str = "\x07";
const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";

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
        process_output(&output);

        let exit_status = output.status.code().unwrap_or(0);
        handle_exit_status(exit_status);

        std::thread::sleep(std::time::Duration::from_secs_f64(interval));
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
