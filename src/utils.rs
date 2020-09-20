use anyhow::Result;
use std::io::copy;
use std::path::{Path, PathBuf};
use tokio::fs::File;

pub(crate) async fn unzip(file_name: &PathBuf, name: &str) -> Result<Vec<String>> {
    let fname: &Path = Path::new(&file_name);
    let file = File::open(&fname).await?.into_std().await;
    let path = kattis_sample_directory(name);
    let dir: &Path = Path::new(&path);

    let mut archive = zip::ZipArchive::new(file)?;

    let mut file_names = Vec::with_capacity(archive.len());

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = dir.join(file.name());
        file_names.push(file.name().to_string());

        if let Some(p) = out_path.parent() {
            if !p.exists() {
                tokio::fs::create_dir_all(&p).await?;
            }
        }

        let mut outfile = File::create(&out_path).await?.into_std().await;
        copy(&mut file, &mut outfile)?;
    }

    println!(
        "Wrote {} sample files for {} to {:#?}",
        archive.len(),
        name,
        dir
    );

    tokio::fs::remove_file(fname).await?;
    Ok(file_names)
}

pub(crate) fn sample_files_url(host: String, id: &str) -> String {
    format!("{}/problems/{}/file/statement/samples.zip", host, id)
}

pub(crate) fn kattis_sample_directory(name: &str) -> String {
    format!("samples/{}/", clean_name(name))
}

pub(crate) fn clean_name(name: impl Into<String>) -> String {
    name.into()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

pub(crate) trait TitleCase {
    fn title_case(&self) -> String;
}

impl TitleCase for String {
    fn title_case(&self) -> String {
        let (first, rest) = self.split_at(1);
        first.to_uppercase() + rest
    }
}
