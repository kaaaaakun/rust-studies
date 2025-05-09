use std::env;
use std::error::Error;
use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(
    name = "micro_cat",
    about = "Micro concatnation",
    usage = "micoro?cat [options] <command>"
)]
 struct Args {
    #[clap(short = 't', long, help = "test if command has a non-zero exit")]
     beep: bool,

    #[clap(name = "command", required = true, help = "command to run")]
     command: Option<String>,
}

fn do_cat(path: &Path) {
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {:?}: {}", path, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {:?}: {}", path, why),
        Ok(_) => print!("{:?} contains:\n{}", path, s),
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();
//    let argc = argv.len();
    let path = Path::new(&argv[1]);

    do_cat(path);
}

