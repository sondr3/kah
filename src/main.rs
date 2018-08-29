#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="kattis-rs")]
struct Opt {
    #[structopt(short = "l", long = "language", parse(from_os_str))]
    language: PathBuf,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
