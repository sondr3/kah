pub(crate) mod python;

use crate::{languages::Languages, problem::ProblemMetadata, test::Test, test::TestResult};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Language {
    async fn build(&self, test: &Test) -> Result<()>;
    async fn run(&self, test: &Test) -> Result<TestResult>;
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
