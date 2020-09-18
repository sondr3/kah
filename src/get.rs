use crate::{kattis::Kattis, problem::Problem};
use anyhow::Result;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{copy, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use tempfile::tempdir;

fn kattis_file_path(id: &str) -> String {
    format!("problems/{}/file/statement/samples.zip", id)
}

fn kattis_samples_output(name: &str) -> String {
    format!("samples/{}/", name)
}

fn create_kattis_folders(name: &str) -> Result<()> {
    let path = Path::new(&kattis_samples_output(name)).exists();

    // TODO: Add force flag to function
    if path {
        println!("Samples already exist, skipping...");
        exit(0);
    } else {
        create_dir_all(kattis_samples_output(name))?;
        Ok(())
    }
}

fn unzip(file_name: &PathBuf, name: &str) -> Result<()> {
    let fname: &Path = Path::new(&file_name);
    let file: File = File::open(&fname)?;
    let path = kattis_samples_output(name);
    let dir: &Path = Path::new(&path);

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = dir.join(file.name());

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

pub async fn get_kattis_sample(id: &str, name: &str) -> Result<()> {
    Problem::get_details(id).await?;
    let dir = tempdir()?;
    let file_path = dir.path().join("samples.zip");

    let mut file = File::create(&file_path)?;

    let url = Kattis::get_kattis_url();

    let path: String = format!("{}/{}", url, &kattis_file_path(id));
    let response = reqwest::get(&path).await?;

    println!("{}", path);
    println!("Status: {}", response.status());

    create_kattis_folders(name)?;
    file.write_all(&response.bytes().await?)?;
    unzip(&file_path, &name)?;
    dir.close()?;
    Ok(())
}
