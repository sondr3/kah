use crate::init::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kah {
    pub user: Config,
}
