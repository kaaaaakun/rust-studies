use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(
    name = "watch",
    about = "A simple command watcher",
    usage = "watch [options] <command>"
)]
pub struct Args {
    #[clap(short = 'b', long, help = "beep if command has a non-zero exit")]
    pub beep: bool,
    #[clap(short = 'c', long, help = "interpret ANSI color and style sequences")]
    pub color: bool,
    #[clap(short = 'd', long, help = "highlight changes between updates")]
    pub differences: Option<String>,
    #[clap(short = 'e', long, help = "exit if command has a non-zero exit")]
    pub errexit: bool,
    #[clap(short = 'g', long, help = "exit when output from command changes")]
    pub chgexit: bool,
    #[clap(short = 'n', long, value_name = "secs", help = "run command every secs seconds")]
    pub interval: Option<f64>,
    #[clap(short = 't', long, help = "turn off header")]
    pub no_title: bool,

    #[clap(name = "command", required = true, help = "command to run")]
    pub command: Option<String>,
}
