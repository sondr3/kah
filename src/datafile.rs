use crate::error::KahError::FileDoesNotExist;
use crate::language::Language;
use crate::problem::ProblemMetadata;
use crate::utils::kattis_sample_directory;
use anyhow::Result;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::exit;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    solution: PathBuf,
    samples: PathBuf,
    language: String,
    solved: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    metadata: ProblemMetadata,
    solution: Solution,
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})\nCPU: {}, MEM: {}, DIF: {}",
            self.metadata.name,
            self.metadata.id,
            self.metadata.cpu_time_limit,
            self.metadata.memory_limit,
            self.metadata.difficulty
        )
    }
}

impl PartialEq for Problem {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.id == other.metadata.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datafile {
    problems: HashMap<String, Problem>,
}

impl Datafile {
    pub(crate) async fn create(force: bool) -> Result<()> {
        let path = Path::new(".kahdata");

        if path.exists() && !force {
            eprintln!("Data file already exists.");
            exit(0);
        } else {
            let mut file = File::create(path).await?;
            let json = serde_json::to_string_pretty::<Datafile>(&Datafile {
                problems: HashMap::new(),
            })?;
            file.write_all(&json.into_bytes()).await?;
        }

        println!("Data file successfully created");

        Ok(())
    }

    pub(crate) async fn add_problem(
        &mut self,
        problem: &ProblemMetadata,
        language: &Language,
        force: bool,
    ) -> Result<()> {
        if self.problems.contains_key(&problem.id) && !force {
            eprintln!("Problem {} already exists, aborting", problem.name);
            exit(1);
        }

        self.problems.insert(
            problem.id.clone(),
            Problem {
                metadata: problem.clone(),
                solution: Solution {
                    solution: PathBuf::from(language.problem_path(&problem.name)),
                    samples: PathBuf::from(kattis_sample_directory(&problem.name)),
                    language: language.to_string(),
                    solved: false,
                },
            },
        );

        println!("{:?}", self);
        self.write().await?;

        Ok(())
    }

    pub(crate) async fn get_datafile() -> Result<Datafile> {
        let path = Path::new(".kahdata");

        if !path.exists() {
            return Err(FileDoesNotExist(path.to_path_buf()).into());
        }

        let mut file = File::open(path).await?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await?;
        let datafile = serde_json::from_str(&buffer)?;

        Ok(datafile)
    }

    pub(crate) fn get_problem(&self, id: &str) -> Option<&Problem> {
        self.problems
            .values()
            .find(|p| p.metadata.id.contains(id) || p.metadata.name.contains(id))
    }

    async fn write(&self) -> Result<()> {
        let path = Path::new(".kahdata");
        let json = serde_json::to_string_pretty(self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .append(false)
            .open(&path)
            .await?;

        file.write_all(&json.into_bytes()).await?;

        Ok(())
    }
}
