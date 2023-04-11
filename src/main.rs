use std::{process};

use rsgrep::Config;                     // import "OUR"Library:


fn main() {
    // reading in args:
    let cfg = Config::init().unwrap_or_else(|err|{
        println!("rsgrep-Error: {err}");
        process::exit(1);
    });
    if let Err(e) = rsgrep::run(&cfg) { // no need to unwrap here since its only () anyways
        println!("rsgrep-Error: {e}");
        process::exit(1);
    }
    process::exit(0);
}

