use std::fs;
use std::io::{Read, Write, stdin};
use std::result::Result;
use std::error::Error;
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
extern crate serde_json;

type EResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvData {
    env_file_name: String,
    git_folder_path: String,
}

#[allow(dead_code)]
#[allow(unused_must_use)]
impl EnvData {
    const ENV_FILE_NAME: &str = ".env";

    pub fn load() -> EResult<Self> {
        let mut file = File::open(EnvData::ENV_FILE_NAME)?;
        let mut buf = vec![];
        match file.read_to_end(&mut buf).is_ok() {
            true => {
                let env_data = serde_json::from_slice::<EnvData>(&buf[..])?;
                Ok(EnvData{
                        env_file_name: String::from(EnvData::ENV_FILE_NAME),
                        git_folder_path: env_data.git_folder_path,
                })
                    
            },
            false => Err("load error".into())
        }
    }

    pub fn get_git_folder_path(&self) -> String {
        self.git_folder_path.clone()
    }

    pub fn check_env_file_exists() -> bool {
        std::path::Path::new(EnvData::ENV_FILE_NAME).exists()
    }

    pub fn create_new(git_folder_path: &str) -> EResult<Self> {
        Ok(EnvData{
            env_file_name: String::from(EnvData::ENV_FILE_NAME),
            git_folder_path: String::from(git_folder_path),
        })
    }

    pub fn new() -> EResult<Self> {
        println!("Please input your repository root: ");
        let mut path: String = String::new();
        stdin().read_line(&mut path).expect("Failed to read input");

        // save local file
        EnvData::create_env_file(&EnvData{
                                        env_file_name: EnvData::ENV_FILE_NAME.to_string(),
                                        git_folder_path: path.trim().to_string()}).unwrap();
        EnvData::create_new(path.trim())
    }

    pub fn create_env_file(envdata: &EnvData) -> EResult<()> {
        let mut f = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open(&envdata.env_file_name)?;
        match f.write_all(serde_json::to_string(envdata)?.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => panic!("write error")
        }
    }

    pub fn update_env_file(&mut self, git_folder_path: &str) -> EResult<()> {
        match EnvData::check_env_file_exists() {
            true => {
                fs::remove_file(&self.env_file_name).expect("File delete failed");
                self.git_folder_path = String::from(git_folder_path);
            },
            false => {},
        }
        EnvData::create_env_file(&self)
    }

}
