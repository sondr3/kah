use crate::{
    language::{run_problem, Language, LanguageConfig},
    languages::Languages,
    problem::ProblemMetadata,
    test::{Test, TestResult},
};
use anyhow::Result;
use async_trait::async_trait;
use std::{fmt, fmt::Formatter};

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
    async fn build(&self, _: &Test) -> Result<()> {
        Ok(())
    }

    async fn run(&self, test: &Test) -> Result<TestResult> {
        let root = &test.code_dir;
        let file = root.join(
            test.problem
                .solution
                .language
                .get_language()
                .problem_path(&test.problem.metadata),
        );

        run_problem(&self.config.run_command, &file, test).await
    }

    fn config(&self) -> &LanguageConfig {
        &self.config
    }

    fn language_path(&self) -> String {
        self.config.variant.to_string().to_ascii_lowercase()
    }

    fn problem_path(&self, problem: &ProblemMetadata) -> String {
        format!(
            "{}/{}.{}",
            self.config.variant.to_string().to_ascii_lowercase(),
            problem.as_os_str(),
            self.config.extension
        )
    }

    fn initial_problem_content(&self) -> String {
        self.config.variant.initial_problem_content()
    }
}
