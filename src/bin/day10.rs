use std::collections::{VecDeque, HashMap};

use aoc::read_lines;

fn is_corrupted(line: &str) -> bool {
    let mut q : VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match c {
            '[' | '(' | '{' | '<' => q.push_front(c),
            ']' => if q.pop_front() != Some('[') { return true },
            ')' => if q.pop_front() != Some('(') { return true },
            '}' => if q.pop_front() != Some('{') { return true },
            '>' => if q.pop_front() != Some('<') { return true },
            _ => panic!("Unexpected: {}", c)
        }
    }
    return false;
}

fn first_corrupted_char(line: &str) -> char {
    let mut q : VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match c {
            '[' | '(' | '{' | '<' => q.push_front(c),
            ']' => if q.pop_front() != Some('[') { return c },
            ')' => if q.pop_front() != Some('(') { return c },
            '}' => if q.pop_front() != Some('{') { return c },
            '>' => if q.pop_front() != Some('<') { return c },
            _ => panic!("Unexpected: {}", c)
        }
    }
    panic!("not corrupted");
}

fn required_to_complete(line: &str) -> VecDeque<char> {
    let mut q : VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        match c {
            '[' | '(' | '{' | '<' => q.push_front(c),
            ']' => if q.pop_front() != Some('[') { panic!() },
            ')' => if q.pop_front() != Some('(') { panic!() },
            '}' => if q.pop_front() != Some('{') { panic!() },
            '>' => if q.pop_front() != Some('<') { panic!() },
            _ => panic!("Unexpected: {}", c)
        }
    }
    return q;
}


fn part1(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        if is_corrupted(&line) {
            let c = first_corrupted_char(line);
            println!("{} => {}", line, c);
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
        if !is_corrupted(&line) {
            let q = required_to_complete(&line);
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

    #[test]
    fn check_corrupted() {
        for line in ["{([(<{}[<>[]}>{[]{[(<()>", "[[<[([]))<([[{}[[()]]]", "[{[{({}]{}}([{[{{{}}([]", "[<(<(<(<{}))><([]([]()", "<{([([[(<>()){}]>(<<{{"] {
            println!("{}", line);
            assert!(is_corrupted(line));
        }
    }

    #[test]
    fn check_not_corrupted() {
        for line in ["[({(<(())[]>[[{[]{<()<>>", "[(()[<>])]({[<{<<[]>>(", "(((({<>}<{<{<>}{[]{[]{}", "{<[[]]>}<{[{[{[]{()[[[]", "<{([{{}}[<[[[<>{}]]]>[]]"] {
            println!("{}", line);
            assert!(!is_corrupted(line));
        }
    }

    #[test]
    fn reuturns_corruped_char() {
        let c = first_corrupted_char("<{([([[(<>()){}]>(<<{{");
        assert_eq!('>', c);

    }
}