use std::{env, process};

use minigrep::Config;

extern crate minigrep;

fn main(){
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    
    if let Err(message) = minigrep::run(config){
        eprintln!("Application error: {message}");
        process::exit(1);
    }
    
}

