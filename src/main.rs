#[macro_use]
extern crate bitflags;

use std::path::PathBuf;

use clap::Parser;
use os_display::Quotable;

use crate::md::MdDevice;
use crate::md::superblock::Superblock;

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

        match MdDevice::open(device) {
            Ok(MdDevice {
                   superblock,
                   minor_version,
                   ..
               }) => {
                println!(
                    "    * Version: {}.{}",
                    superblock.major_version(),
                    minor_version
                );
                println!("    * Array UUID: {}", superblock.array_uuid());
                match superblock.array_name() {
                    None => {}
                    Some(name) => println!("    * Array Name: {}", name.maybe_quote())
                }
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
