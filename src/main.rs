use std::fs::{create_dir_all, remove_file, File};
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use tempfile::tempdir;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "kattis-rs", about = "a simple Kattis helper utility")]
struct Opt {
    #[structopt(short = "v", long = "verbose")]
    /// Verbose messages
    verbose: bool,
    #[structopt(short = "f", long = "force")]
    /// Overwrite existing files
    force: bool,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt, PartialEq, Debug)]
enum Cmd {
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
        /// URL to fetch files from
        url: String,
    },

    #[structopt(name = "test")]
    /// Test a Kattis assignment
    Test {
        #[structopt(help = "Kattis assignment to run")]
        /// Which assignment to run tests for
        file: String,
        #[structopt(short = "l", long = "language")]
        /// Select language for problem
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

fn kattis_file_path(id: &str) -> String {
    format!("problems/{}/file/statement/samples.zip", id)
}

fn get_kattis_sample(url: &str, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("samples.zip");

    println!("{:?}", dir);
    println!("{:?}", file_path);

    let mut file = File::create(&file_path)?;
    let mut buffer = Vec::new();

    let path: String = format!("{}/{}", url, &kattis_file_path(id));
    let mut response = reqwest::get(&path)?;
    response.read_to_end(&mut buffer)?;

    println!("{}", path);
    println!("Status: {}", response.status());
    println!("Headers:\n{:?}", response.headers());

    file.write_all(&buffer)?;
    unzip(&file_path)?;
    dir.close()?;
    Ok(())
}

fn unzip(file_name: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let fname = std::path::Path::new(&file_name);
    let file = File::open(&fname)?;

    println!("{:?}", fname);
    println!("{:?}", file);

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file.sanitized_name();

        println!(
            "File {} extracted to \"{}\" ({} bytes)",
            i,
            outpath.as_path().display(),
            file.size()
        );
        if let Some(p) = outpath.parent() {
            if !p.exists() {
                create_dir_all(&p).unwrap();
            }
        }
        let mut outfile = File::create(&outpath)?;
        io::copy(&mut file, &mut outfile)?;
    }

    remove_file(fname)?;
    Ok(())
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

    let matches = Opt::from_args();

    match matches.cmd {
        Cmd::Get { .. } => get_kattis_sample("https://open.kattis.com", "trik")?,
        Cmd::Test { .. } => test_kattis()?,
    }

    println!("{:?}", matches);
    Ok(())
}
