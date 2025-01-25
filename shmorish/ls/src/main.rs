use std::env;
use std::io;

/*
    ls c b a /
    ls: a: No such file or directory
    ls: b: No such file or directory
    ls: c: No such file or directory
    /:
    Applications System       Volumes      cores        etc          opt          sbin         usr
    Library      Users        bin          dev          home         private      tmp          var
*/

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("args -> {:?}", args);
    for path in &args {
        println!("{}", path);
    }
    Ok(())
}