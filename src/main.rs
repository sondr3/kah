mod datafile;
mod error;
mod kah;
mod language;
mod languages;
mod problem;
mod test;
mod utils;

use crate::{
    error::KahError::{self, ForceProblemCreationError, NoSuchProblem},
    kah::Kah,
    languages::Languages,
    problem::ProblemMetadata,
    test::Test,
};
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{convert::TryFrom, path::PathBuf, str::FromStr};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "kah", about = "a simple Kattis helper utility", global_settings(& [AppSettings::ColoredHelp]))]
pub struct Opt {
    #[structopt(short, long)]
    /// Verbose messages
    verbose: bool,
    #[structopt(short, long)]
    /// Overwrite existing files
    force: bool,
    #[structopt(subcommand)]
    pub cmd: Cmd,
}

#[derive(StructOpt, PartialEq, Debug)]
pub enum Cmd {
    #[structopt(name = "problem", alias = "p")]
    /// Get sample test files from Kattis
    Problem {
        /// Problem ID
        id: String,
        #[structopt(short, parse(from_occurrences))]
        /// Force creation of problem files, a single `-f` will recreate the
        /// samples, `-ff` will recreate the .kahdata entry and `-fff` will
        /// recreate everything including the solution
        force: u64,
    },

    #[structopt(name = "test", alias = "t")]
    /// Run tests for a Kattis problem locally
    Test {
        /// Kattis problem to test
        problem_id: String,
        #[structopt(short, long)]
        /// Run tests verbosely (e.g. print output, stderr)
        verbose: bool,
    },

    #[structopt(name = "submit", alias = "s")]
    /// Submit your solution to a Kattis problem
    Submit {
        /// Kattis problem to submit
        file: String,
        #[structopt(short, long)]
        /// Select language for problem
        language: Option<String>,
    },

    #[structopt(name = "info", alias = "i")]
    /// Show information about a problem and its solution
    Info {
        /// Problem ID
        problem: String,
    },

    #[structopt(name = "init")]
    /// Fetch user configuration file
    Init {
        #[structopt(parse(from_os_str))]
        /// URL to fetch files from
        file: PathBuf,
        #[structopt(short, long)]
        /// Force creation of config file
        force: bool,
    },

    #[structopt(name = "update", alias = "u")]
    Update,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum ForceProblemCreation {
    Nothing,
    Samples,
    SamplesMetadata,
    SamplesMetadataSolution,
}

impl ForceProblemCreation {
    pub(crate) fn recreate_metadata(&self) -> bool {
        match self {
            ForceProblemCreation::Nothing | ForceProblemCreation::Samples => false,
            ForceProblemCreation::SamplesMetadata
            | ForceProblemCreation::SamplesMetadataSolution => true,
        }
    }

    pub(crate) fn recreate_solution(&self) -> bool {
        match self {
            ForceProblemCreation::Nothing
            | ForceProblemCreation::Samples
            | ForceProblemCreation::SamplesMetadata => false,
            ForceProblemCreation::SamplesMetadataSolution => true,
        }
    }
}

impl TryFrom<u64> for ForceProblemCreation {
    type Error = KahError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ForceProblemCreation::Nothing),
            1 => Ok(ForceProblemCreation::Samples),
            2 => Ok(ForceProblemCreation::SamplesMetadata),
            3 => Ok(ForceProblemCreation::SamplesMetadataSolution),
            _ => Err(ForceProblemCreationError(value)),
        }
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.cmd {
        Cmd::Problem { id, force } => create_problem(&id, ForceProblemCreation::try_from(force)?)?,
        Cmd::Test {
            problem_id,
            verbose,
        } => {
            let kah = Kah::get()?;
            let problem = match kah.get_problem(&problem_id) {
                Some(x) => Ok(x),
                None => Err(NoSuchProblem(problem_id)),
            }?;
            let mut test = Test::new(&kah, problem, verbose);
            test.run()?;
        }
        Cmd::Submit { .. } => println!("You are submitting something!"),
        Cmd::Info { problem } => {
            let kah = Kah::get()?;
            let problem = match kah.get_problem(&problem) {
                Some(problem) => problem,
                None => return Err(NoSuchProblem(problem).into()),
            };

            println!("{}", problem);
        }
        Cmd::Init { file, force } => {
            Kah::new(file, force)?;
        }
        Cmd::Update => {
            let mut kah = Kah::get()?;
            kah.update()?;

            println!("Successfully updated data");
        }
    }

    Ok(())
}

fn create_problem(problem_id: &str, force: ForceProblemCreation) -> Result<()> {
    let languages = &["Rust", "Kotlin", "Java", "Python", "Haskell"];
    let language = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a language to solve problem in")
        .items(&languages[..])
        .interact()?;

    let language = Languages::from_str(languages[language])?;
    let problem = ProblemMetadata::new(problem_id)?;
    let mut kah = Kah::get()?;

    kah.create_problem(&problem, language, force)?;

    Ok(())
}
