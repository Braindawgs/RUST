use std::process;
use std::env;
 
extern crate rust_cli;
use rust_cli::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let confg = Config::new(&args).unwrap_or_else(|err|
        {
            println!("Problem parsing arg: {}", err);
            process::exit(1);
        }
    ); 

    if let Err(e) =  rust_cli::run(confg)
    {
        println!("App error:{}", e);
        process::exit(1);
    }
}


