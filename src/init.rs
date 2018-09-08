use ini::Ini;
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    token: String,
    kattis: String,
}

pub fn parse_kattisrc(path: String) -> Result<(), Box<Error>> {
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

    println!("{:?}", user);

    Ok(())
}
