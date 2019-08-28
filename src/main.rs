use std::error::Error;

use kah::cli;
use kah::cli::Cmd;
use kah::get::get_kattis_sample;
use kah::init::parse_kattisrc;
use kah::test::test_kattis;

fn main() -> Result<(), Box<dyn Error>> {
    match cli::parse().cmd {
        Cmd::Get { pid, name } => get_kattis_sample(&pid, &name)?,
        Cmd::Test { .. } => test_kattis()?,
        Cmd::Submit { .. } => println!("You are submitting something!"),
        Cmd::Init { file, force } => parse_kattisrc(file, force)?,
    }

    Ok(())
}
