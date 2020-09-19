use crate::{
    error::KahError::{FetchError, ScrapeError},
    kattis::Kattis,
    utils::*,
};
use anyhow::Result;
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};
use serde::{Deserialize, Serialize};
use std::{path::Path, process::exit};
use tempfile::tempdir;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemMetadata {
    pub(crate) name: String,
    pub(crate) id: String,
    pub(crate) cpu_time_limit: String,
    pub(crate) memory_limit: String,
    pub(crate) difficulty: f32,
}

impl ProblemMetadata {
    pub async fn new(id: &str, force: bool) -> Result<ProblemMetadata> {
        let problem = ProblemMetadata::get(id).await?;

        println!("Found problem {}, fetching samples", problem.name);
        problem.get_sample_files(force).await?;

        Ok(problem)
    }

    pub async fn get(id: &str) -> Result<ProblemMetadata> {
        let url = Kattis::get_kattis_url();
        let path: String = format!("{}/problems/{}", url, id);
        let response = reqwest::get(&path).await?;

        let body = match response.error_for_status() {
            Ok(resp) => resp.text().await?,
            Err(err) => return Err(FetchError(id.to_string(), err.to_string()).into()),
        };

        let document = Document::from(&body[..]);
        let title = document
            .find(Class("headline-wrapper").descendant(Name("h1")))
            .next()
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find title".to_string()))?
            .text();

        let name = title;

        let sidebar = document
            .find(Class("problem-download"))
            .next()
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find problem download".into()))?
            .parent()
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find sidebar".to_string()))?
            .find(Name("p"))
            .collect::<Vec<_>>();

        let cpu_time_limit = sidebar[1]
            .children()
            .nth(1)
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find CPU time limit".into()))?
            .text();
        let memory_limit = sidebar[2]
            .children()
            .nth(1)
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find memory limit".to_string()))?
            .text();
        let difficulty: f32 = sidebar[3]
            .find(Name("span"))
            .next()
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find difficulty".to_string()))?
            .text()
            .parse()?;

        Ok(ProblemMetadata {
            id: id.into(),
            name: name.trim().to_string(),
            cpu_time_limit: cpu_time_limit.trim().to_string(),
            memory_limit: memory_limit.trim().to_string(),
            difficulty,
        })
    }

    async fn get_sample_files(&self, force: bool) -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("samples.zip");

        let mut temp_file = File::create(&file_path).await?;

        let url = Kattis::get_kattis_url();

        let path: String = sample_files_url(url, &self.id);
        let response = reqwest::get(&path).await?;

        self.create_sample_folder(force).await?;
        temp_file.write_all(&response.bytes().await?).await?;
        unzip(&file_path, &self.name).await?;
        temp_dir.close()?;
        Ok(())
    }

    async fn create_sample_folder(&self, force: bool) -> Result<()> {
        let path = Path::new(&kattis_sample_directory(&self.name)).exists();

        if path && !force {
            println!("Samples already exist, skipping...");
            exit(0);
        } else {
            tokio::fs::create_dir_all(kattis_sample_directory(&self.name)).await?;
            Ok(())
        }
    }
}
