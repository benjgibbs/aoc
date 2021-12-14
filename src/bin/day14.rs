use std::{collections::HashMap, char};

use aoc::read_lines;

fn read_input(lines: &Vec<String>) -> (Vec<char>, HashMap<String,char>) {
    let template = lines[0].to_string();
    let mut rule_map : HashMap<String, char> = HashMap::new();
    for rule in lines.iter().skip(2) {
        let rule_parts :Vec<&str> = rule.split(" -> ").collect();
        rule_map.insert(rule_parts[0].to_string(), rule_parts[1].as_bytes()[0] as char);
    }

    let buffer : Vec<char> = template.chars().collect();
    (buffer, rule_map)
}

fn next_step(buffer: &Vec<char>, rule_map: &HashMap<String,char>) -> Vec<char> {
    let mut next: Vec<char> = Vec::new();
    next.push(buffer[0]);
    for i in 0..buffer.len() - 1{
        let check : String = [buffer[i], buffer[i+1]].iter().collect();
        if let Some(c) = rule_map.get(&check) {
            next.push(*c);
        }
        next.push(buffer[i+1]);
    }
    next
}

fn get_counts(buffer: &Vec<char>) -> Vec<(char, u64)> {
    let mut counts : HashMap<char, u64> = HashMap::new();
    for c in buffer {
        *counts.entry(*c).or_insert(0u64) += 1;
    }
    let mut minmax : Vec<(char, u64)> = counts.into_iter().collect();
    minmax.sort_by(|a,b| b.1.cmp(&a.1));
    minmax
}

fn part1(lines: &Vec<String>) -> u64 {
    let (mut buffer, rule_map) = read_input(lines);
    
    for _i in 0..10 {
        buffer = next_step(&buffer, &rule_map);
    }

    let minmax = get_counts(&buffer);

    return minmax[0].1 - minmax[minmax.len()-1].1;
}


fn part2(lines: &Vec<String>) -> u64 {
    let (buffer, rule_map) = read_input(lines);

    let rule_map2: HashMap<(char,char), Vec<(char,char)>> = 
        rule_map.iter().map(|kvp| {
            let key = kvp.0.as_bytes();
            let k1 = key[0] as char;
            let k2 = key[1] as char;
            let m = *kvp.1;
            ((k1,k2), Vec::from([(k1,m), (m, k2)]))
            }).collect();

    let mut pair_count: HashMap<(char, char), u64> = HashMap::new();
    for i in 0..buffer.len() -1 {
        let p = (buffer[i], buffer[i+1]);
        *pair_count.entry(p).or_insert(0) += 1;
    }

    for _i in 0..40 {
        let mut next_pair_count: HashMap<(char, char), u64> = HashMap::new();
        for p in pair_count {
            for q in rule_map2[&p.0].iter() {
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
        //6919645078901
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