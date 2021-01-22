use crate::{
    error::KahError::{FetchError, ScrapeError},
    kah::Kah,
    utils::*,
};
use anyhow::Result;
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use tempfile::tempdir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Sample {
    pub(crate) input: String,
    pub(crate) expected: String,
}

impl Default for Sample {
    fn default() -> Self {
        Sample {
            input: "".to_string(),
            expected: "".to_string(),
        }
    }
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
    pub(crate) fn new(id: &str) -> Result<ProblemMetadata> {
        let mut problem = ProblemMetadata::get(id)?;

        println!("Found problem {}, fetching data", problem.name);
        problem.get_samples()?;

        Ok(problem)
    }

    pub fn get(id: &str) -> Result<ProblemMetadata> {
        let url = Kah::get()?.get_kattis_url();
        let path: String = format!("{}/problems/{}", url, id);
        let resp = reqwest::blocking::get(&path)?;

        let body = match resp.error_for_status() {
            Ok(resp) => resp.text()?,
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

    fn get_samples(&mut self) -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("samples.zip");

        let mut temp_file = File::create(&file_path)?;

        let url = Kah::get()?.get_kattis_url();

        let path: String = self.sample_files_url(url);
        let response = reqwest::blocking::get(&path)?;

        temp_file.write_all(&response.bytes()?)?;
        self.samples = unzip(&file_path)?;
        temp_dir.close()?;
        Ok(())
    }

    fn sample_files_url(&self, url: String) -> String {
        format!("{}/problems/{}/file/statement/samples.zip", url, self.id)
    }

    pub(crate) fn as_os_str(&self) -> String {
        self.name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect()
    }
}
