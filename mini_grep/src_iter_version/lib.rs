use std::fs;
use std::error::Error;
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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // 传入一个迭代器参数，并取得所有权
        if args.len() < 3 {
            return Err("not enough arguments!");
        }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("didn't get a query string");
            }
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("didn't get a filename string");
            }
        };
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
    // let mut result = Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result

    // 使用迭代器
    content.lines()
        .filter(|x| x.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // let query = query.to_lowercase();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    // result
    // 使用迭代器
    content.lines()
        .filter(|x| x.to_lowercase().contains(query))
        .collect()
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

