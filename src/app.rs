use std::error::Error;
use std::io;
use clap::{ArgMatches, crate_authors, crate_version};
use crate::file::FileOps;
use std::path::{Path, PathBuf};
use chrono::{Utc};

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

    fn get_todays_date(){
        let now = Utc::now();
        println!(
            "The current UTC date is {}-{:02}-{:02} {:?} ({})",
            year,
            now.month(),
            now.day(),
            now.weekday(),
            if is_common_era { "CE" } else { "BCE" }
        )
    }
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
            if let Some(_new_dl) = Self.matches{
                // Check if the path arg was specified
                if let Some(path) = Self.matches.try_get_one("path"){
                   let path_b = PathBuf::from(path);
                    // Check if the file path already exists,
                    if path_b.exists(){
                        println!("cannot open a new file with name {} it already exists", path)
                    }
                    // Call FileOps to generate new DL file.
                    println!("Open file at this path {}", path);
                }

                // Call FileOps to generate new DL file.
                println!("Open file at this path {}", path)
            }
        }
    }
}