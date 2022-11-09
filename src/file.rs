use std::{env, fs, io};
use std::env::VarError;
use std::ffi::OsString;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use dirs::{config_dir, home_dir};
use serde::{Serialize, Deserialize};
use serde_yaml::Value::Null;

extern crate dirs;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
    pub dir: PathBuf,
    pub file_ext: String,
    pub editor: String,
    pub with_template: bool
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigArg {
    Dir,
    FileExt,
    Editor,
    WithTemplate
}

pub struct FileOps;

impl FileOps {
    pub fn new(&self) -> Self {
        return FileOps
    }
    pub fn generate_dl_file(&self, with_template:bool, file_path: &PathBuf) -> Result<(), io::Error> {
        let mut template = format!(r#"## {}"#, file_path.file_name().expect("invalid file name").to_string_lossy());

        if with_template{
            template.push_str("An extension of the template")
        }

        fs::write(&file_path, template)?;


        Ok(())
    }

    pub fn generate_config_file(&self) -> Result<(), io::Error> {
        todo!()
    }

    pub fn config_file_path(&self) -> PathBuf{
        home_dir().unwrap().join(".config").join("daily_logger").join("daily_logger.yaml")
    }


    pub fn get_args_from_env_var(&self, key: &str) -> Result<String, VarError> {
        let mut full_key = String::from(key);
        full_key.insert_str(0, "DL_");
        env::var(full_key)
    }

    pub fn config_args(&self) -> Config {
        let mut config = match self.get_cfg_file_args() {
            Ok(config) => config,
            Err(err) => {
                dbg!(err);
                Config::default()
            }
        };

        match self.get_args_from_env_var("DIR") {
            Ok(s) => {config.dir = s.parse().unwrap()},
            _ => {}
        }

        match self.get_args_from_env_var("EXT") {
            Ok(s) => config.dir = s.parse().unwrap(),
            _ => {}
        }

        match self.get_args_from_env_var("TEMPL") {
            Ok(s) => {
                config.with_template = self.bool_from_string(value);;
            }
            _ => {
                config.file_ext = String::from(".md").parse().unwrap()
            }
        }

        match self.get_args_from_env_var("EDITOR") {
            Ok(s) => config.editor = s.parse().unwrap(),
            _ => {}
        }

        config
    }

    pub fn get_cfg_file_args(&self) -> io::Result<Config> {
        let config_file_path = self.config_file_path();
        fs::metadata(&config_file_path)?;

        let mut file = fs::File::open(&config_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_yaml::from_str(&contents).unwrap())
    }

   pub fn path_from_date(&self, mut date: String) -> PathBuf{
       let config = self.config_args();

        let file_ext = config.file_ext;
       date.push_str(&*file_ext);
        let dir = config.dir.join(date);

        return dir;
   }

    pub fn open_program(&self, file_path: PathBuf, program: &str) -> ExitStatus {
        Command::new(&program).arg(&file_path).status().expect("failed to execute process")
    }

    fn bool_from_string(&self, value: String) -> bool{
        let truth_value: bool = match value.as_str() {
            "true" => true,
            "t" => true,
            "false" => false,
            "f" => false,
            _ => false
        };

        return truth_value
    }
    pub fn set_config(&self, config_type: ConfigArg, value: String) -> io::Result<()> {
        let config_path = self.config_file_path();

        // Create file if it doesn't exist, otherwise get it
        let mut file = fs::File::create(config_path)?;

        let mut config = self.config_args();
        match config_type {
            ConfigArg::Dir => config.dir = PathBuf::from(value),
            ConfigArg::Editor => config.editor = value,
            ConfigArg::FileExt => config.file_ext = value,
            ConfigArg::WithTemplate => {
                config.with_template = self.bool_from_string(value);;
            }
        }

        let json = serde_json::to_string(&config)?;

        file.write_all(json.as_bytes())
    }
}


