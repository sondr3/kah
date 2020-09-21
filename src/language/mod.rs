pub(crate) mod python;

use crate::{languages::Languages, problem::ProblemMetadata, test::Test, test::TestResult};
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::Instant;

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

pub(crate) async fn run_problem(command: &str, file: &PathBuf, test: &Test) -> Result<TestResult> {
    let mut result = TestResult::new();

    for case in &test.problem.metadata.samples {
        let before = Instant::now();
        let mut command = Command::new(command)
            .arg(file)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = command.stdin.as_mut().unwrap();
        stdin.write_all(case.input.as_bytes()).await?;

        let output = command.wait_with_output().await?;
        let after = Instant::now();
        let duration = after - before;
        result.timings.push(duration);
        let stdout = String::from_utf8(output.stdout)?;

        result
            .results
            .push(test.problem.check_output(&case.expected, stdout));
    }

    Ok(result)
}
