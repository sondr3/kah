use crate::{
    language::problem_path,
    language::run_problem,
    test::{Test, TestResult},
};
use anyhow::Result;

pub(crate) fn run_python(test: &Test) -> Result<TestResult> {
    let root = &test.code_dir;

    let language = &test.problem.solution.language;
    let path = problem_path(language, &test.problem.metadata);
    let file = root.join(path);

    run_problem("python3", &file, test)
}
