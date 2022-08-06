use std::env;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
#[derive(Debug)]
struct Server {
    addr: Option<String>,
    port: Option<String>,
    database_name: Option<String>,
    master_password: Option<String>,
    c_addons_path: Option<String>,
    config_file: Option<String>,
    ssh_username: Option<String>,
    ssh_password: Option<String>,
}

impl Server {
    fn _write_to_file(&self, content: &[u8], filename: String){
        let current_directory = env::current_dir().expect("couldn't get the current directory");
        let zip_file = current_directory.join(filename);
        let mut file = File::create(zip_file).unwrap();
        file.write(content).unwrap();

        dbg!(&file);
    }

    async fn run_backup_request(&self) -> bool{
        let full_url = {
            let mut string = "".to_string();
            string.push_str( "http://" );
            string.push_str( self.addr.as_ref().expect("No Addres was provided") );
            string.push_str( ":" );
            string.push_str( self.port.as_ref().expect("No port was provided") );
            string.push_str( "/web/database/backup" );
            string
        };
        dbg!(&full_url);
        let request_param = {
            let master_password = self.master_password.as_ref().expect("Master Password Is Not Set").to_owned();
            let database_name = self.database_name.as_ref().expect("Database Name Is Not Set").to_owned();
            let params = [
              ("master_pwd", master_password),
              ("name", database_name),
              ("backup_format", String::from("zip")),
            ];
            params
        };
        let client = reqwest::Client::new();
        let response = client.post(full_url)
            .form(&request_param)
            .send()
            .await
            .expect("Something wrong occurs in the call");
        let response_body = &response.bytes().await.expect("Something went wrong");
        self._write_to_file(response_body.as_ref(), String::from("demo_db.zip"));
        return true
    }
}

struct RunFile (String);

impl RunFile{
    fn _returned_cleaned_line(&self, line: String) -> Option<String>{
        let cleaned_line = line.trim();
        if cleaned_line.len() == 0 || cleaned_line.starts_with("#"){
            return None
        }
        if cleaned_line.contains("#"){ // removing inline_comment
            let mut splitted_line = cleaned_line.split("#");
            let str_value = splitted_line.nth(0).expect("Somehting went wrong, please solve it");
            return Some( String::from(str_value) )
        } else {
            return Some( String::from(cleaned_line) )
        }
    }

    fn all_items(&self) -> HashMap<String, String>{
        let mut hash_map = HashMap::new();
        let splitted_lines = self.0.split("\n");
        for line in splitted_lines{
            let clean_line = self._returned_cleaned_line( String::from(line) );
            if clean_line.is_none(){
                continue;
            }
            let line = String::from(clean_line.unwrap());
            let mut line_split = line.split("=");
            let key = line_split.nth(0).unwrap().trim();
            let value = {
                let mut exception_msg = "Incorrect line format on ".to_owned();
                exception_msg.push_str(&line);
                let tmp_str = line_split.nth(0).expect(&exception_msg);
                let mut tmp_str_split = tmp_str.split("#");
                let cleaned_str = tmp_str_split.nth(0).unwrap().trim();
                cleaned_str
            };
            hash_map.insert(
                String::from( key ),
                String::from( value )
            );
        }
        hash_map
    }
    
}

fn open_file(file_path: String) -> String{
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    return contents
}

fn create_source_server_struct(all_items: HashMap<String, String>) -> Server{
    fn get_value(all_items: &HashMap<String, String>, key: &str) -> Option<String> {
        // using a custome get_value instead of default get() because the latter returns Option<&String>
        let value_obj = all_items.get(key);
        match value_obj {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }
    let server = Server{
        addr: get_value(&all_items, "source_ip"),
        port: get_value(&all_items, "source_port"),
        database_name: get_value(&all_items, "source_database_name"),
        master_password: get_value(&all_items, "source_master_password"),
        c_addons_path: get_value(&all_items, "source_c_addons_path"),
        config_file: get_value(&all_items, "source_config_file"),
        ssh_username: get_value(&all_items, "source_ssh_username"),
        ssh_password: get_value(&all_items, "ssh_password"),
    };
    return server
}

#[tokio::main]
async fn main(){
    // let a = b"something random";
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let content = open_file(filepath.to_string());
    let run_file = RunFile(content);
    let all_items = run_file.all_items();
    if all_items.get("source_ip").is_some(){
        let source_server = create_source_server_struct(all_items);
        source_server.run_backup_request().await;
    }
}
