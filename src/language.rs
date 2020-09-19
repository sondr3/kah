use crate::error::KahError;
use anyhow::Result;
use std::path::Path;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const PYTHON_CODE: &str = include_str!("./data/Problem.py");
const JAVA_CODE: &str = include_str!("./data/Problem.java");
const KOTLIN_CODE: &str = include_str!("./data/Problem.kt");
const RUST_CODE: &str = include_str!("./data/Problem.rs");
const HASKELL_CODE: &str = include_str!("./data/Problem.hs");

#[derive(Debug)]
pub struct LanguageConfiguration {
    extension: String,
    compile_command: String,
    run: String,
}

#[derive(Debug)]
pub enum Language {
    Python {
        configuration: LanguageConfiguration,
    },
    Java {
        configuration: LanguageConfiguration,
    },
    Haskell {
        configuration: LanguageConfiguration,
    },
    Rust {
        configuration: LanguageConfiguration,
    },
    Kotlin {
        configuration: LanguageConfiguration,
    },
}

impl Language {
    pub(crate) async fn create_problem(&self, name: String, force: bool) -> Result<()> {
        let code = self.initial_problem_content();
        let path = format!(
            "{}/{}.{}",
            self.to_string(),
            name,
            self.configuration().extension
        );

        if !Path::new(&self.to_string()).exists() {
            tokio::fs::create_dir_all(self.to_string()).await?;
        }

        let path = Path::new(&path);
        if path.exists() && !force {
            eprintln!("{} already exists for language {}", name, self.to_string())
        } else {
            let mut file = File::create(path).await?;
            file.write_all(code.as_bytes()).await?;
        }

        println!("Created {} in {}", name, self.to_string());

        Ok(())
    }

    pub(crate) fn problem_path(&self, name: &str) -> String {
        format!(
            "{}/{}.{}",
            self.to_string(),
            name,
            self.configuration().extension
        )
    }

    fn initial_problem_content(&self) -> String {
        match self {
            Language::Python { .. } => PYTHON_CODE,
            Language::Java { .. } => JAVA_CODE,
            Language::Haskell { .. } => HASKELL_CODE,
            Language::Rust { .. } => RUST_CODE,
            Language::Kotlin { .. } => KOTLIN_CODE,
        }
        .to_string()
    }

    fn configuration(&self) -> &LanguageConfiguration {
        match self {
            Language::Python { configuration } => configuration,
            Language::Java { configuration } => configuration,
            Language::Haskell { configuration } => configuration,
            Language::Rust { configuration } => configuration,
            Language::Kotlin { configuration } => configuration,
        }
    }
}

impl FromStr for Language {
    type Err = KahError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" => Ok(Language::Python {
                configuration: LanguageConfiguration {
                    extension: "py".to_string(),
                    compile_command: "python3".to_string(),
                    run: "".to_string(),
                },
            }),
            "java" => Ok(Language::Java {
                configuration: LanguageConfiguration {
                    extension: "java".to_string(),
                    compile_command: "javac".to_string(),
                    run: "".to_string(),
                },
            }),
            "haskell" => Ok(Language::Haskell {
                configuration: LanguageConfiguration {
                    extension: "hs".to_string(),
                    compile_command: "ghci".to_string(),
                    run: "".to_string(),
                },
            }),
            "rust" => Ok(Language::Rust {
                configuration: LanguageConfiguration {
                    extension: "rs".to_string(),
                    compile_command: "rustc -g --crate-type bin".to_string(),
                    run: "".to_string(),
                },
            }),
            "kotlin" => Ok(Language::Kotlin {
                configuration: LanguageConfiguration {
                    extension: "kt".to_string(),
                    compile_command: "kotlinc".to_string(),
                    run: "".to_string(),
                },
            }),
            _ => Err(KahError::LanguageParseError(s.to_string())),
        }
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Python { .. } => "python",
            Language::Java { .. } => "java",
            Language::Haskell { .. } => "haskell",
            Language::Rust { .. } => "rust",
            Language::Kotlin { .. } => "kotlin",
        }
        .to_string()
    }
}
