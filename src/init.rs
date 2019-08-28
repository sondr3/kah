use ini::Ini;
use crate::kah::Kah;
use serde_json;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub token: String,
    pub kattis: String,
}

fn remove_kattisrc(path: String) -> Result<(), Box<dyn Error>> {
    remove_file(path)?;
    Ok(())
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
    let kattisrc = Ini::load_from_file(&path).unwrap();

    let user_section = kattisrc.section(Some("user")).unwrap();
    let kattis_section = kattisrc.section(Some("kattis")).unwrap();

    let kattis_url = kattis_section.get("loginurl").unwrap();
    let mut url = Url::parse(kattis_url)?;
    url.path_segments_mut().map_err(|_| "cannot be base")?.pop();

    let user = User {
        username: user_section.get("username").unwrap().parse()?,
        token: user_section.get("token").unwrap().parse()?,
        kattis: url.to_string(),
    };

    let kah = Kah { user };

    create_kah_dotfile(".kah", &kah, force)?;
    remove_kattisrc(path)?;

    Ok(())
}
