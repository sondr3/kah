use crate::kattis::Kattis;
use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize)]
struct KahConfig {
    config_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Kah {
    config: KahConfig,
    kattis: Kattis,
}

impl Kah {
    pub(crate) async fn new(kattisrc_path: PathBuf, force: bool) -> Result<Self> {
        let dir = ProjectDirs::from("com", "Sondre Nilsen", "kah")
            .expect("Could not create project dir identifier");
        let config_dir = dir.config_dir();

        println!("{:?}", kattisrc_path);

        let kah = Kah {
            config: KahConfig {
                config_dir: config_dir.to_path_buf(),
            },
            kattis: Kattis::new(&kattisrc_path),
        };

        kah.create_config_dir(force).await?;
        if !kah.kattisrc_exists() {
            tokio::fs::copy(&kattisrc_path, config_dir.join("kattisrc")).await?;
            tokio::fs::remove_file(&kattisrc_path).await?;
        }

        kah.create_config_file().await?;

        Ok(kah)
    }

    async fn create_config_file(&self) -> Result<()> {
        let mut file = File::create(self.config.config_dir.join("config.json")).await?;

        let json = serde_json::to_string_pretty(self)?;
        let buffer = json.into_bytes();

        file.write_all(&buffer).await?;

        Ok(())
    }

    fn config_exists(&self) -> bool {
        self.config.config_dir.exists() || self.config.config_dir.join("config.json").exists()
    }

    fn kattisrc_exists(&self) -> bool {
        self.config.config_dir.join("kattisrc").exists()
    }

    async fn create_config_dir(&self, force: bool) -> Result<()> {
        if !self.config_exists() && !force {
            tokio::fs::create_dir_all(&self.config.config_dir).await?;
        }

        Ok(())
    }
}
