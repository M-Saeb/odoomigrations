
use std::env;
use std::collections::HashMap;
use super::server::Server;

pub struct RunMigration(pub HashMap<String, String>);

impl RunMigration{
    pub fn run(&self){
        // TODO: this function is where I left off
        // if self.start_from_source_server(){
        //     source_server = self.create_source_server();
        //     source_server.database_name.is_some(){
        //         source_server.backup_database_up_to(path);
        //     }
        //     source_server.c_addons_path.is_some(){
        //         source_server.backup_c_addons_to(path);
        //     }
        //     source_server.config_file_path.is_some(){
        //         source_server.backup_config_file_to(path);
        //     }
        // }
        // if self.upload_dest_server(){
        //     dest_server = self.create_dest_server():
        //     dest_server.database_name.is_some(){
        //         dest_server.upload_database_from(database_path);
        //     }
        //     dest_server.c_addons_path.is_some(){
        //         dest_server.upload_c_addons_from(c_addons_path);
        //     }
        //     dest_server.config_file_path.is_some(){
        //         dest_server.upload_config_file_from(config_file_path);
        //     }

        // }
    }

    fn start_from_source_server(&self) -> bool{
        return self.0.get("source_database_name").is_some() 
        || self.0.get("source_c_addons_path").is_some()
        || self.0.get("source_config_file_path").is_some();
    }

    fn get_cache_dir_path(&self) -> String {
        let given_cache_dir = self.0.get("cache_dir");
        let current_directory = env::current_dir().expect("couldn't get the current directory");
        match given_cache_dir {
            Some(value) => {
                let dir_path_buff = current_directory.join(value);
                let dir_path_string = dir_path_buff.to_str().expect("Invalid Path").to_string();
                dir_path_string
            },
            None => {
                let dir_path_buff = current_directory.join(RunMigration::get_default_cache_dir());
                let dir_path_string = dir_path_buff.to_str().expect("Invalid Path").to_string();
                dir_path_string
            },
        }
    }


	fn create_source_server_struct(&self) -> Server{
		// using a custome get_value instead of default get() because the latter returns Option<&String>
		let get_value = |key| {
			let value_obj = &self.0.get(key);
			match value_obj {
				Some(value) => Some(value.to_string()),
				None => None,
			}
		};
		let server = Server{
			addr: get_value("source_ip"),
			port: get_value("source_port"),
			database_name: get_value("source_database_name"),
			master_password: get_value("source_master_password"),
			c_addons_path: get_value("source_c_addons_path"),
			config_file_path: get_value("source_config_file"),
			ssh_username: get_value("source_ssh_username"),
			ssh_password: get_value("ssh_password"),
		};
		return server
	}

	fn get_default_cache_dir() -> String {
        String::from("odoomigrations_cache")
    }
}
