use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum KahError {
    #[error("Could not scrape {0}: {1}")]
    ScrapeError(String, String),
    #[error("Could not fetch {0}: {1}")]
    FetchError(String, String),
    #[error("No language {0} matches predefined language")]
    LanguageParseError(String),
    #[error("No such problem exists: {0}")]
    NoSuchProblem(String),
    #[error("No such flag: {0}")]
    ForceProblemCreationError(u64),
    #[error("Could not parse kattisrc")]
    KattisrcParseError,
}
