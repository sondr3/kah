use ini::Ini;
use std::error::Error;
use url::Url;
use serde_json;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use kah::Kah;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub token: String,
    pub kattis: String,
}

fn create_kah_dotfile(name: &str, input: Kah, force: bool) -> Result<(), Box<Error>> {
    let path = Path::new(&name);
    let mut file;
    if path.exists() && !force {
        eprintln!("The dotfile for kah already exists");
        Err(From::from("Exiting..."))
    } else {
        file = File::create(path)?;

        let json = serde_json::to_string_pretty(&input)?;
        let buffer = json.into_bytes();

        file.write(&buffer)?;

        Ok(())
    }
}

pub fn parse_kattisrc(path: String, force: bool) -> Result<(), Box<Error>> {
    let kattisrc = Ini::load_from_file(path).unwrap();

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

    let kah = Kah {
        user,
    };

    create_kah_dotfile(".kah", kah, force)?;

    Ok(())
}
