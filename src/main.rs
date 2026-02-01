mod link_checker;
use std::env;
//the amount of threads to run at a given execution
const  THREADS:usize = 32;

#[tokio::main]
async fn main(){
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!(
            "wrong amount of arguments please run the program with: cargo run <input-path> <output-path>"
        );
        return
    }
    let path = args[1].to_string();
    link_checker::check_links(path, THREADS).await.unwrap();
}