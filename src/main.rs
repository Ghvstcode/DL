mod cli;

use std::{io, process};
use io::Result;

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
  return Ok(true)
}