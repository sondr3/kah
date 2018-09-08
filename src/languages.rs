#[derive(Debug)]
pub enum Languages {
    Python,
    Java,
    Haskell,
    None,
}

fn get_language(name: &str) -> Languages {
    match name {
        "Python" => Languages::Python,
        "Java" => Languages::Java,
        "Haskell" => Languages::Haskell,
        _ => Languages::None,
    }
}

#[derive(Debug)]
pub struct Language {
    name: Languages,
    extension: String,
    command: String,
}

impl Language {
    fn new(name: &str, extension: String, command: String) -> Language {
        let language = get_language(&name);
        Language {
            name: language,
            extension,
            command,
        }
    }
}
