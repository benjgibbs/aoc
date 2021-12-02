use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Error};

pub fn read_lines(filename: &str) -> Result<Vec<String>, Error> {
    return File::open(filename)
        .and_then(|f| Ok(io::BufReader::new(f).lines().map(|l| l.unwrap()).collect()));
}

pub fn get_nums(line: &str) -> Vec<i32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let xs = re.captures_iter(line);
    let mut result: Vec<i32> = Vec::new();
    for i in xs {
        let d = i.get(0).unwrap().as_str().parse::<i32>().unwrap();
        result.push(d);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(get_nums("1 2 3 4"), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_multidigits() {
        assert_eq!(get_nums("123 4"), vec![123, 4]);
    }

    #[test]
    fn test_single_digit_in_text() {
        assert_eq!(get_nums("Some text 123 with some other text"), vec![123]);
    }
}
