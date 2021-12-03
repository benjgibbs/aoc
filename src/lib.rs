use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Error};

pub fn read_lines(filename: &str) -> Result<Vec<String>, Error> {
    return File::open(filename).and_then(|f| {
        Ok(io::BufReader::new(f)
            .lines()
            .into_iter()
            .flatten()
            .collect())
    });
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

pub fn int_to_bin_string(mut i: isize) -> String {
    let mut result = Vec::new();

    if i == 0 {
        result.push("0")
    } else {
        while i > 0 {
            if i & 0x1 == 0x1 {
                result.push("1");
            } else {
                result.push("0");
            }
            i /= 2;
        }
    }
    result.reverse();
    return result.join("");
}

pub fn bin_string_to_int(bs: &str) -> isize {
    return isize::from_str_radix(bs, 2).unwrap();
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

    #[test]
    fn test_to_bin_string_and_back() {
        assert_eq!(int_to_bin_string(5), "101");
        assert_eq!(bin_string_to_int("101"), 5);
        assert_eq!(int_to_bin_string(63), "111111");
        assert_eq!(bin_string_to_int("111111"), 63);
        assert_eq!(int_to_bin_string(127), "1111111");
        assert_eq!(bin_string_to_int("1111111"), 127);
        assert_eq!(int_to_bin_string(128), "10000000");
        assert_eq!(bin_string_to_int("10000000"), 128);

        assert_eq!(int_to_bin_string(0), "0");
        assert_eq!(bin_string_to_int("0"), 0);
    }
}
