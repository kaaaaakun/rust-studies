use std::env;
use std::fs;
use std::io;
use std::path::Path;

/*
    ls c b a /
    ls: a: No such file or directory
    ls: b: No such file or directory
    ls: c: No such file or directory
    /:
    Applications System       Volumes      cores        etc          opt          sbin         usr
    Library      Users        bin          dev          home         private      tmp          var
*/

fn get_args() -> Vec<String> {
    let mut args: Vec<String> = env::args()
                                    .skip(1)
                                    .collect();
    args.sort();
    return args;
}

// fn directory_exists() -> Result<Vec<String>, io::Error> {
//     let mut paths: Vec<String> = Vec::new();
//     for path in &args {
//         if path.is_dir() {
//             paths.push(path);
//         } else {
//             Err(io::Error::new(io::ErrorKind::NotFound, "No such file or directory"));
//         }
//     }
//     return Ok(paths);
// }

fn print_dir(path: &str) {
    println!("{}", path);
}


fn main() -> io::Result<()> {
    let args: Vec<String> = get_args();
    let dir_path:

    if args.len() == 0 {
        print_dir(".");
        return Ok(());
    }
    for path in &args {
        println!("{}", path);
    }
    return Ok(());
}

// use std::env;
// use std::fs;
// use std::io;
// use std::path::Path;

// fn main() -> io::Result<()> {
//     let args: Vec<String> = env::args().collect();

//     let dir_path = if args.len() > 1 {
//         &args[1]
//     } else {
//         "."
//     };

//     let path = Path::new(dir_path);

//     for entry_result in fs::read_dir(path)? {
//         let entry = entry_result?;
//         let file_name = entry.file_name();
//         println!("{}", file_name.to_string_lossy());
//     }

//     Ok(())
// }