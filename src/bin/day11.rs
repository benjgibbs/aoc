use core::panic;
use std::collections::{HashSet, VecDeque};

use aoc::read_lines;

struct Cave {
    octopuses: Vec<u32>,
    max_row: usize,
    max_col: usize,
    neighbours: [(i32, i32); 8],
}

impl Cave {
    fn new(input: &Vec<String>) -> Cave {
        let vals: Vec<u32> = input
            .iter()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();

        Cave {
            octopuses: vals,
            max_row: input.get(0).unwrap().len(),
            max_col: input.len(),
            neighbours: [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ],
        }
    }
    fn print(&self) {
        for y in 0..self.max_col {
            for x in 0..self.max_row {
                print!("{}", self.get((x, y)));
            }
            println!();
        }
    }
    fn get(&self, p: (usize, usize)) -> u32 {
        return *self.octopuses.get(self.to_idx(p)).unwrap();
    }

    fn to_point(&self, i: usize) -> (usize, usize) {
        let x = i % self.max_row;
        let y = i / self.max_row;
        (x, y)
    }

    fn to_idx(&self, p: (usize, usize)) -> usize {
        (p.0 + self.max_row * p.1) as usize
    }

    fn valid_neighbours(&self, i: usize) -> Vec<usize> {
        let p = self.to_point(i);
        self.neighbours
            .map(|n| (p.0 as i32 + n.0, p.1 as i32 + n.1))
            .iter()
            .filter(|p| {
                p.0 >= 0
                    && p.1 >= 0
                    && (p.0 as usize) < self.max_row
                    && (p.1 as usize) < self.max_col
            })
            .map(|p| self.to_idx((p.0 as usize, p.1 as usize)))
            .collect()
    }

    fn next(&self) -> (Cave, u32) {
        let mut flash_count = 0u32;

        let mut flash_q = VecDeque::<usize>::new();
        let mut flash_set = HashSet::new();
        let mut o2: Vec<u32> = self.octopuses.iter().map(|x| x + 1).collect();

        for i in 0..o2.len() {
            if o2[i] > 9 && !flash_set.contains(&i) {
                flash_count += 1;
                flash_set.insert(i);
                for n in self.valid_neighbours(i) {
                    flash_q.push_back(n)
                }
            }
        }
        while !flash_q.is_empty() {
            let i = flash_q.pop_front().unwrap();
            o2[i] += 1;

            if o2[i] > 9 && !flash_set.contains(&i) {
                flash_count += 1;
                flash_set.insert(i);
                for n in self.valid_neighbours(i) {
                    flash_q.push_back(n)
                }
            }
        }

        for i in 0..o2.len() {
            if o2[i] > 9 {
                o2[i] = 0;
            }
        }

        (
            Cave {
                octopuses: o2,
                max_row: self.max_row,
                max_col: self.max_col,
                neighbours: self.neighbours,
            },
            flash_count,
        )
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
    while true {
        step_count += 1;
        let (next, count) = cave.next();
        if count == 100 {
            return step_count;
        }
        cave = next;
    }
    panic!("Failed to stop!");
}

fn main() {
    if let Ok(lines) = read_lines("./input/day11.txt") {
        println!("Part 1: {}", part1(&lines, 100));
        println!("Part 2: {}", part2(&lines));
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
            assert_eq!(0, part2(&lines));
        }
    }
}
