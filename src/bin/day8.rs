use std::collections::{BTreeSet, HashMap, HashSet};

use aoc::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./input/day8.txt") {
        let mut count = 0;
        for line in lines.iter() {
            let m = line
                .split("|")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .split(" ")
                .filter(|s| HashSet::from([2, 4, 3, 7]).contains(&s.len()))
                .count();
            count += m;
        }
        println!("Part1 (530): {}", count);

        println!("Part2 (1051087): {}", sum_decoded_lines(&lines));
    }
}

fn sum_decoded_lines(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines.iter() {
        let num = decode_line(line);
        sum += num;
    }
    return sum;
}

fn get_singleton<'a>(m: &'a HashMap<usize, HashSet<BTreeSet<char>>>, i: usize) -> &'a BTreeSet<char> {
    return m.get(&i).unwrap().iter().nth(0).unwrap();
}

fn get_predicated<'a, F>(m: &'a HashMap<usize, HashSet<BTreeSet<char>>>, sz: usize,  p: F) 
    -> &'a BTreeSet<char> 
where
    F: FnMut(&&BTreeSet<char>) -> bool {
    return m
        .get(&sz)
        .unwrap()
        .iter()
        .filter(p)
        .nth(0)
        .unwrap();
}

fn decode_line(line: &str) -> i32 {
    let mut counts: HashMap<usize, HashSet<BTreeSet<char>>> = HashMap::new();
    for w in line.replace(" | ", " ").split(" ") {
        let s: BTreeSet<char> = w.chars().collect();
        counts
            .entry(w.len())
            .or_insert(HashSet::<BTreeSet<char>>::new())
            .insert(s);
    }

    let one = get_singleton(&counts, 2);
    let four = get_singleton(&counts, 4);
    let seven = get_singleton(&counts, 3);
    let eight = get_singleton(&counts, 7);

    let nine = get_predicated(&counts, 6, |s| s.is_superset(four));
    let three = get_predicated(&counts, 5, |s| s.is_superset(one));
    let five = get_predicated(&counts, 5, |s| s.is_subset(nine) && *s != three);
    let two = get_predicated(&counts, 5, |s| *s != three && *s != five);
    
    let top_right = one.intersection(two).nth(0).unwrap();

    let zero = get_predicated(&counts, 6, |s| *s != nine && s.contains(top_right));
    let six = get_predicated(&counts, 6, |s| *s != zero && *s != nine);

    let lookup = HashMap::from([
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
        (zero, 0),
    ]);

    let mut pos = 1;
    let mut num = 0;
    for result in line
        .split("|")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split(" ")
        .filter(|s| s.trim().len() > 0)
        .collect::<Vec<&str>>()
        .iter()
        .rev()
    {
        let resultid: BTreeSet<char> = result.chars().collect();
        num += pos * lookup[&resultid];
        pos *= 10;
    }
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part2() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        assert_eq!(8394, decode_line(input));
    }

    #[test]
    fn question_example() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(5353, decode_line(input));
    }

    #[test]
    fn full_day_18_part_2() {
        if let Ok(lines) = read_lines("./input/example8.txt") {
            assert_eq!(61229, sum_decoded_lines(&lines));
        }
        if let Ok(lines) = read_lines("./input/day8.txt") {
            assert_eq!(1051087, sum_decoded_lines(&lines));
        }
    }
}
