use std::collections::{HashSet, VecDeque};

use aoc::read_lines;

struct Cave {
    octopuses: Vec<u32>,
}

const MAX_ROW: usize = 10;
const MAX_COL: usize = 10;
const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Cave {
    fn new(input: &Vec<String>) -> Cave {
        let vals: Vec<u32> = input
            .iter()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();

        Cave { octopuses: vals }
    }

    fn to_point(&self, i: usize) -> (usize, usize) {
        let x = i % MAX_ROW;
        let y = i / MAX_ROW;
        (x, y)
    }

    fn to_idx(&self, p: (usize, usize)) -> usize {
        (p.0 + MAX_ROW * p.1) as usize
    }

    fn valid_neighbours(&self, i: usize) -> Vec<usize> {
        let p = self.to_point(i);
        NEIGHBOURS
            .map(|n| (p.0 as i32 + n.0, p.1 as i32 + n.1))
            .iter()
            .filter(|p| {
                p.0 >= 0 && p.1 >= 0 && (p.0 as usize) < MAX_ROW && (p.1 as usize) < MAX_COL
            })
            .map(|p| self.to_idx((p.0 as usize, p.1 as usize)))
            .collect()
    }

    fn increment(&self, i: usize, o2: &mut Vec<u32>, flash_set: &mut HashSet<usize>, flash_q: &mut VecDeque<usize>) -> u32 {
        o2[i] += 1;
        if o2[i] > 9 && !flash_set.contains(&i) {
            flash_set.insert(i);
            for n in self.valid_neighbours(i) {
                flash_q.push_back(n)
            }
            return 1;
        }
        return 0;
    }

    fn next(&self) -> (Cave, u32) {
        let mut flash_count = 0u32;

        let mut flash_q = VecDeque::<usize>::new();
        let mut flash_set = HashSet::new();
        let mut o2: Vec<u32> = self.octopuses.clone();

        for i in 0..o2.len() {
            flash_count += self.increment(i, &mut o2, &mut flash_set, &mut flash_q);
        }

        while !flash_q.is_empty() {
            let i = flash_q.pop_front().unwrap();
            flash_count += self.increment(i, &mut o2, &mut flash_set, &mut flash_q);
        }

        for i in 0..o2.len() {
            if o2[i] > 9 {
                o2[i] = 0;
            }
        }

        (Cave { octopuses: o2 }, flash_count)
    }
}

fn part1(lines: &Vec<String>, steps: u32) -> u32 {
    let mut cave = Cave::new(lines);
    let mut flash_count = 0;
    for _i in 0..steps {
        let (next, count) = cave.next();
        cave = next;
        flash_count += count;
    }
    flash_count
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut cave = Cave::new(lines);
    let mut step_count = 0;
    loop {
        step_count += 1;
        let (next, count) = cave.next();
        if count == 100 {
            return step_count;
        }
        cave = next;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input/day11.txt") {
        println!("Part 1 (1793): {}", part1(&lines, 100));
        println!("Part 2 (247): {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_example1() {
        if let Ok(lines) = read_lines("./input/example11.txt") {
            assert_eq!(204, part1(&lines, 10));
            assert_eq!(1656, part1(&lines, 100));
        }
    }

    #[test]
    fn day11_example2() {
        if let Ok(lines) = read_lines("./input/example11.txt") {
            assert_eq!(195, part2(&lines));
        }
    }
}
