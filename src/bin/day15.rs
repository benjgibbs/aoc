use std::{
    cmp::{max, Ordering},
    collections::BinaryHeap,
};

use aoc::read_lines;

struct Cave {
    risk: Vec<u32>,
    max_x: usize,
    max_y: usize,
}

impl Cave {
    pub fn new(lines: &Vec<String>) -> Cave {
        let mut risk: Vec<u32> = Vec::new();
        let mut max_x: usize = 0;

        for line in lines.iter() {
            max_x = max(max_x, line.len());
            risk.append(&mut line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }
        Cave {
            risk: risk,
            max_x: max_x,
            max_y: lines.len(),
        }
    }

    pub fn risk(&self, i: usize) -> u32 {
        self.risk[i]
    }

    pub fn risk_by_pos(&self, p: (usize, usize)) -> u32 {
        self.risk[self.pos_to_idx(p)]
    }

    pub fn neighbours(&self, i: usize) -> Vec<usize> {
        let adjacents: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let pos = self.idx_to_pos(i);
        adjacents
            .iter()
            .map(|a| (pos.0 as i32 + a.0, pos.1 as i32 + a.1))
            .filter(|p| {
                p.0 >= 0 && p.1 >= 0 && (p.0 as usize) < self.max_x && (p.1 as usize) < self.max_y
            })
            .map(|p| self.pos_to_idx((p.0 as usize, p.1 as usize)))
            .collect()
    }

    fn idx_to_pos(&self, i: usize) -> (usize, usize) {
        (i % self.max_x, i / self.max_x)
    }

    fn pos_to_idx(&self, pos: (usize, usize)) -> usize {
        pos.0 + pos.1 * self.max_x
    }

    fn extend(&self, by: usize) -> Cave {
        let mut risks = Vec::new();

        for y in 0..self.max_y * by {
            for x in 0..self.max_x * by {
                let risk = if x < self.max_x && y < self.max_x {
                    self.risk_by_pos((x, y))
                } else if y < self.max_y {
                    // x >= max
                    let xpos = x - self.max_x;
                    let ypos = y;
                    risks[xpos + ypos * self.max_x * by] + 1
                } else {
                    let xpos = x;
                    let ypos = y - self.max_y;
                    risks[xpos + ypos * self.max_x * by] + 1
                };

                risks.push(max(risk % 10, 1));
            }
        }

        Cave {
            risk: risks,
            max_x: self.max_x * by,
            max_y: self.max_y * by,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    risk: u32,
    pos: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(cave: Cave) -> Option<u32> {
    let end = cave.pos_to_idx((cave.max_x - 1, cave.max_y - 1));
    let mut heap = BinaryHeap::new();
    heap.push(State { risk: 0, pos: 0 });

    let mut min_risks: Vec<_> = (0..cave.max_x * cave.max_y).map(|_| u32::MAX).collect();

    while let Some(state) = heap.pop() {
        if state.pos == end {
            return Some(state.risk);
        }

        if state.risk > min_risks[state.pos] {
            continue;
        }

        for n in cave.neighbours(state.pos) {
            let next = State {
                risk: state.risk + cave.risk(n),
                pos: n,
            };
            if next.risk < min_risks[next.pos] {
                heap.push(next);
                min_risks[next.pos] = next.risk;
            }
        }
    }
    None
}

fn part1(lines: &Vec<String>) -> Option<u32> {
    let cave = Cave::new(lines);
    search(cave)
}

fn part2(lines: &Vec<String>) -> Option<u32> {
    let mut cave = Cave::new(lines);
    cave = cave.extend(5);
    search(cave)
}

fn main() {
    if let Ok(lines) = read_lines("./input/day15.txt") {
        println!("Part 1: {:?}", part1(&lines));
        println!("Part 2: {:?}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_example1() {
        if let Ok(lines) = read_lines("./input/example15.txt") {
            assert_eq!(Some(40), part1(&lines));
        }
    }

    #[test]
    fn day15_example2() {
        if let Ok(lines) = read_lines("./input/example15.txt") {
            assert_eq!(Some(315), part2(&lines));
        }
    }
}
