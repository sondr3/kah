use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kattis-rs", about = "a simple Kattis helper utility")]
enum Opt {
    #[structopt(name = "get")]
    /// Get sample test files from Kattis
    Get {
        #[structopt(short = "u", long = "url", default_value = "https://open.kattis.com")]
        url: String
    },
    #[structopt(name = "test")]
    /// Test a Kattis assignment
    Test {
        #[structopt(help = "Kattis assignment to run")]
        file: String,
        #[structopt(short = "l", long = "language", help = "Override language")]
        language: Option<String>,
    },
}

#[derive(Debug)]
enum Language {
    Python,
    Java,
}

#[derive(Debug)]
struct LanguageInfo {
    name: Language,
    extension: String,
    command: String
}

fn get_kattis_sample() {
    let mut res = reqwest::get("https://open.kattis.com").expect("Whoops");

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());
}

fn test_kattis() {
    println!("YOU ARE TESTING ME");
}

fn main() {
    let java = LanguageInfo {
        name: Language::Java,
        extension: "java".to_string(),
        command: "java".to_string()
    };

    let opt = Opt::from_args();

    match opt {
        Opt::Get{url: _} => get_kattis_sample(),
        Opt::Test{file: _, language: _} => test_kattis(),
    }

    println!("{:?}", opt);
}
