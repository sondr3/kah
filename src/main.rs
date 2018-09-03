extern crate kattis_rs;

use kattis_rs::cli;
use kattis_rs::cli::Cmd;
use kattis_rs::get::get_kattis_sample;
use kattis_rs::test::test_kattis;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli::parse().cmd {
        Cmd::Get { pid, name, url } => get_kattis_sample(&url, &pid, &name)?,
        Cmd::Test { .. } => test_kattis()?,
        Cmd::Submit { .. } => println!("You are submitting something!"),
        Cmd::Init { .. } => println!("Fetching user information..."),
    }

    Ok(())
}
