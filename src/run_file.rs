use std::fs;
use std::collections::HashMap;

pub struct RunFile (pub String);

impl RunFile{
	pub fn open_file(filepath: &str) -> Self{
		let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
		let run_file = RunFile(contents);
	    return run_file
	}

    fn _clean_line(&self, line: String) -> Option<String>{
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

    pub fn all_items(&self) -> HashMap<String, String>{
        let mut hash_map = HashMap::new();
        let splitted_lines = self.0.split("\n");
        for line in splitted_lines{
            let clean_line = self._clean_line( String::from(line) );
            if clean_line.is_none(){
                continue;
            }
            let line = clean_line.unwrap();
            let mut line_split = line.split("=");
            let key = line_split.nth(0).unwrap().trim();
            let value = {
                let mut exception_msg = "Incorrect line format on ".to_owned();
                exception_msg.push_str(&line);
                let uncleaned_value = line_split.nth(0).expect(&exception_msg);
                let mut uncleaned_value_split = uncleaned_value.split("#");
                let cleaned_value = uncleaned_value_split.nth(0).unwrap().trim();
                cleaned_value
            };
            hash_map.insert(
                String::from( key ),
                String::from( value )
            );
        }
        hash_map
    }
    
}
