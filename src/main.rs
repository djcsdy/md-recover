#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate bitflags;

use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;
use os_display::Quotable;

use crate::md::{MdArray, MdDevice};

mod block_device;
mod confidence;
mod ext;
mod ext4;
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

    let (devices, device_errors): (Vec<_>, Vec<_>) = options
        .devices
        .iter()
        .map(|path| MdDevice::open_path(path).map_err(|err| (path, err)))
        .partition_result();

    if device_errors.is_empty() {
        let array = MdArray::new(devices);
        let diagnosis = array.diagnose();
        println!("{:?}", diagnosis);
    } else {
        for (path, error) in device_errors {
            println!("{}: {}", path.maybe_quote(), error);
        }
    }
}
