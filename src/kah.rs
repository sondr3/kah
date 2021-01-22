use crate::language::problem_path;
use crate::languages::Languages;
use crate::{error::KahError::KattisrcParseError, problem::ProblemMetadata, ForceProblemCreation};
use anyhow::Result;
use directories::ProjectDirs;
use ini::Ini;
use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    fs,
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Kattis {
    pub(crate) username: String,
    pub(crate) token: String,
    pub(crate) hostname: String,
    pub(crate) submit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct KahConfig {
    pub(crate) code: PathBuf,
    pub(crate) dir: PathBuf,
    pub(crate) file: PathBuf,
    pub(crate) data: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Kah {
    pub(crate) config: KahConfig,
    pub(crate) kattis: Kattis,
}

impl Kah {
    pub(crate) fn new(kattisrc_path: PathBuf, force: bool) -> Result<Self> {
        let config_dir = Kah::project_dir().config_dir().to_owned();

        let kah = Kah {
            config: KahConfig {
                code: current_dir()?,
                dir: config_dir.clone(),
                file: config_dir.join("config.json"),
                data: config_dir.join("data.json"),
            },
            kattis: match Kah::parse_kattisrc(kattisrc_path.clone()) {
                Some(x) => Ok(x),
                None => Err(KattisrcParseError),
            }?,
        };

        kah.create_config_dir(force)?;
        if !kah.kattisrc_exists() {
            fs::copy(&kattisrc_path, config_dir.join("kattisrc"))?;
            fs::remove_file(&kattisrc_path)?;
        }

        kah.create_config_file()?;
        kah.create_datafile(force)?;

        println!("Successfully created configuration");

        Ok(kah)
    }

    pub(crate) fn get() -> Result<Self> {
        let path = Kah::project_dir().config_dir().join("config.json");
        let file = read_to_string(path)?;

        let result = serde_json::from_str(&file)?;
        Ok(result)
    }

    pub(crate) fn get_kattis_url(&self) -> String {
        self.kattis.hostname.to_string()
    }

    pub(crate) fn create_problem(
        &mut self,
        problem: &ProblemMetadata,
        language: Languages,
        force: ForceProblemCreation,
    ) -> Result<()> {
        let code = language.initial_problem_content();
        let path = problem_path(&language, &problem);

        let language_folder = &language.language_path();

        if !Path::new(language_folder).exists() {
            fs::create_dir_all(language_folder)?;
        }

        let path = Path::new(&path);
        if path.exists() && !force.recreate_solution() {
            eprintln!(
                "{} already exists for language {}, skipping code creation",
                problem.name,
                language.to_string()
            );
        } else {
            let mut file = File::create(path)?;
            file.write_all(code.as_bytes())?;
        }

        println!("Created {} in {}", problem.name, language.to_string());

        self.add_problem(problem, &language, force)?;

        Ok(())
    }

    fn create_config_file(&self) -> Result<()> {
        let mut file = File::create(&self.config.file)?;

        let json = serde_json::to_string_pretty(self)?;
        let buffer = json.into_bytes();

        file.write_all(&buffer)?;

        Ok(())
    }

    fn config_exists(&self) -> bool {
        self.config.dir.exists() || self.config.file.exists()
    }

    fn kattisrc_exists(&self) -> bool {
        self.config.dir.join("kattisrc").exists()
    }

    fn create_config_dir(&self, force: bool) -> Result<()> {
        if !self.config_exists() && !force {
            fs::create_dir_all(&self.config.dir)?;
        }

        Ok(())
    }

    fn project_dir() -> ProjectDirs {
        ProjectDirs::from("com", "Sondre Nilsen", "kah")
            .expect("Could not create project dir identifier")
    }

    fn parse_kattisrc(path: PathBuf) -> Option<Kattis> {
        let file = Ini::load_from_file(path).ok()?;

        let kattisrc = {
            let user_section = file.section(Some("user"))?;
            let kattis_section = file.section(Some("kattis"))?;

            let username = user_section.get("username")?;
            let token = user_section.get("token")?;

            let submit = kattis_section.get("submissionurl")?;
            let hostname = kattis_section.get("hostname")?;
            let hostname = format!("{}{}", "https://", hostname);

            Some(Kattis {
                username: username.into(),
                token: token.into(),
                hostname,
                submit: submit.to_owned(),
            })
        };

        kattisrc
    }
}
