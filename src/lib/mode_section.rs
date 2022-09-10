use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ModeSection{
	pub cache_dir: String,
	pub db_migrate_method: String,
	pub should_log: bool
}

impl ModeSection{
	pub fn create_default_mode() -> ModeSection{
		return ModeSection{
            cache_dir: "odoomigrations_cache".to_string(),
            db_migrate_method: "local".to_string(),
			should_log: true,
		}
	}

	pub fn set_value(&mut self, key: String, value: String){
		if key.as_str() == "cache_dir"{
			self.cache_dir = value;
			return
		}
		if key.as_str() == "db_migrate_method"{
			assert!(["local", "direct"].contains(&value.as_str()), "Invalid value for db_migrate_method. Can only assign 'local' or 'direct'");
			self.db_migrate_method = value;
			return
		}
		if key.as_str() == "log"{
			assert!(["true", "false"].contains(&value.as_str()), "Invalid value for db_migrate_method. Can only assign 'local' or 'direct'");
			self.should_log = match value.to_lowercase().as_str() {
				"true" => true,
				_ => false,
			};
			return
		}
		panic!("{} is invalid mode key", key)
	}

	pub fn creat_cache_dir(&self) -> PathBuf{
		let current_dir = env::current_dir().expect("Could not fetch current directory");
		let full_cache_dir = current_dir.join(&self.cache_dir);
		fs::create_dir(&full_cache_dir);	
		full_cache_dir
	}

	pub fn write_bytes_to_cache_dir(&self, filename: &str, content: &[u8]){
		if self.db_migrate_method == "local".to_string(){
			let cache_dir = self.creat_cache_dir();
            let zip_file = cache_dir.join(filename);
            let mut file = File::create(zip_file).unwrap();
            file.write(content).unwrap();
		}
	}
}