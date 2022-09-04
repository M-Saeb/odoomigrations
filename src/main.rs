mod lib;

use lib::run_file::RunFile;
use std::env;

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("No instruction file was provided")
    }
    let filepath = &args[1];
    let run_file = RunFile::from_file(filepath);
    run_file.process_source_machine_backup().await;
}