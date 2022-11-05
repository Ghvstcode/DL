use std::{env, fs, io};
use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;

// pub trait FileOps {
//     // Take in a specific date and populate it with template data
//     // Should only be used if the user wants the template
//     fn generate_dl_file(&self) -> Result<(), E>;
//     fn generate_config_file(&self) -> Result<(), E>;
//     fn config_file(&self) -> PathBuf;
//     fn file_exists(&self) -> PathBuf;
//     // Since we default to using date as file names
//     // This function takes in a date in String format
//     // it returns a full path based on the root dir in the config & The file type in the config
//    fn path_from_date(&self, date: &String) -> PathBuf;
// }
pub(crate) struct FileOps;

impl FileOps {
    pub fn generate_dl_file(&self) -> Result<(), io::Error> {
        todo!()
    }

    pub fn generate_config_file(&self) -> Result<(), io::Error> {
        todo!()
    }

    pub fn config_file(&self) -> PathBuf {
        todo!()
    }

    pub fn file_exists(&self) -> PathBuf {
        todo!()
    }

    // Since we default to using date as file names
    // This function takes in a date in String format
    // it returns a full path based on the root dir in the config & The file type in the config
   pub fn path_from_date(&self, date: &mut String) -> PathBuf{
       // let file_with_ext = date.push_str(".md");
       // // This should include the specified file extension
       //  PathBuf::from(root_path).join(file_with_ext)
        todo!()
   }
}


