use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::path::Path;

pub fn read_lines(path: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(&path)?;
    Ok(BufReader::new(file).lines())
}

pub fn read_key_value(line: &str, delimiter: &str) -> (String, String) {
    let kv: Vec<&str> = Regex::new(delimiter).unwrap().split(&line).collect();
    (String::from(kv[0]), String::from(kv[1]))
}
