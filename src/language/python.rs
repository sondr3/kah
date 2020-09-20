use crate::languages::Languages;
use crate::problem::ProblemMetadata;
use crate::{
    datafile::Problem,
    kah::Kah,
    language::{Language, LanguageConfig},
};
use anyhow::Result;
use async_trait::async_trait;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::process::Stdio;
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
        let root = &kah.config.code;
        let file = root.join(
            problem
                .solution
                .language
                .get_language()
                .problem_path(&problem.metadata),
        );

        for (num, case) in problem.metadata.samples.iter().enumerate() {
            print!("Running test case #{}: ", num + 1);

            let command = Command::new(&self.config.run_command)
                .arg(&file)
                .stdin(File::open(&case.input_path)?)
                .stderr(Stdio::inherit())
                .output()
                .await?;

            let expected = tokio::fs::read_to_string(&case.expected_path).await?;
            let correct = problem.check_output(expected, String::from_utf8(command.stdout)?);

            println!("{}", if correct { "OK" } else { "FAIL" });
        }

        Ok(())
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
