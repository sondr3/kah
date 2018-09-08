use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(
    name = "kah",
    about = "a simple Kattis helper utility",
    raw(global_settings = "&[AppSettings::ColoredHelp]")
)]
pub struct Opt {
    #[structopt(short = "v", long = "verbose")]
    /// Verbose messages
    verbose: bool,
    #[structopt(short = "f", long = "force")]
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
        #[structopt(help = "Problem ID")]
        pid: String,
        #[structopt(help = "Problem Name")]
        name: String,
    },

    #[structopt(name = "test")]
    /// Run tests for a Kattis problem locally
    Test {
        #[structopt(help = "Kattis problem to test")]
        file: String,
        #[structopt(short = "l", long = "language")]
        /// Select language for problem
        language: Option<String>,
    },

    #[structopt(name = "submit")]
    /// Submit your solution to a Kattis problem
    Submit {
        #[structopt(help = "Kattis problem to submit")]
        file: String,
        #[structopt(short = "l", long = "language")]
        /// Select language for problem
        language: Option<String>,
    },

    #[structopt(name = "init")]
    /// Fetch user configuration file
    Init {
        #[structopt(default_value = ".kattisrc")]
        /// URL to fetch files from
        file: String,
        #[structopt(short = "f", long = "force")]
        /// Overwrite existing files
        force: bool,
    },
}

pub fn parse() -> Opt {
    Opt::from_args()
}
