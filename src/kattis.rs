use ini::Ini;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kattis {
    pub username: String,
    pub token: String,
    pub hostname: String,
    pub submit: String,
}

impl Kattis {
    pub fn new(path: &PathBuf) -> Self {
        let kattis_rc = Ini::load_from_file(&path).unwrap();

        let user_section = kattis_rc.section(Some("user")).unwrap();
        let kattis_section = kattis_rc.section(Some("kattis")).unwrap();

        let username = user_section
            .get("username")
            .expect("Could not find 'username' field");
        let token = user_section
            .get("token")
            .expect("Could not find 'token' field");

        let submit = kattis_section.get("submissionurl").unwrap();
        let hostname = kattis_section.get("hostname").unwrap();
        let hostname = format!("{}{}", "https://", hostname);

        Kattis {
            username: username.into(),
            token: token.into(),
            hostname,
            submit: submit.to_owned(),
        }
    }

    pub(crate) fn get_kattis_url() -> String {
        let file = File::open(".kah").expect("Could not find .kah");
        let reader = BufReader::new(file);
        let json: Kattis = serde_json::from_reader(reader).expect("Could not read .kah");

        json.hostname
    }
}
