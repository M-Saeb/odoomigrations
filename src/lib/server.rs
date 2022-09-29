use bytes::Bytes;
use reqwest::Error;

use log;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::fs::File;
use ssh2::Session;

use super::mode_section::ModeSection;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub port: String,
    pub database_name: Option<String>,
    pub master_password: Option<String>,
    pub c_addons_path: Option<String>,
    pub config_file_path: Option<String>,
    pub ssh_username: Option<String>,
    pub ssh_password: Option<String>,
}


impl Server {
    pub fn default_server() -> Server{
        return Server { 
            address: "127.0.0.1".to_string(),
            port: "8069".to_string(),
            database_name: None,
            master_password: None,
            c_addons_path: None,
            config_file_path: None,
            ssh_username: None,
            ssh_password: None 
        }
    }

    pub fn set_value(&mut self, key: String, value: String){
        if String::from("address") == key {
            self.address = value;
            return
        }
        if String::from("port") == key {
            self.port = value;
            return
        }
        if String::from("database_name") == key {
            self.database_name = Some(value);
            return
        }
        if String::from("master_password") == key {
            self.master_password = Some(value);
            return
        }
        if String::from("c_addons_path") == key {
            self.c_addons_path = Some(value);
            return
        }
        if String::from("config_file_path") == key {
            self.config_file_path = Some(value);
            return
        }
        if String::from("ssh_username") == key {
            self.ssh_username = Some(value);
            return
        }
        if String::from("ssh_password") == key {
            self.ssh_password = Some(value);
            return
        } 

        panic!("That shouldn't have happned with {}", key);            

    }

    fn remove_prefix<'a>(&self, full_string: &'a str, suffix: &str) -> &'a str {

        match full_string.strip_prefix(suffix) {
            Some(clipped) => clipped,
            None => full_string
        }
    }

    fn ssh_info_is_available(&self) -> bool{
        self.ssh_username.is_some() && self.ssh_password.is_some()
    }

    fn to_backup_database(&self) -> bool{
        self.database_name.is_some() && self.master_password.is_some()
    }

    fn to_bakup_c_addons(&self) -> bool{
        self.ssh_info_is_available() && self.c_addons_path.is_some()
    }

    fn to_backup_config_file(&self) -> bool{
        self.ssh_info_is_available() && self.config_file_path.is_some()
    }

    fn create_ssh_connection(&self) -> Session{
        let username = self.ssh_username.as_ref().unwrap().as_ref();
        let password = self.ssh_password.as_ref().unwrap().as_ref();
        // Connect to the local SSH server

        let tcp_address = format!(
            "{}:22",
            self.remove_prefix(&self.address, "http://")
        );
        dbg!(&username, &password, &tcp_address);
        let tcp = TcpStream::connect(tcp_address).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_password(
            username,
            password
        ).unwrap();

        return session
    }

    fn run_remote_command(&self, command: &str){
        let sess = self.create_ssh_connection();
        // creating a zip file for the c-addons on the remote server
        let mut channel = sess.channel_session().unwrap();
        channel.exec(command).unwrap();
    }

    fn downlaod_file(&self, full_path: &str) -> Vec<u8>{
        let sess = self.create_ssh_connection();

        // downloading the zip file in the contents variable
        let (mut remote_file, stat) = sess.scp_recv(Path::new(full_path)).unwrap();
        println!("remote file size: {}", stat.size());
        let mut contents = Vec::new();
        remote_file.read_to_end(&mut contents).unwrap();
        
        // Close the channel and wait for the whole content to be tranferred
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();

        return contents
    }

    fn write_vector_to_file(&self, contents: Vec<u8>, file_path: &str) -> File{
        // writing the contents variable to c-addons.zip file
        let mut file = File::create(file_path).expect("Unable to create file");                                                                                                          
        for i in &contents{
            file.write_all(&[*i]).expect("Unable to write data");
        }
        return file
    }

    pub async fn run_full_backup_process(&self, mode_section: &ModeSection){        
        if self.to_backup_database(){
            log::info!("called backup request for source server");
            let request_response = self.call_database_backup_request().await;
            if request_response.is_ok(){
                mode_section.write_bytes_to_cache_dir("database.zip", &request_response.unwrap());
                log::info!("successfuly backeduped database for source server");
            }
        }
        if self.to_bakup_c_addons(){
            log::info!("ran scp for source server c_addons");
            let zip_path = "/tmp/c-addons.zip";
            let c_addons_path = self.c_addons_path.as_ref().unwrap();
            let zip_command = format!("zip -r {} {}", zip_path, c_addons_path);
            self.run_remote_command(&zip_command);
            let contents = self.downlaod_file(zip_path);
            self.write_vector_to_file(contents, "c-addons.zip");
            log::info!("downloaded source server c_addons");
        }
        if self.to_backup_config_file(){
            log::info!("ran scp for source server config_file");
            let config_file_path = self.config_file_path.as_ref().unwrap();
            let contents = self.downlaod_file(config_file_path);
            self.write_vector_to_file(contents, "odoo.conf");
            log::info!("downloaded source server config_file");
        }
    }

    async fn call_database_backup_request(&self) -> Result<Bytes, Error>{
        let full_url = format!("{}:{}/web/database/backup/", self.address, self.port);
        let request_param = {
            let master_password = self.master_password.to_owned().expect("No Master Password was provided");
            let database_name = self.database_name.to_owned().expect("No Database Name was provided");
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
        let response_body = &response.bytes().await.expect("The odoo response was not correct");
        return Ok(response_body.to_owned())
    }

}
