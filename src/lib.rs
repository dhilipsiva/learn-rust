mod front_of_house;

use crate::front_of_house::hosting::add_to_waitlist;
use std::{error::Error, fs};

pub fn eat_at_restaurant() {
    // front_of_house::hosting::add_to_waitlist();
    add_to_waitlist();
}

pub struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args.get(1).unwrap();
        let file_path = args.get(2).unwrap();
        Ok(Config { query, file_path })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
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
        let query = "foo";
        let content = "\
limp fimp
fimp foop
gloop sloop";
        assert_eq!(vec!["fimp foop"], search(query, content));
    }
}
