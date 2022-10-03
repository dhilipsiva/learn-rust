mod front_of_house;

use crate::front_of_house::hosting::add_to_waitlist;
use std::{error::Error, fs};

pub fn eat_at_restaurant() {
    // front_of_house::hosting::add_to_waitlist();
    add_to_waitlist();
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path"),
        };
        Ok(Config { query, file_path })
    }
}

fn search<'a>(query: String, content: &'a String) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    dbg!(&content);
    for line in search(config.query, &content) {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "foo".to_string();
        let content = "\
limp fimp
fimp foop
gloop sloop"
            .to_string();
        assert_eq!(vec!["fimp foop"], search(query, &content));
    }
}
