use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use futures::stream::{self, StreamExt};
use std::io::{Error};


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
    println!("checking line: {}", line);
}