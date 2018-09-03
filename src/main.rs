extern crate kattis_rs;

use kattis_rs::cli;
use kattis_rs::cli::Cmd;
use kattis_rs::get::get_kattis_sample;

#[derive(Debug)]
enum Language {
    Python,
    Java,
}

#[derive(Debug)]
struct LanguageInfo {
    name: Language,
    extension: String,
    command: String,
}

fn test_kattis() -> Result<(), Box<dyn std::error::Error>> {
    println!("YOU ARE TESTING ME");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _java = LanguageInfo {
        name: Language::Java,
        extension: "java".to_string(),
        command: "java".to_string(),
    };

    let _python = LanguageInfo {
        name: Language::Python,
        extension: ".py".to_string(),
        command: "python".to_string(),
    };

    match cli::parse().cmd {
        Cmd::Get { pid, name, url } => get_kattis_sample(&url, &pid, &name)?,
        Cmd::Test { .. } => test_kattis()?,
        Cmd::Submit { .. } => println!("You are submitting something!"),
        Cmd::Init { .. } => println!("Fetching user information..."),
    }

    Ok(())
}
