#[derive(Debug)]
pub enum Language {
    Python,
    Java,
}

#[derive(Debug)]
pub struct LanguageInfo {
    name: Language,
    extension: String,
    command: String,
}

pub fn java() -> LanguageInfo {
    LanguageInfo {
        name: Language::Java,
        extension: "java".to_string(),
        command: "java".to_string(),
    }
}

pub fn python() -> LanguageInfo {
    LanguageInfo {
        name: Language::Python,
        extension: ".py".to_string(),
        command: "python".to_string(),
    }
}
