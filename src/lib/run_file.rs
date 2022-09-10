use std::fs;
use super::server::Server;
use super::mode_section::ModeSection;
use std::error::Error;

use log::LevelFilter;
use log;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};


#[derive(Debug)]
pub struct RunFile{
    content: String,
    pub mode_section: ModeSection,
    pub source_server: Server,
    pub destination_server: Server,
}

impl RunFile{
	pub fn from_file(filepath: &str) -> Self{
		let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
		let mut run_file = RunFile{
            content: contents,
            mode_section: ModeSection::create_default_mode(),
            source_server: Server::default_server(),
            destination_server: Server::default_server()
        };
        run_file.prepare();
        run_file.setup_log4rs();
	    return run_file
	}

    // TODO: this needs a lot of cleaning exception handeling
    fn prepare(&mut self){
        fn _clean_line(line: String) -> Option<String>{
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

        fn is_mode_line(line: &String) -> bool{
            return line.starts_with("[") && line.ends_with("]")
        }

        fn remove_square_brackets(line: &String) -> String{
            let line_str = line.as_str();
            let line_without_brackets = &line_str[1..line_str.len()-1];
            line_without_brackets.trim().to_string()
        }

        let lines = self.content.split("\n");
        let mut current_section_option: Option<String> = None;
        for line in lines{
            let cleaned_line_option = _clean_line(line.to_string());
            if cleaned_line_option.is_none(){
                continue
            }
            let clean_line: String = cleaned_line_option.unwrap();
            if is_mode_line(&clean_line){
                // switching to new section
                let section_name = remove_square_brackets(&clean_line);
                current_section_option.replace( section_name );
                continue;
            } else {
                let mut splitted_line = clean_line.split("=");
                let key: String = splitted_line.nth(0).expect("this shouldn't have happend !?").trim().to_string();
                let value: String = splitted_line.nth(0).expect("invalid line format").trim().to_string();
                let current_section = current_section_option.as_ref().expect("Invalid run file format").as_str();
                if current_section == "mode"{
                    self.mode_section.set_value(key, value);
                    continue
                }
                if current_section == "source"{
                    self.source_server.set_value(key, value);
                    continue
                }
                if current_section == "destination"{
                    self.destination_server.set_value(key, value);
                    continue
                }
            }
        }
    }

    fn setup_log4rs(&self) -> Result<(), Box<dyn Error>> {
        let full_path = self.mode_section.cache_dir.to_owned() + "/output.log";
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
            .build(full_path)?;
    
        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder()
                       .appender("logfile")
                       .build(LevelFilter::Info))?;
    
        log4rs::init_config(config)?;
    
        log::info!("Fuck Yeah!");
    
        Ok(())
    }

    pub async fn run_proces(&self){
        self.mode_section.creat_cache_dir();
        self.source_server.run_full_backup_process(&self.mode_section).await;
    }
}