use crate::kah::Kah;
use reqwest;
use serde_json;
use std::error::Error;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{copy, Read, Write};
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use zip;

fn kattis_file_path(id: &str) -> String {
    format!("problems/{}/file/statement/samples.zip", id)
}

fn kattis_samples_output(name: &str) -> String {
    format!("samples/{}/", name)
}

fn get_kattis_url() -> String {
    let file = File::open(".kah").expect("Could not find .kah");
    let json: Kah = serde_json::from_reader(file).expect("Could not read .kah");

    json.user.hostname
}

fn create_kattis_folders(name: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&kattis_samples_output(name)).exists();

    if path {
        eprintln!("The sample files for {} already exists", name);
        Err(From::from("Exiting..."))
    } else {
        create_dir_all(kattis_samples_output(name))?;
        Ok(())
    }
}

fn unzip(file_name: &PathBuf, name: &str) -> Result<(), Box<dyn Error>> {
    let fname: &Path = Path::new(&file_name);
    let file: File = File::open(&fname)?;
    let path = kattis_samples_output(name);
    let dir: &Path = Path::new(&path);

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = dir.join(file.sanitized_name());

        if let Some(p) = out_path.parent() {
            if !p.exists() {
                create_dir_all(&p).unwrap();
            }
        }

        let mut outfile = File::create(&out_path)?;
        copy(&mut file, &mut outfile)?;
    }

    println!(
        "Wrote {} sample files for {} to {:#?}",
        archive.len(),
        name,
        dir
    );
    remove_file(fname)?;
    Ok(())
}

pub fn get_kattis_sample(id: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("samples.zip");

    let mut file = File::create(&file_path)?;
    let mut buffer = Vec::new();

    let url = get_kattis_url();

    let path: String = format!("{}{}", url, &kattis_file_path(id));
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
