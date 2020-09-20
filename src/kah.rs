use crate::{error::KahError::KattisrcParseError, language::Language, ForceProblemCreation};
use anyhow::Result;
use directories::ProjectDirs;
use ini::Ini;
use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    fmt::Display,
    path::{Path, PathBuf},
};
use tokio::{fs::File, io::AsyncWriteExt};

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
    pub(crate) async fn new(kattisrc_path: PathBuf, force: bool) -> Result<Self> {
        let config_dir = Kah::project_dir().config_dir().to_owned();

        let kah = Kah {
            config: KahConfig {
                code: current_dir()?,
                dir: config_dir.to_path_buf(),
                file: config_dir.join("config.json"),
                data: config_dir.join("data.json"),
            },
            kattis: match Kah::parse_kattisrc(kattisrc_path.clone()) {
                Some(x) => Ok(x),
                None => Err(KattisrcParseError),
            }?,
        };

        kah.create_config_dir(force).await?;
        if !kah.kattisrc_exists() {
            tokio::fs::copy(&kattisrc_path, config_dir.join("kattisrc")).await?;
            tokio::fs::remove_file(&kattisrc_path).await?;
        }

        kah.create_config_file().await?;
        kah.create_datafile(force).await?;

        println!("Successfully created configuration");

        Ok(kah)
    }

    pub(crate) async fn get() -> Result<Self> {
        let path = Kah::project_dir().config_dir().join("config.json");
        let file = tokio::fs::read_to_string(path).await?;

        let result = serde_json::from_str(&file)?;
        Ok(result)
    }

    pub(crate) fn get_kattis_url(&self) -> String {
        self.kattis.hostname.to_string()
    }

    pub(crate) async fn create_problem<T: Language + Display>(
        &self,
        name: &str,
        language: &T,
        force: ForceProblemCreation,
    ) -> Result<()> {
        let code = language.initial_problem_content();
        let path = language.problem_path(name);

        if !Path::new(&language.to_string()).exists() {
            tokio::fs::create_dir_all(language.to_string()).await?;
        }

        let path = Path::new(&path);
        if path.exists() && !force.recreate_solution() {
            eprintln!(
                "{} already exists for language {}",
                name,
                language.to_string()
            )
        } else {
            let mut file = File::create(path).await?;
            file.write_all(code.as_bytes()).await?;
        }

        println!("Created {} in {}", name, language.to_string());

        Ok(())
    }

    async fn create_config_file(&self) -> Result<()> {
        let mut file = File::create(&self.config.file).await?;

        let json = serde_json::to_string_pretty(self)?;
        let buffer = json.into_bytes();

        file.write_all(&buffer).await?;

        Ok(())
    }

    fn config_exists(&self) -> bool {
        self.config.dir.exists() || self.config.file.exists()
    }

    fn kattisrc_exists(&self) -> bool {
        self.config.dir.join("kattisrc").exists()
    }

    async fn create_config_dir(&self, force: bool) -> Result<()> {
        if !self.config_exists() && !force {
            tokio::fs::create_dir_all(&self.config.dir).await?;
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
