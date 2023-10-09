#[macro_use]
extern crate bitflags;

use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;
use os_display::Quotable;

use crate::md::Md;

mod ioctl;
mod md;

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
        println!(" * {}", device.maybe_quote());

        match Md::open(device) {
            Ok(Md {
                   superblock,
                   minor_version,
                   ..
               }) => {
                println!(
                    "    * Version: {}.{}",
                    superblock.major_version().read(),
                    minor_version
                );
                println!(
                    "    * Array UUID: {:02x}",
                    superblock.array_uuid().iter().format("")
                );
                println!(
                    "    * Array Name: {}",
                    String::from_utf8_lossy(superblock.array_name())
                );
            }
            Err(error) => println!("    * Error: {}", error),
        }
    }

    println!();

    println!("Recovered files will be written to:");

    for path in options.output {
        println!(" * {}", path.maybe_quote());
    }
}
