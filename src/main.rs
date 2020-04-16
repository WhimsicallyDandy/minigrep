use std::env;
use std::process;


use minigrep::Config;

// note we only have error messages printing in main.rs so far
fn main() {
    // turns cmd line arguments into a String Vector. Need to specify type, as collect can do a lot of data structures
    let args: Vec<String> = env::args().collect();
    // debug formatting is {:?} for vectors and other collections
    // &args[0] is the binary executable name

    // unwrap_or_else will unwrap the Ok value
    // otherwise, it will perform the code in this anonymous function
    // with the err in vertical bars (|err|) containing the value
    // passed in the Err from the Config constructor
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // execute run(), and if there is an Error, do this
    // used instead of above "let config = " because we don't care about
    // keeping a return value, as a successful execution just returns "()"
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
}

