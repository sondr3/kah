use crate::init::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kah {
    pub user: User,
}
