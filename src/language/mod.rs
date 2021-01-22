pub(crate) mod cpp;
pub(crate) mod python;

use crate::{languages::Languages, problem::ProblemMetadata, test::Test, test::TestResult};
use anyhow::Result;
use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::Instant,
};

pub(crate) fn problem_path(lang: &Languages, problem: &ProblemMetadata) -> String {
    format!(
        "{}/{}.{}",
        lang.language_path(),
        problem.as_os_str(),
        lang.extension()
    )
}

pub(crate) fn run_problem(command: &str, file: &PathBuf, test: &Test) -> Result<TestResult> {
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
        stdin.write_all(case.input.as_bytes())?;

        let output = command.wait_with_output()?;
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
