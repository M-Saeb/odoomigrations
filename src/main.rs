use std::env;
use std::fs;
use std::collections::HashMap;

// struct Server {
//     addr: &'static str,
//     port: &'static str,
//     database_name: &'static str,
//     master_password: &'static str,
//     c_addons_path: &'static str,
//     config_file: &'static str,
//     ssh_username: &'static str,
//     ssh_password: &'static str,
// }

// impl Server {
// }

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
            dbg!(&value);
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

fn main(){
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let content = open_file(filepath.to_string());
    let run_file = RunFile(content);
    let all_items = run_file.all_items();
    println!("all items are {:#?}", all_items)
}
