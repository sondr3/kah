use crate::language::problem_path;
use crate::{
    language::run_problem,
    test::{Test, TestResult},
};
use anyhow::Result;
use std::process::{Command, Stdio};

pub(crate) fn build_cpp() -> Result<()> {
    Command::new("cmake --build .")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(())
}

pub(crate) fn run_cpp(test: &Test) -> Result<TestResult> {
    let root = &test.code_dir;
    let language = &test.problem.solution.language;
    let path = problem_path(language, &test.problem.metadata);
    let file = root.join(path);

    run_problem("./", &file, test)
}
