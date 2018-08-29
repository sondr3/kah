use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kattis-rs", about = "a simple Kattis helper utility")]
enum Opt {
    #[structopt(name = "get")]
    /// Get sample test files from Kattis
    Get {
        #[structopt(help = "Problem ID")]
        id: String,
        #[structopt(help = "Problem Name")]
        name: String,
        #[structopt(short = "u", long = "url", default_value = "https://open.kattis.com")]
        url: String,
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

fn kattis_file_path(assignment: String) -> String {
    String::from(format!("problems/{}/file/statement/samples.zip", assignment))
}

fn get_kattis_sample(url: String, assignment: String) {
    let path = format!("{}/{}", url, &kattis_file_path(assignment));
    let mut res = reqwest::get(&path).expect("Whoops");

    println!("{}", path);
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
        Opt::Get{id: _, name: _, url: _} => get_kattis_sample("https://open.kattis.com".to_string(), "trik".to_string()),
        Opt::Test{file: _, language: _} => test_kattis(),
    }

    println!("{:?}", opt);
}
