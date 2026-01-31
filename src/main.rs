mod link_checker;
use std::env;
use std::io::{Error};
//the amount of threads to run at a given execution
const  THREADS:usize = 32;
const PATH: &'static str = "/home/bmorselrtto/Rust-Project-Linkchecker/test_data/test_file.md";

#[tokio::main]
async fn main(){
    //let args: Vec<_> = env::args().collect();
    /*if args.len() != 4 {
        println!(
            "wrong amount of arguments please run the program with: cargo run <input-path> <output-file-name> "
        );
        return
    }*/
    link_checker::check_links(PATH.to_string(), THREADS).await.unwrap();
}