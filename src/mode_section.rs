#[derive(Debug)]
pub struct ModeSection{
	cache_dir: String,
	db_migrate_method: String
}

impl ModeSection{
	pub fn create_default_mode() -> ModeSection{
		return ModeSection{
            cache_dir: "odoomigrations_cache".to_string(),
            db_migrate_method: "local".to_string(),
		}
	}

	pub fn set_value(&mut self, key: String, value: String){
		if key.as_str() == "cache_dir"{
			self.cache_dir = value;
			return
		}
		if key.as_str() == "db_migrate_method"{
			self.cache_dir = value;
			return
		}
		panic!("{} is invalid mode key", key)
	}
}