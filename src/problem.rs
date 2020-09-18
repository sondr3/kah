use crate::error::KahError::{FetchError, ScrapeError};
use crate::kattis::Kattis;
use anyhow::Result;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[derive(Debug)]
pub struct Problem {
    name: String,
    id: String,
    cpu_time_limit: String,
    memory_limit: String,
    difficulty: f32,
}

impl Problem {
    pub async fn get_details(id: &str) -> Result<Problem> {
        let url = Kattis::get_kattis_url();
        let path: String = format!("{}/problems/{}", url, id);
        let response = reqwest::get(&path).await?;

        let body = match response.error_for_status() {
            Ok(resp) => resp.text().await?,
            Err(err) => return Err(FetchError(id.to_string(), err.to_string()).into()),
        };

        let document = Document::from(&body[..]);
        let title = document
            .find(Class("headline-wrapper").descendant(Name("h1")))
            .next()
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find title".to_string()))?
            .text();

        let name = Problem::clean_title(title);

        let sidebar = document
            .find(Class("problem-download"))
            .next()
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find problem download".into()))?
            .parent()
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find sidebar".to_string()))?
            .find(Name("p"))
            .collect::<Vec<_>>();

        let cpu_time_limit = sidebar[1]
            .children()
            .nth(1)
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find CPU time limit".into()))?
            .text();
        let memory_limit = sidebar[2]
            .children()
            .nth(1)
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find memory limit".to_string()))?
            .text();
        let difficulty: f32 = sidebar[3]
            .find(Name("span"))
            .next()
            .ok_or_else(|| ScrapeError(id.to_string(), "Could not find difficulty".to_string()))?
            .text()
            .parse()?;

        Ok(Problem {
            id: id.into(),
            name: name.trim().to_string(),
            cpu_time_limit: cpu_time_limit.trim().to_string(),
            memory_limit: memory_limit.trim().to_string(),
            difficulty,
        })
    }

    fn clean_title(title: String) -> String {
        title
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect()
    }
}
