use crate::init::User;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kah {
    pub user: User,
}
