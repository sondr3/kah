use std::error::Error;

use kah::cli;
use kah::cli::Cmd;
use kah::get::get_kattis_sample;
use kah::kattis::Kattis;
use kah::test::test_kattis;
use kah::init::create_kah_dotfile;

fn main() -> Result<(), Box<dyn Error>> {
    match cli::parse().cmd {
        Cmd::Get { pid, name } => get_kattis_sample(&pid, &name)?,
        Cmd::Test { .. } => test_kattis()?,
        Cmd::Submit { .. } => println!("You are submitting something!"),
        Cmd::Init { file } => {
            let kattis = Kattis::new(file);
            create_kah_dotfile(".kah", &kattis, false).expect("Could not initialize kah")
        }
    }

    Ok(())
}
