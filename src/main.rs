mod cli;
mod app;
mod file;

use std::{io, process};
use io::Result;
use clap::{Arg,arg, Command, crate_authors, crate_version};
use crate::app::App;
use crate::file::FileOps;
use crate::cli::build_app;

fn main() {

    let result = run();

    match result {
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
        _ => {}
    }
}


fn run() ->Result<bool> {
    let app = build_app();

    let matches = app.get_matches();
    let file_ops = FileOps.new();
    let app = App::new(matches, file_ops);
    match app {
        Err(_error) => {
            let stderr = io::stderr();
            process::exit(1);
        }

        Ok(app) =>{
            app.run();
        }
    }
  return Ok(true)
}

