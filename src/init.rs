use crate::kah::Kah;
use ini::Ini;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub token: String,
    pub hostname: String,
    pub submit: String,
}

fn create_kah_dotfile(name: &str, input: &Kah, force: bool) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&name);
    let mut file;
    if path.exists() && !force {
        eprintln!("The dotfile for kah already exists");
        Err(From::from("Exiting..."))
    } else {
        file = File::create(path)?;

        let json = serde_json::to_string_pretty(&input)?;
        let buffer = json.into_bytes();

        file.write_all(&buffer)?;

        Ok(())
    }
}

pub fn parse_kattisrc(path: String, force: bool) -> Result<(), Box<dyn Error>> {
    let kattis_rc = Ini::load_from_file(&path).unwrap();

    let user_section = kattis_rc.section(Some("user")).unwrap();
    let kattis_section = kattis_rc.section(Some("kattis")).unwrap();

    let submit = kattis_section.get("submissionurl").unwrap();
    let hostname = kattis_section.get("hostname").unwrap();

    let user = Config {
        username: user_section.get("username").unwrap().parse()?,
        token: user_section.get("token").unwrap().parse()?,
        hostname: hostname.to_owned(),
        submit: submit.to_owned(),
    };

    let kah = Kah { user };

    create_kah_dotfile(".kah", &kah, force)?;

    Ok(())
}
