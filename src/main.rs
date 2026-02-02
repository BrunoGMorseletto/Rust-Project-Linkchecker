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
    let input_path: String = args[1].to_string();
    let output_path: String = args[2].to_string();
    link_checker::check_links(input_path, THREADS, output_path).await.unwrap();
}