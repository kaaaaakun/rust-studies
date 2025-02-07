use chrono::Local;
use terminal_size::{Width, terminal_size};
use nix::sys::utsname;

const DEFAULT_WIDTH: usize = 80;

pub fn print_first_line(command: &str, interval: f64) {

    let cname_time = format!("Every {}s: {}", interval, command);

    let binding = utsname::uname();
    let uname = binding.sysname();
    let now = Local::now();
    let date = now.format("%a %b %d %H:%M:%S %Y").to_string();
    let display_on_right = format!("{}: {}", uname, date);

    let terminal_width;
    if let Some((Width(w), _)) = terminal_size() {
        terminal_width = w as usize;
    } else {
        terminal_width = DEFAULT_WIDTH;
    }

    let padding = terminal_width - cname_time.len();
    println!("{}{:>padding$}",cname_time, display_on_right, padding = padding);
    println!();
}
