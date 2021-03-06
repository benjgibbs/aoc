use aoc::{get_nums, read_lines};
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Board {
    board: Vec<Vec<i32>>,
    rows: Vec<HashSet<i32>>,
    cols: Vec<HashSet<i32>>,
}

impl Board {
    pub fn new(b: Vec<Vec<i32>>) -> Board {
        Board {
            rows: Board::rows(&b),
            cols: Board::cols(&b),
            board: b,
        }
    }
    pub fn bingo(&self, calls: &HashSet<i32>) -> bool {
        for r in self.rows.iter() {
            if r.is_subset(calls) {
                return true;
            }
        }
        for c in self.cols.iter() {
            if c.is_subset(calls) {
                return true;
            }
        }
        return false;
    }

    pub fn score(&self, calls: &HashSet<i32>, last_call: i32) -> i32 {
        let mut result = 0;
        for i in 0..self.board.len() {
            for j in 0..self.board.get(i).unwrap().len() {
                let n = self.board.get(i).unwrap().get(j).unwrap();
                if !calls.contains(n) {
                    result += n;
                }
            }
        }
        return result * last_call;
    }

    fn rows(board: &Vec<Vec<i32>>) -> Vec<HashSet<i32>> {
        return board
            .iter()
            .map(|r| HashSet::from_iter(r.iter().cloned()))
            .collect();
    }
    fn cols(board: &Vec<Vec<i32>>) -> Vec<HashSet<i32>> {
        let mut result = Vec::new();
        for i in 0..board.get(0).unwrap().len() {
            let mut c = HashSet::new();
            for j in 0..board.len() {
                c.insert(*board.get(j).unwrap().get(i).unwrap());
            }
            result.push(c);
        }
        return result;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input/day4.txt") {
        let calls = get_nums(lines.get(0).unwrap());

        let mut boards = Vec::new();
        let mut board = Vec::new();

        for i in 2..lines.len() {
            let line = lines.get(i).unwrap();
            if line.trim().len() == 0 {
                boards.push(Board::new(board));
                board = Vec::new();
            } else {
                board.push(get_nums(line));
            }
        }
        let mut called = HashSet::new();
        let mut won: HashSet<usize> = HashSet::new();
        let mut winner: Option<i32> = None;

        for i in 0..calls.len() {
            called.insert(*calls.get(i).unwrap());
            for bi in 0..boards.len() {
                if !won.contains(&bi) {
                    let b = boards.get(bi).unwrap();
                    if b.bingo(&called) {
                        let score = b.score(&called, *calls.get(i).unwrap());
                        if winner.is_none() {
                            println!("Part 1 (32844): {}", score);
                        }
                        winner = Some(score);
                        won.insert(bi);
                    }
                }
            }
        }
        println!("Part 2 (4920): {}", winner.unwrap());
    }
}
