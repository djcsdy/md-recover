use clap::Parser;
use os_display::Quotable;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Options {
    devices: Vec<PathBuf>,

    #[arg(short, long)]
    output: Vec<PathBuf>,
}

fn main() {
    let options = Options::parse();

    println!("Attempting to recover data from:");

    for device in options.devices {
        println!(" * {}", device.maybe_quote())
    }

    println!();

    println!("Recovered files will be written to:");

    for path in options.output {
        println!(" * {}", path.maybe_quote());
    }
}
