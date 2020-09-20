use crate::languages::Languages;
use crate::utils::clean_name;
use crate::{
    datafile::Problem,
    kah::Kah,
    language::{Language, LanguageConfig},
};
use anyhow::Result;
use async_trait::async_trait;
use std::fmt;
use std::fmt::Formatter;
use tokio::process::Command;

#[derive(Debug)]
pub(crate) struct Python {
    pub(crate) config: LanguageConfig,
}

impl fmt::Display for Python {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.config.variant.to_string())
    }
}

impl Default for Python {
    fn default() -> Self {
        Python {
            config: LanguageConfig {
                variant: Languages::Python,
                extension: "py".to_string(),
                compile_command: None,
                run_command: "python3".to_string(),
            },
        }
    }
}

impl Python {
    pub(crate) fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl Language for Python {
    async fn build(&self, _: &Problem) -> Result<()> {
        Ok(())
    }

    async fn run(&self, kah: &Kah, problem: &Problem) -> Result<()> {
        let command = Command::new(&self.config.run_command).arg("");

        Ok(())
    }

    fn configuration(&self) -> &LanguageConfig {
        &self.config
    }

    fn problem_path(&self, name: &str) -> String {
        format!(
            "{}/{}.{}",
            self.config.variant,
            clean_name(name),
            self.config.extension
        )
    }

    fn initial_problem_content(&self) -> String {
        self.config.variant.initial_problem_content()
    }
}
