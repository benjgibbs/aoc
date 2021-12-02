use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_nums(line: &String) -> Vec<i32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let xs = re.captures(line).unwrap();
    let mut result: Vec<i32> = Vec::new();
    for i in 0..xs.len() {
        let d = xs.get(i).unwrap().as_str().parse::<i32>().unwrap();
        result.push(d);
    }
    return result;
}
