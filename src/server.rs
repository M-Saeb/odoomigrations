use std::env;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Server {
    pub addr: Option<String>,
    pub port: Option<String>,
    pub database_name: Option<String>,
    pub master_password: Option<String>,
    pub c_addons_path: Option<String>,
    pub config_file_path: Option<String>,
    pub ssh_username: Option<String>,
    pub ssh_password: Option<String>,
}


impl Server {
    fn _write_to_file(&self, content: &[u8], filename: String){
        let current_directory = env::current_dir().expect("couldn't get the current directory");
        let zip_file = current_directory.join(filename);
        let mut file = File::create(zip_file).unwrap();
        file.write(content).unwrap();

        dbg!(&file);
    }

    pub async fn call_backup_request(&self) -> bool{
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
            [
              ("master_pwd", master_password),
              ("name", database_name),
              ("backup_format", String::from("zip")),
            ]
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
