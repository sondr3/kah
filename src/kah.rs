use crate::kattis::Kattis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kah {
    pub kattis: Kattis,
}
