use std::collections::{VecDeque, HashMap};

use aoc::read_lines;

enum CorruptedResult {
    Corrupted(char),
    Incomplete(VecDeque<char>)
}

use CorruptedResult::*;

fn line_check(line: &str) -> CorruptedResult {
    let mut q : VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match c {
            '[' | '(' | '{' | '<' => q.push_front(c),
            ']' => if q.pop_front() != Some('[') { return Corrupted(c) },
            ')' => if q.pop_front() != Some('(') { return Corrupted(c) },
            '}' => if q.pop_front() != Some('{') { return Corrupted(c) },
            '>' => if q.pop_front() != Some('<') { return Corrupted(c) },
            _ => panic!("Unexpected: {}", c)
        }
    }
    return Incomplete(q);
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        if let Corrupted(c) = line_check(&line) {
            sum += match  c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Unexpected char")
            }
        }
    }
    return sum;
}

fn completion_score(q: &VecDeque<char>) -> u64 {
    let scores = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4)
    ]);
    let mut score = 0;
    for c in q {
        score *= 5;
        score += scores[c];
    }
    return score;
}


fn part2(lines: &Vec<String>) -> u64 {
    let mut scores = Vec::new();
    for line in lines {
        if let Incomplete(q) = line_check(&line) {
            scores.push(completion_score(&q));
        }
    }
    scores.sort();
    return scores[scores.len()/2];
}

fn main() {
    if let Ok(lines) = read_lines("./input/day10.txt") {
        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_example1() {
        if let Ok(lines) = read_lines("./input/example10.txt") {
            assert_eq!(26397, part1(&lines));
        }
    }

    #[test]
    fn day10_example2() {
        if let Ok(lines) = read_lines("./input/example10.txt") {
            assert_eq!(288957, part2(&lines));
        }
    }

}