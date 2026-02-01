use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use futures::stream::{self, StreamExt};
use std::io::{Error};
use regex::Regex;
use reqwest::Client;


pub(crate) async fn check_links(path:String, threads: usize)->Result<bool, Error>{
    let file: File = File::open(path).await.unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines = reader.lines();
    let mut lines_stream: Vec<String> = Vec::new();
    while let Some(line) = lines.next_line().await.unwrap() {
        lines_stream.push(line);
    }
    stream::iter(lines_stream)
        .map(|line| async move {
            process_line(line).await
        })
        .buffer_unordered(threads) 
        .collect::<Vec<_>>()
        .await;
    Ok(true)
}

async fn  process_line(line:String){
    if line.contains("https") == true{
        let regex = Regex::new(r"\((.*?)\)").unwrap();
        if let Some(url) = regex.captures(line.as_str()) {
        println!("{}", &url[1]);
        make_request_to(&url[1]).await; 
        }
    }
}

async fn make_request_to(url:&str){
    let client = Client::new();
    let request = client.get(url).build();
    if request.is_ok(){
        let response = client.execute(request.unwrap()).await;
        if response.is_ok(){
            let response_text = response.unwrap().text().await;
            let title_regex = Regex::new(r"<title>(.*?)</title>").unwrap();
            if let Some(title) = title_regex.captures(response_text.unwrap().as_str()){
                println!("{}", &title[1]); 
            }
        }else{
            println!("request failed, {}", response.unwrap_err());
        }
    }
}