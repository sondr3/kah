use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum KahError {
    #[error("Could not scrape {0}: {1}")]
    ScrapeError(String, String),
    #[error("Could not fetch {0}: {1}")]
    FetchError(String, String),
}
