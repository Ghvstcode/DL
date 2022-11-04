use std::{env, fs, io};
use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;

pub trait FileOps {
    // Take in a specific date and populate it with template data
    // Should only be used if the user wants the template
    //
    fn generate_dl_file(&self) -> Result<(), E>;
    fn generate_config_file(&self) -> Result<(), E>;
    fn config_file(&self) -> PathBuf;
    fn file_exists(&self) -> PathBuf;
}
struct File;

impl FileOps for File {
    fn generate_dl_file(&self) -> Result<(), E> {
        todo!()
    }

    fn generate_config_file(&self) -> Result<(), E> {
        todo!()
    }

    fn config_file(&self) -> PathBuf {
        todo!()
    }

    fn file_exists(&self) -> PathBuf {
        todo!()
    }
}

