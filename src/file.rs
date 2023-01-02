use std::{env, fs, io};
use std::env::VarError;
use std::ffi::OsString;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
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
                config.with_template = self.bool_from_string(s);
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
        //let serd_res;
        let config_file_path = self.config_file_path();
        fs::metadata(&config_file_path)?;

        let mut file = fs::File::open(&config_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // let serd_res =  serde_yaml::from_str(&contents).map_err(|_| {
        //     io::Error::new(
        //         ErrorKind::InvalidData,
        //         "Couldn't parse received string as utf8",
        //     )
        // });
        serde_yaml::from_str(&contents)?
       //  let result;
       //  if let Ok(val) = serde_yaml::from_str(&contents).unwrap(){
       //      result = val
       // } else {
       //     result = Config
       // };
       //  result
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

    fn read_user_input(&self)-> String{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("unable to read user input");
        input.trim().to_string()
    }



   pub fn setup_config_file(&self) {
        loop {
            println!("please provide the absolute path to the directory you would like to store daily logs");
            let user_input = &self.read_user_input();
            if user_input.is_empty() {
                continue;
            }
            let path = Path::new(user_input);

            if !path.is_absolute(){
                println!("The path must be absolute");
                break
            }
            let _res = match self.set_config(ConfigArg::Dir, path.display().to_string()){
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                    continue
                }
            };
            println!("Path {}", path.display())
        }
    }

    fn set_config(&self, config_type: ConfigArg, value: String) -> io::Result<()> {
        let config_path = self.config_file_path();

        let prefix = config_path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        println!(" config path {}", config_path.display());
        // Create file if it doesn't exist, otherwise get it
        let mut file = match fs::File::create(config_path){
            Ok(file ) => {file},
            Err(err) => {
                println!("179:: {}", err);
                return Err(err);
            }
        };


        let mut config = self.config_args();
        match config_type {
            ConfigArg::Dir => config.dir = PathBuf::from(value),
            ConfigArg::Editor => config.editor = value,
            ConfigArg::FileExt => config.file_ext = value,
            ConfigArg::WithTemplate => {
                config.with_template = self.bool_from_string(value);
            }
        }

        let yaml = serde_yaml::to_string(&config);
        file.write_all(yaml.unwrap().as_bytes())
    }
}


