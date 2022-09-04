use bytes::Bytes;
use reqwest::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Server {
    pub address: Option<String>,
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
            address: Some("127.0.0.1:8069".to_string()),
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
            self.address = Some(value);
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

    pub async fn call_database_backup_request(&self) -> Result<Bytes, Error>{
        let full_url = {
            let mut string = "".to_string();
            string.push_str( self.address.to_owned().expect("No Addres was provided").as_ref() );
            string.push_str( "/web/database/backup/" );
            string
        };
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
        dbg!(&full_url);
        let response = client.post(full_url)
            .form(&request_param)
            .send()
            .await
            .expect("Something wrong occurs in the call");
        let response_body = &response.bytes().await.expect("The odoo response was not correct");
        return Ok(response_body.to_owned())
    }

}
