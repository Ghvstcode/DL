use std::error::Error;
use std::io;
use clap::ArgMatches;
use crate::file::FileOps;

pub struct App {
    pub matches: ArgMatches,
    file_ops: dyn FileOps,
}

impl App {
    pub fn new() -> Result<Self, io::Error> {
        Ok(App {
            matches: ()?,
            file_ops: ()
        })
    }

    // This generates a filename from the current day's date
    // & the file-format specified in the config file
    // This should exist in file ops & should return PathBuf
    fn get_filename_from_date(&self){}

    pub fn run(&self) -> &'static str {
        // If config file does not exist, we should create a new one.
        // New config file should have default values(File has template, File format)
        // If config file exists, we should attempt to see if a file for today's log exists
        // We should have a get filename function that gives us a filename based on today's date
        // if there is a filename param, we should ignore the above function --
        // and attempt to retrieve the file from the directory specified in the config_path.
        // If the file does not exist, We should ask the user if they would like to create the file in that path
        if !self.file_ops.config_file().exists(){
            dbg!("missing config — creating new config file")
        } else {
            // The config has already been setup

            // Check if the new arg was specified
            if let Some(new_dl) = Self.matches.try_get_one("new"){
                // Check if the path arg was specified
                if let Some(path) = Self.matches.try_get_one("path"){
                    println!("Open file at this path {}", path);
                }
                println!("Open file at this path {}", path);
            }

           match Self.matches.try_get_one("path"){
               Some(x) => {
                   match Self.matches.try_get_one("path"){
                       Some()
                   }
               }
           }

            let date = self.get_filename_from_date();
        }
    }
}