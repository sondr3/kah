use crate::{datafile::Problem, kah::Kah, language::Language};
use anyhow::Result;

impl Kah {
    pub(crate) async fn test_problem(&self, problem: &Problem) -> Result<()> {
        self.build_problem(problem).await?;
        self.run_tests(problem).await?;

        Ok(())
    }

    async fn build_problem(&self, problem: &Problem) -> Result<()> {
        problem
            .solution
            .language
            .get_language()
            .build(problem)
            .await
    }

    async fn run_tests(&self, problem: &Problem) -> Result<()> {
        problem
            .solution
            .language
            .get_language()
            .run(&self, problem)
            .await
    }
}
