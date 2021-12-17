use std::{collections::{HashMap, BTreeMap}, char};

use aoc::read_lines;

fn read_input(lines: &Vec<String>) -> (Vec<char>, HashMap<(char, char),Vec<(char, char)>>) {
    let template = lines[0].to_string();
    let mut rule_map : HashMap<(char,char), Vec<(char,char)>> = HashMap::new();
    for rule in lines.iter().skip(2) {
        let rule_parts :Vec<&str> = rule.split(" -> ").collect();
        let pair = rule_parts[0].to_string();
        let p1 = pair.as_bytes()[0] as char;
        let p2  = pair.as_bytes()[1] as char;
        let m = rule_parts[1].as_bytes()[0] as char;
        rule_map.insert((p1,p2), Vec::from([(p1,m), (m,p2)]) );
    }
    let buffer : Vec<char> = template.chars().collect();
    (buffer, rule_map)
}

fn part1(lines: &Vec<String>) -> u64 {
    run(lines, 10)
}

fn part2(lines: &Vec<String>) -> u64 {
    run(lines, 40)
}

fn run(lines: &Vec<String>, iters: u16) -> u64 {
    let (buffer, rule_map) = read_input(lines);
    
    let mut pair_count: BTreeMap<(char, char), u64> = BTreeMap::new();
    for i in 0..buffer.len() -1 {
        let p = (buffer[i], buffer[i+1]);
        *pair_count.entry(p).or_insert(0) += 1;
    }

    for _i in 0..iters {
        let mut next_pair_count: BTreeMap<(char, char), u64> = BTreeMap::new();
        for p in pair_count {
            for q in rule_map[&p.0].iter() {
                *next_pair_count.entry(*q).or_insert(0) += p.1;
            }
        }
        pair_count = next_pair_count;
    }
    let mut sums : HashMap<char, u64> = HashMap::new();
    for p in pair_count {
        *sums.entry(p.0.0).or_insert(0) += p.1;
        *sums.entry(p.0.1).or_insert(0) += p.1;
    }
    
    let mut minmax : Vec<(char, u64)> = sums.into_iter().collect();
    minmax.sort_by(|a,b| b.1.cmp(&a.1));

    return (minmax[0].1 - minmax[minmax.len()-1].1 + 1)/2;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day14.txt") {
        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_example1() {
        if let Ok(lines) = read_lines("./input/example14.txt") {
            assert_eq!(1588, part1(&lines));
        }
    }

    #[test]
    fn day14_example2() {
        if let Ok(lines) = read_lines("./input/example14.txt") {
            assert_eq!(2188189693529, part2(&lines));
        }
    }

    #[test]
    fn day14_part1() {
        if let Ok(lines) = read_lines("./input/day14.txt") {
            assert_eq!(3009, part1(&lines));
        }
    }

    #[test]
    fn day14_part2() {
        if let Ok(lines) = read_lines("./input/day14.txt") {
            assert_eq!(3459822539451, part2(&lines));
        }
    }
}