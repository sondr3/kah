pub(crate) mod python;

use crate::languages::Languages;
use crate::{datafile::Problem, kah::Kah};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Language {
    async fn build(&self, problem: &Problem) -> Result<()>;
    async fn run(&self, kah: &Kah, problem: &Problem) -> Result<()>;
    fn configuration(&self) -> &LanguageConfig;
    fn problem_path(&self, name: &str) -> String;
    fn initial_problem_content(&self) -> String;
}

#[derive(Debug)]
pub struct LanguageConfig {
    pub(crate) variant: Languages,
    pub(crate) extension: String,
    pub(crate) compile_command: Option<String>,
    pub(crate) run_command: String,
}
