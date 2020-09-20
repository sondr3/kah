pub(crate) mod python;

use crate::{datafile::Problem, kah::Kah, languages::Languages, problem::ProblemMetadata};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Language {
    async fn build(&self, problem: &Problem) -> Result<()>;
    async fn run(&self, kah: &Kah, problem: &Problem) -> Result<()>;
    fn config(&self) -> &LanguageConfig;
    fn language_path(&self) -> String;
    fn problem_path(&self, problem: &ProblemMetadata) -> String;
    fn initial_problem_content(&self) -> String;
}

#[derive(Debug)]
pub struct LanguageConfig {
    pub(crate) variant: Languages,
    pub(crate) extension: String,
    pub(crate) compile_command: Option<String>,
    pub(crate) run_command: String,
}
