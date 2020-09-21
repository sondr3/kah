use crate::{datafile::Problem, kah::Kah, language::Language};
use anyhow::Result;
use std::path::PathBuf;
use tempfile::tempdir;

#[derive(Debug)]
pub(crate) struct Test {
    pub(crate) problem: Problem,
    pub(crate) temp_dir: PathBuf,
    pub(crate) code_dir: PathBuf,
    pub(crate) verbose: bool,
}

impl Test {
    pub(crate) fn new(kah: &Kah, problem: Problem, verbose: bool) -> Self {
        Test {
            problem,
            temp_dir: tempdir().expect("Could not create temp dir").into_path(),
            code_dir: kah.config.code.clone(),
            verbose,
        }
    }

    pub(crate) async fn run(&mut self) -> Result<()> {
        self.build_problem().await?;
        self.run_tests().await?;

        Ok(())
    }

    async fn build_problem(&self) -> Result<()> {
        self.problem
            .solution
            .language
            .get_language()
            .build(&self)
            .await
    }

    async fn run_tests(&self) -> Result<()> {
        self.problem
            .solution
            .language
            .get_language()
            .run(&self)
            .await
    }
}
