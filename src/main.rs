mod server;
mod run_file;
mod run_migration;

use std::env;
use run_file::RunFile;
use run_migration::RunMigration;


#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let run_file = RunFile::open_file(filepath);
    let all_items = run_file.all_items();
    let run_migration = RunMigration(all_items);
}
