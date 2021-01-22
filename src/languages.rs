use crate::{error::KahError, language::python::Python, language::Language};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

const PYTHON_CODE: &str = include_str!("./data/Problem.py");
const JAVA_CODE: &str = include_str!("./data/Problem.java");
const KOTLIN_CODE: &str = include_str!("./data/Problem.kt");
const RUST_CODE: &str = include_str!("./data/Problem.rs");
const HASKELL_CODE: &str = include_str!("./data/Problem.hs");
const CPP_CODE: &str = include_str!("./data/Problem.cpp");

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Languages {
    Python,
    Java,
    Haskell,
    Rust,
    Kotlin,
    CPP,
}

impl fmt::Display for Languages {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            Languages::Python { .. } => "Python",
            Languages::Java { .. } => "Java",
            Languages::Haskell { .. } => "Haskell",
            Languages::Rust { .. } => "Rust",
            Languages::Kotlin { .. } => "Kotlin",
            Languages::CPP { .. } => "C++",
        }
        .to_string();
        write!(f, "{}", name)
    }
}

impl FromStr for Languages {
    type Err = KahError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" => Ok(Languages::Python),
            "java" => Ok(Languages::Java),
            "haskell" => Ok(Languages::Haskell),
            "rust" => Ok(Languages::Rust),
            "kotlin" => Ok(Languages::Kotlin),
            "cpp" | "c++" => Ok(Languages::Kotlin),
            _ => Err(KahError::LanguageParseError(s.to_string())),
        }
    }
}

impl Languages {
    pub(crate) fn get_language(&self) -> impl Language + Display {
        match self {
            Languages::Python => Python::new(),
            Languages::Java => todo!(),
            Languages::Haskell => todo!(),
            Languages::Rust => todo!(),
            Languages::Kotlin => todo!(),
            Languages::CPP => todo!(),
        }
    }

    pub(crate) fn initial_problem_content(&self) -> String {
        match self {
            Languages::Python { .. } => PYTHON_CODE,
            Languages::Java { .. } => JAVA_CODE,
            Languages::Haskell { .. } => HASKELL_CODE,
            Languages::Rust { .. } => RUST_CODE,
            Languages::Kotlin { .. } => KOTLIN_CODE,
            Languages::CPP { .. } => CPP_CODE,
        }
        .to_string()
    }
}
