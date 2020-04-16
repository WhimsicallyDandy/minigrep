use std::fs;
use std::error::Error;

// returns a Result, which means we can do things with the Error
// the second Result type is a type that implements the Error trait.
// we don't have to specify what the return type will exactly be.
// so we can be more flexible with return Error values
// dyn is short for dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // so the ? means it will return the Error from read_to_string
    // rather than .expect() and our own error handling
    let contents = fs::read_to_string(config.filename)?;    

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    // return the unit type (), which i think is just nothing?
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // takes a reference to a vector of strings to create a struct of String objects (not slices)!
    // returns a Result<Config, str> which is important
    // &'static str is the type of string literals!
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // default constructor, order as arguments appear in code
        Ok(Config { query, filename })
    }

    // Extracting the cmd line arguments into a specific configuration.
    // Uses owned clones rather than string slices, which are references.
    // This is an easier, but a little more inefficient method.
    // However, because we clone the data, we don't need to manage lifetimes
    // so it's a trade-off of simplicity vs performance. And in this case it's
    // worthwhile.
    // the query and filename strings are likely to be very small.
    // as always: Working --then--> Efficiency
}

// we'll be returning a substring of "contents"
// so we need to guarantee the function that it's still around.
// The return data will last as long as contents do
// And we tell exactly which of query or contents we are borrowing from
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    // the vector will not allocate memory until elements are pushed onto it!
    let mut results = Vec::new();
    // line-by-line iteration of strings
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
