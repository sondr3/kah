use crate::{kah::Kah, languages::Languages, problem::ProblemMetadata, ForceProblemCreation};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Formatter,
    fs::{read_to_string, File, OpenOptions},
    io::Write,
    process::exit,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Solution {
    pub(crate) language: Languages,
    pub(crate) solved: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Problem {
    pub(crate) metadata: ProblemMetadata,
    pub(crate) solution: Solution,
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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

impl Problem {
    pub(crate) fn check_output(&self, expected: &str, output: String) -> bool {
        let expected: String = expected.trim_end().lines().map(|s| s.trim_end()).collect();
        let actual: String = output.trim_end().lines().map(|s| s.trim_end()).collect();

        actual == expected
    }
}

impl Kah {
    pub(crate) fn create_datafile(&self, force: bool) -> Result<()> {
        if self.datafile_exists() && !force {
            eprintln!("Data file already exists.");
            exit(0);
        } else {
            let mut file = File::create(&self.config.data)?;
            let map: HashMap<String, Problem> = HashMap::new();
            let json = serde_json::to_string_pretty(&map)?;
            file.write_all(&json.into_bytes())?;
        }

        println!("Data file successfully created");

        Ok(())
    }

    pub(crate) fn add_problem(
        &mut self,
        problem: &ProblemMetadata,
        language: &Languages,
        force: ForceProblemCreation,
    ) -> Result<()> {
        let mut problems = self.open_datafile()?;
        if problems.contains_key(&problem.id) && !force.recreate_metadata() {
            eprintln!("Datafile already contains {}, aborting", problem.name);
            exit(1);
        }

        problems.insert(
            problem.id.clone(),
            Problem {
                metadata: problem.clone(),
                solution: Solution {
                    language: language.clone(),
                    solved: false,
                },
            },
        );

        self.write_datafile(&problems)?;

        Ok(())
    }

    pub(crate) fn get_problem(&self, id: &str) -> Option<Problem> {
        let problems = self.open_datafile().ok()?;

        problems
            .values()
            .cloned()
            .find(|p| p.metadata.id.contains(id) || p.metadata.name.contains(id))
    }

    pub(crate) fn update(&mut self) -> Result<()> {
        Ok(())
    }

    pub(crate) fn datafile_exists(&self) -> bool {
        self.config.data.exists()
    }

    fn open_datafile(&self) -> Result<HashMap<String, Problem>> {
        if !self.datafile_exists() {
            self.write_datafile(&HashMap::new())?;
        }

        let file = read_to_string(&self.config.data)?;
        let result = serde_json::from_str(&file)?;

        Ok(result)
    }

    fn write_datafile(&self, datafile: &HashMap<String, Problem>) -> Result<()> {
        let json = serde_json::to_string_pretty(&datafile)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(&self.config.data)?;

        file.write_all(&json.into_bytes())?;

        Ok(())
    }
}
