mod datafile;
mod error;
mod init;
mod kattis;
mod languages;
mod problem;
mod test;
mod utils;

use crate::datafile::Datafile;
use crate::init::create_kah_dotfile;
use crate::kattis::Kattis;
use crate::languages::Language;
use crate::problem::ProblemMetadata;
use crate::test::test_kattis;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::str::FromStr;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(
    name = "kah",
    about = "a simple Kattis helper utility",
    global_settings(& [AppSettings::ColoredHelp])
)]
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
    #[structopt(name = "get")]
    /// Get sample test files from Kattis
    Get {
        /// Problem ID
        pid: String,
        #[structopt(short, long)]
        /// Force creation of sample files
        force: bool,
    },

    #[structopt(name = "test")]
    /// Run tests for a Kattis problem locally
    Test {
        /// Kattis problem to test
        file: String,
        #[structopt(short, long)]
        /// Select language for problem
        language: Option<String>,
    },

    #[structopt(name = "submit")]
    /// Submit your solution to a Kattis problem
    Submit {
        /// Kattis problem to submit
        file: String,
        #[structopt(short, long)]
        /// Select language for problem
        language: Option<String>,
    },

    #[structopt(name = "init")]
    /// Fetch user configuration file
    Init {
        #[structopt(default_value = ".kattisrc")]
        /// URL to fetch files from
        file: String,
        #[structopt(short, long)]
        /// Force creation of config file
        force: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.cmd {
        Cmd::Get { pid, force } => create_problem(&pid, force).await?,
        Cmd::Test { .. } => test_kattis()?,
        Cmd::Submit { .. } => println!("You are submitting something!"),
        Cmd::Init { file, force } => {
            let kattis = Kattis::new(file);
            create_kah_dotfile(".kah", &kattis, force).expect("Could not initialize kah");
            Datafile::create(force).await?;
        }
    }

    Ok(())
}

async fn create_problem(problem_id: &str, force: bool) -> Result<()> {
    let languages = &["Rust", "Kotlin", "Java", "Python", "Haskell"];
    let language = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a language to solve problem in")
        .items(&languages[..])
        .interact()?;

    let language = Language::from_str(languages[language])?;
    let problem = ProblemMetadata::new(problem_id, force).await?;
    let mut datafile = Datafile::get_datafile().await?;

    datafile.add_problem(&problem, &language, force).await?;
    language.create_problem(problem.name, force).await?;

    Ok(())
}
