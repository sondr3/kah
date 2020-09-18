use crate::kattis::Kattis;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn create_kah_dotfile(name: &str, input: &Kattis, force: bool) -> Result<(), Box<dyn Error>> {
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

        println!("Successfully created configuration file");

        Ok(())
    }
}
