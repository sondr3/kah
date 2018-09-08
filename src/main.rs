extern crate kattis_rs;

use std::error::Error;

use kattis_rs::cli;
use kattis_rs::cli::Cmd;
use kattis_rs::get::get_kattis_sample;
use kattis_rs::test::test_kattis;
use kattis_rs::init::parse_kattisrc;

fn main() -> Result<(), Box<Error>> {
    match cli::parse().cmd {
        Cmd::Get { pid, name, url } => get_kattis_sample(&url, &pid, &name)?,
        Cmd::Test { .. } => test_kattis()?,
        Cmd::Submit { .. } => println!("You are submitting something!"),
        Cmd::Init { file } => parse_kattisrc(file)?,
    }

    Ok(())
}
