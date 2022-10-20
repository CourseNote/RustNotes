use std::fs;
use::std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 只关心是否出现
        let value = env::var("CASE_SENSITIVE").unwrap(); // 用值来区分
        let case_sensitive = if value.parse::<i32>().unwrap() == 1 {
            true
        } else {
            false
        };
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn one_result() {
        let query = "duct";
        let content = "
Rust:\n
safe, fast, productive.\n
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "
Rust:\n
safe, fast, productive.\n
Pick three.\n
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let content = "
Rust:\n
safe, fast, productive.\n
Pick three.\n
trust me.";

        assert_eq!(
            vec!["Rust:", "trust me."],
            search_case_insensitive(query, content)
        )
    }
}

