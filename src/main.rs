use structopt::StructOpt;
use tempfile::tempdir;
use std::fs::File;
use std::io::{Write, Read};

#[derive(StructOpt, Debug)]
#[structopt(name = "kattis-rs", about = "a simple Kattis helper utility")]
enum Opt {
    #[structopt(name = "get")]
    /// Get sample test files from Kattis
    Get {
        #[structopt(help = "Problem ID")]
        id: String,
        #[structopt(help = "Problem Name")]
        name: String,
        #[structopt(
            short = "u",
            long = "url",
            default_value = "https://open.kattis.com"
        )]
        url: String,
    },
    #[structopt(name = "test")]
    /// Test a Kattis assignment
    Test {
        #[structopt(help = "Kattis assignment to run")]
        file: String,
        #[structopt(short = "l", long = "language", help = "Override language")]
        language: Option<String>,
    },
}

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

fn kattis_file_path(id: String) -> String {
    String::from(format!("problems/{}/file/statement/samples.zip", id))
}

fn get_kattis_sample(url: String, id: String) -> Result<(), Box<std::error::Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("samples.zip");

    println!("{:?}", dir);
    println!("{:?}", file_path);

    let mut file = File::create(file_path)?;
    let mut buffer = Vec::new();

    let path: String = format!("{}/{}", url, &kattis_file_path(id));
    let mut response = reqwest::get(&path)?;
    response.read_to_end(&mut buffer)?;

    println!("{}", path);
    println!("Status: {}", response.status());
    println!("Headers:\n{:?}", response.headers());

    file.write(&mut buffer)?;
    dir.close()?;
    Ok(())
}

fn test_kattis() -> Result<(), Box<std::error::Error>> {
    println!("YOU ARE TESTING ME");
    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let java = LanguageInfo {
        name: Language::Java,
        extension: "java".to_string(),
        command: "java".to_string(),
    };

    let opt = Opt::from_args();

    match opt {
        Opt::Get {
            id: _,
            name: _,
            url: _,
        } => get_kattis_sample("https://open.kattis.com".to_string(), "trik".to_string())?,
        Opt::Test {
            file: _,
            language: _,
        } => test_kattis()?,
    }

    println!("{:?}", opt);
    Ok(())
}
