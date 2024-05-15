mod get_paragraph_blob;
mod position;
mod problem;
mod range;

use std::{sync::Arc, time::Instant};

use get_paragraph_blob::get_paragraph_blob;

use futures::{stream::FuturesUnordered, StreamExt};
use languagetool_rust::{CheckRequest, ServerClient};
use problem::Problem;
use typst::syntax::{FileId, Source, VirtualPath};
use typst_languagetool_preprocessor::preprocess;

pub async fn check_file(
    host: &str,
    port: &str,
    //
    file_path: &str,
    file_contents: String,
    //
    language: String,
    disabled_rules: Option<Vec<String>>,
    disabled_categories: Option<Vec<String>>,
    ignore_words: Option<Vec<String>>,
) -> Result<Vec<Problem>, languagetool_rust::error::Error> {
    let start = Instant::now();
    let source = Source::new(
        FileId::new(None, VirtualPath::new(file_path)),
        file_contents,
    );
    let paragraphs = preprocess(&source);

    let client = Arc::new(ServerClient::new(host, port));

    // Check each paragraph
    let mut tasks: FuturesUnordered<_> = paragraphs
        .iter()
        .enumerate()
        .filter_map(|(index, paragraph)| {
            let (blob, node_ranges) = get_paragraph_blob(paragraph);

            // Ignore small blobs
            if blob.len() <= 5 {
                return None;
            }

            let mut request = CheckRequest::default()
                .with_text(blob.clone())
                .with_language(language.clone());
            request.disabled_categories = disabled_categories.clone();
            request.disabled_rules = disabled_rules.clone();

            let client = Arc::clone(&client);

            Some(async move { (client.check(&request).await, node_ranges, blob, index) })
        })
        .collect();

    let end = Instant::now();
    eprintln!(
        "Created {} tasks from {} paragraphs in {}μs",
        tasks.len(),
        paragraphs.len(),
        end.duration_since(start).as_micros()
    );

    let mut problems = vec![];

    let start = Instant::now();
    // For each check result, find the problems
    while let Some((check_result, node_ranges, blob, index)) = tasks.next().await {
        let check_response = check_result?;
        let paragraph = paragraphs.get(index).unwrap();

        for check_match in check_response.matches {
            // check if the error region is a ignore word
            let match_string = &blob[check_match.offset..check_match.offset + check_match.length];
            if let Some(ignore_words) = ignore_words.as_ref() {
                if ignore_words.contains(&match_string.to_string()) {
                    continue;
                }
            }

            if let Some(problem) = Problem::try_from_match(
                &source,
                paragraph,
                &node_ranges,
                check_match,
                match_string.to_string(),
            ) {
                problems.push(problem);
            };
        }
    }
    let end = Instant::now();
    eprintln!(
        "Languagetool found {} issues in {}ms",
        problems.len(),
        end.duration_since(start).as_millis()
    );

    Ok(problems)
}
