mod server;
mod run_file;
mod mode_section;

use std::env;
use run_file::RunFile;

// #[tokio::main]
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("No instruction file was provided")
    }
    let filepath = &args[1];
    let run_file = RunFile::from_file(filepath);
}