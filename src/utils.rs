use crate::problem::Sample;
use anyhow::Result;
use std::{io::Read, path::PathBuf};
use tokio::fs::File;
use tokio::time::Duration;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct ZipFile {
    name: String,
    content: String,
}

pub(crate) async fn unzip(file_name: &PathBuf) -> Result<Vec<Sample>> {
    let file = File::open(&file_name).await?.into_std().await;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut files = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        files.push(ZipFile {
            name: file.name().to_string(),
            content,
        })
    }

    let mut inputs: Vec<_> = files.iter().filter(|s| s.name.ends_with("in")).collect();
    let mut outputs: Vec<_> = files.iter().filter(|s| s.name.ends_with("ans")).collect();
    inputs.sort_unstable_by(|s, o| s.name.cmp(&o.name));
    outputs.sort_unstable_by(|s, o| s.name.cmp(&o.name));

    let samples = inputs
        .iter()
        .zip(outputs.iter())
        .map(|(i, o)| Sample {
            input: i.content.clone(),
            expected: o.content.clone(),
        })
        .collect();

    Ok(samples)
}

pub(crate) fn average_duration(timings: &[Duration], count: usize) -> String {
    format_duration(Duration::from_secs_f64(
        timings.iter().map(|t| t.as_secs_f64()).sum::<f64>() / count as f64,
    ))
}

pub(crate) fn min_duration(timings: &[Duration]) -> String {
    format_duration(match timings.iter().min() {
        Some(x) => *x,
        None => Duration::from_secs(0),
    })
}

pub(crate) fn max_duration(timings: &[Duration]) -> String {
    format_duration(match timings.iter().max() {
        Some(x) => *x,
        None => Duration::from_secs(0),
    })
}

fn format_duration(duration: Duration) -> String {
    format!("{:.2?}s", duration.as_secs_f64())
}
