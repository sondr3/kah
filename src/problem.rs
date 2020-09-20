use crate::{
    error::KahError::{FetchError, ScrapeError},
    kah::Kah,
    utils::*,
    ForceProblemCreation,
};
use anyhow::Result;
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};
use serde::{Deserialize, Serialize};
use std::{env::current_dir, path::Path, path::PathBuf, process::exit};
use tempfile::tempdir;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sample {
    input_path: PathBuf,
    expected_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemMetadata {
    pub(crate) name: String,
    pub(crate) id: String,
    pub(crate) cpu_time_limit: String,
    pub(crate) memory_limit: String,
    pub(crate) difficulty: f32,
    pub(crate) samples: Vec<Sample>,
}

impl ProblemMetadata {
    pub(crate) async fn new(id: &str, force: ForceProblemCreation) -> Result<ProblemMetadata> {
        let mut problem = ProblemMetadata::get(id).await?;

        println!("Found problem {}, fetching data", problem.name);
        let samples = problem.get_sample_files(force).await?;
        problem.build_samples(samples)?;

        Ok(problem)
    }

    pub async fn get(id: &str) -> Result<ProblemMetadata> {
        let url = Kah::get().await?.get_kattis_url();
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
            samples: Vec::new(),
        })
    }

    async fn get_sample_files(&self, force: ForceProblemCreation) -> Result<Vec<String>> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("samples.zip");

        let mut temp_file = File::create(&file_path).await?;

        let url = Kah::get().await?.get_kattis_url();

        let path: String = sample_files_url(url, &self.id);
        let response = reqwest::get(&path).await?;

        self.create_sample_folder(force).await?;
        temp_file.write_all(&response.bytes().await?).await?;
        let files = unzip(&file_path, &self.name).await?;
        temp_dir.close()?;
        Ok(files)
    }

    async fn create_sample_folder(&self, force: ForceProblemCreation) -> Result<()> {
        let path = Path::new(&kattis_sample_directory(&self.name)).exists();

        if path && !force.recreate_samples() {
            println!("Samples already exist, skipping...");
            exit(0);
        } else {
            tokio::fs::create_dir_all(kattis_sample_directory(&self.name)).await?;
            Ok(())
        }
    }

    fn build_samples(&mut self, samples: Vec<String>) -> Result<()> {
        let mut inputs: Vec<_> = samples
            .iter()
            .filter(|s| s.ends_with("in"))
            .map(|s| s.strip_suffix(".in").unwrap())
            .collect();
        let mut outputs: Vec<_> = samples
            .iter()
            .filter(|s| s.ends_with("ans"))
            .map(|s| s.strip_suffix(".ans").unwrap())
            .collect();
        inputs.sort_unstable();
        outputs.sort_unstable();

        println!("{:?}", inputs);
        println!("{:?}", outputs);

        let cwd = current_dir()?;
        let path = PathBuf::from(kattis_sample_directory(&self.name));
        self.samples = inputs
            .iter()
            .zip(outputs.iter())
            .map(|(i, o)| Sample {
                input_path: cwd.join(path.join(format!("{}.in", i))),
                expected_path: cwd.join(path.join(format!("{}.ans", o))),
            })
            .collect();

        Ok(())
    }
}
