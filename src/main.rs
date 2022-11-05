mod cli;
mod app;
mod file;

use std::{io, process};
use io::Result;
use clap::{crate_authors, crate_version};
use crate::app::App;

fn main() {
    let cli_flags = clap::Command::new("eureka")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Input and store your ideas without leaving the terminal")
        .arg(
            clap::Arg::new(ARG_CLEAR_CONFIG)
                .long(ARG_CLEAR_CONFIG)
                .help("Clear your stored configuration"),
        )
        .arg(
            clap::Arg::new(ARG_VIEW)
                .long(ARG_VIEW)
                .short(ARG_VIEW.chars().next().unwrap())
                .help("View ideas with your $PAGER env variable. If unset use less"),
        )
        .get_matches();
    if cli_flags.get_flag()
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