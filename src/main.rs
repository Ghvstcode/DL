mod cli;
mod app;
mod file;

use std::{io, process};
use io::Result;
use crate::app::App;

fn main() {
    let result = run();

    match result {
        Err(error) => {
            let stderr = io::stderr();
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}


fn run() ->Result<bool> {
    let app = App::new();
    match app {
        Err(error) => {
            let stderr = io::stderr();
            process::exit(1);
        }

        Ok(app) =>{
            app.run()
        }
    }
  return Ok(true)
}