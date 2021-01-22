use crate::utils::{average_duration, max_duration, min_duration};
use crate::{datafile::Problem, kah::Kah};
use anyhow::Result;
use std::{path::PathBuf, time::Duration};
use tempfile::tempdir;

#[derive(Debug)]
pub(crate) struct Test {
    pub(crate) problem: Problem,
    pub(crate) temp_dir: PathBuf,
    pub(crate) code_dir: PathBuf,
    pub(crate) verbose: bool,
}

#[derive(Debug)]
pub(crate) struct TestResult {
    pub(crate) timings: Vec<Duration>,
    pub(crate) results: Vec<bool>,
}

impl TestResult {
    pub(crate) fn new() -> Self {
        TestResult {
            timings: Vec::new(),
            results: Vec::new(),
        }
    }

    pub(crate) fn report(&self, test: &Test) {
        println!("{: <10} {: <10} {: <10}", "Case", "Result", "Time");
        println!("{: <10} {: <10} {: <10}", "----", "------", "----");

        for (num, (result, timing)) in self.results.iter().zip(self.timings.iter()).enumerate() {
            println!(
                "#{: <10}{: <10} {}ms",
                num + 1,
                if *result { "OK" } else { "FAIL" },
                timing.as_millis()
            );
        }

        if test.verbose {
            println!(
                "\n{: <10} {: <10} {: <10}",
                "Avg time", "Min time", "Max time"
            );
            println!(
                "{: <10} {: <10} {: <10}",
                "--------", "--------", "--------"
            );
            println!(
                "{: <10} {: <10} {: <10}",
                average_duration(&self.timings, test.problem.metadata.samples.len()),
                min_duration(&self.timings),
                max_duration(&self.timings),
            )
        }
    }
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

    pub(crate) fn run(&mut self) -> Result<()> {
        self.build_problem()?;
        let result = self.run_tests()?;
        result.report(self);

        Ok(())
    }

    fn build_problem(&self) -> Result<()> {
        self.problem.solution.language.build()
    }

    fn run_tests(&self) -> Result<TestResult> {
        self.problem.solution.language.run(&self)
    }
}
