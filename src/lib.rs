use std::env;
use std::fs;
use std::process;
use std::error::Error;

pub struct Config
{
    pub query: String, 
    pub filename: String,
}

// Runs shit
pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    for line in search_case_insensitive(&config.query, &contents)
    {
        println!("{}", line);
    }

    Ok(())
}


// Fill the struct by cloning the args
impl Config
{
    pub fn new(args: &[String]) -> Result<Config, &str>
    {
        if args.len() < 3
        {
            return Err("Args number err");
        }

        let config_struct = Config
        {
            query: args[1].clone(),
            filename: args[2].clone(),
        };

        return Ok(config_struct);
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.contains(query)
        {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            result.push(line);
        }
    }

    return result;
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn one_result()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}