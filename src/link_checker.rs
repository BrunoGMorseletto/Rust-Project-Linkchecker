//! This module provides tools for link checking and validation.
use futures::stream::{self, StreamExt};
use regex::Regex;
use reqwest::Client;
use std::io::Error;
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
/// Function that receives a string that represents the path to the input file, the amount of threads to run and the path to the output file
pub async fn check_links(
    input_path: String,
    threads: usize,
    output_path: String,
) -> Result<bool, Error> {
    let file: File = File::open(input_path).await.unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines = reader.lines();
    let mut lines_stream: Vec<String> = Vec::new();
    while let Some(line) = lines.next_line().await.unwrap() {
        lines_stream.push(line);
    }
    let line_processing_result = stream::iter(lines_stream)
        .map(|line| async move { process_line(line).await })
        .buffered(threads)
        .collect::<Vec<_>>()
        .await;

    let output_file_create: Result<File, Error> = File::create(output_path).await;
    if let Err(e) = output_file_create {
        return Err(e);
    }
    let mut output_file: File = output_file_create.unwrap();
    for url_result in line_processing_result.iter() {
        match url_result {
            Ok(data) => {
                let title_url = data;
                if title_url.0.is_empty() {
                    continue;
                }
                let output_text = format!("[{}]({url})\n", title_url.1, url = title_url.0);
                let write_result = output_file.write(output_text.as_bytes()).await;
                match write_result {
                    Ok(_data) => {
                        continue;
                    }
                    Err(error) => return Err(error),
                }
            }
            Err(_error) => {
                return Err(Error::new(
                    std::io::ErrorKind::Unsupported,
                    "unexpected error",
                ));
            }
        }
    }
    Ok(true)
}
/// Function that that searches for urls in given string, it also makes a simple request to the url
/// return the url and the request result be it an error or a response
async fn process_line(line: String) -> Result<(String, String), Error> {
    if line.contains("https") {
        let regex = Regex::new(r"\((.*?)\)").unwrap();
        if let Some(url) = regex.captures(line.as_str()) {
            let request_result = make_request_to(&url[1]).await.unwrap();
            Ok((url[1].to_string(), request_result))
        } else {
            Ok(("".to_string(), "".to_string()))
        }
    } else {
        Ok(("".to_string(), "".to_string()))
    }
}
///Async function that makes simple request to an url to check its status
/// in case of being sucessful returns the content of the title html tag in the response
async fn make_request_to(url: &str) -> Result<String, Error> {
    let client = Client::new();
    let request = client.get(url).build().unwrap();
    let response = client.execute(request).await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                let response_text = response.text().await;
                let title_regex = Regex::new(r"<title>(.*?)</title>").unwrap();
                if let Some(title) = title_regex.captures(response_text.unwrap().as_str()) {
                    println!("{}", &title[1]);
                    Ok(title[1].to_string())
                } else {
                    Ok("no title found".to_string())
                }
            } else {
                let status = response.status();
                Ok(format!("Error: {}", status))
            }
        }
        Err(e) => Ok(e.to_string()),
    }
}
