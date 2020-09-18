use anyhow::Result;
use kah::get::get_kattis_sample;
use kah::init::create_kah_dotfile;
use kah::kattis::Kattis;
use kah::test::test_kattis;
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
        /// Problem name
        name: String,
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
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.cmd {
        Cmd::Get { pid, name } => get_kattis_sample(&pid, &name)?,
        Cmd::Test { .. } => test_kattis()?,
        Cmd::Submit { .. } => println!("You are submitting something!"),
        Cmd::Init { file } => {
            let kattis = Kattis::new(file);
            create_kah_dotfile(".kah", &kattis, false).expect("Could not initialize kah")
        }
    }

    Ok(())
}
