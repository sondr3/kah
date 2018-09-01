extern crate reqwest;
#[macro_use]
extern crate structopt;
extern crate tempfile;
extern crate zip;

use std::error::Error;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{copy, Read, Write};
use std::path::{Path, PathBuf};
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
        pid: String,
        #[structopt(help = "Problem Name")]
        name: String,
        #[structopt(short = "u", long = "url", default_value = "https://open.kattis.com")]
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

fn kattis_samples_output(name: &str) -> String {
    format!("samples/{}/", name)
}

fn create_kattis_folders(name: &str) -> Result<(), Box<Error>> {
    let path = Path::new(&kattis_samples_output(name)).exists();

    match path {
        false => {
            create_dir_all(kattis_samples_output(name))?;
            Ok(())
        }
        true => {
            eprintln!("The sample files for {} already exists", name);
            Err(From::from("Exiting..."))
        }
    }
}

fn get_kattis_sample(url: &str, id: &str, name: &str) -> Result<(), Box<Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("samples.zip");

    let mut file = File::create(&file_path)?;
    let mut buffer = Vec::new();

    let path: String = format!("{}/{}", url, &kattis_file_path(id));
    let mut response = reqwest::get(&path)?;
    response.read_to_end(&mut buffer)?;

    println!("{}", path);
    println!("Status: {}", response.status());

    create_kattis_folders(name)?;
    file.write_all(&buffer)?;
    unzip(&file_path, &name)?;
    dir.close()?;
    Ok(())
}

fn unzip(file_name: &PathBuf, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fname: &Path = Path::new(&file_name);
    let file: File = File::open(&fname)?;
    let path = kattis_samples_output(name);
    let dir: &Path = Path::new(&path);

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = dir.join(file.sanitized_name());

        println!(
            "File {} extracted to \"{}\" ({} bytes)",
            i + 1,
            out_path.as_path().display(),
            file.size()
        );
        if let Some(p) = out_path.parent() {
            if !p.exists() {
                create_dir_all(&p).unwrap();
            }
        }

        let mut outfile = File::create(&out_path)?;
        copy(&mut file, &mut outfile)?;
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

    match Opt::from_args().cmd {
        Cmd::Get { pid, name, url } => get_kattis_sample(&url, &pid, &name)?,
        Cmd::Test { .. } => test_kattis()?,
    }

    Ok(())
}
