#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]
#[macro_use]
extern crate structopt;
extern crate ini;
extern crate reqwest;
extern crate tempfile;
extern crate zip;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod cli;
pub mod get;
pub mod init;
pub mod kah;
pub mod languages;
pub mod test;
