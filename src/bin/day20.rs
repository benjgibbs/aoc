use std::collections::HashSet;

use aoc::read_lines;
use bitvec::{order::Lsb0, prelude::BitVec};

fn read_enancement(line: &str) -> BitVec<Lsb0, u32> {
    let mut result = BitVec::<Lsb0, u32>::new();
    for c in line.chars() {
        result.push(c == '#')
    }
    return result;
}

fn neighbours(p: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for y in p.1 - 1..=p.1 + 1 {
        for x in p.0 - 1..=p.0 + 1 {
            result.push((x, y));
        }
    }
    result.reverse();
    result
}

fn iterate(
    input: &HashSet<(i32, i32)>,
    lookup: &BitVec<Lsb0, u32>,
    default: bool,
) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    let minx = input.iter().map(|x| x.0).min().unwrap();
    let miny = input.iter().map(|x| x.1).min().unwrap();
    let maxx = input.iter().map(|x| x.0).max().unwrap();
    let maxy = input.iter().map(|x| x.1).max().unwrap();

    for y in (miny-1)..=(maxy+1) {
        for x in (minx-1)..=(maxx+1) {
            let bit = (x, y);
            let nbs = neighbours(&bit);

            let mut num = 0;
            let mut base = 1;
            for n in nbs {
                if input.contains(&n)
                    || (default && (n.1 < miny || n.0 < minx || n.1 > maxy || n.0 > maxx))
                {
                    num += base;
                }
                base *= 2;
            }
            if lookup[num] {
                result.insert(bit);
            }
        }
    }

    result
}

fn part1(lines: &Vec<String>) -> i32 {
    solve(lines, 2)
}

fn solve(lines: &Vec<String>, n: i32) -> i32 {
    let lookup = read_enancement(&lines[0]);

    let mut set_bits: HashSet<(i32, i32)> = HashSet::new();

    for i in 2..lines.len() {
        let line: Vec<char> = lines[i].chars().collect();
        for j in 0..line.len() {
            if line[j] == '#' {
                set_bits.insert((j as i32, (i - 2) as i32));
            }
        }
    }

    let mut default = false;
    for _i in 0..n {
        set_bits = iterate(&set_bits, &lookup, default);
        if lookup[0] {
            default = !default;
        }
    }

    return set_bits.len() as i32;
}

fn part2(lines: &Vec<String>) -> i32 {
    solve(lines, 50)
}

fn main() {
    if let Ok(lines) = read_lines("./input/day20.txt") {
        println!("Part 1 (5647): {}", part1(&lines));
        println!("Part 2 (15653): {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_enhancement() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##";
        let result = read_enancement(input);
        assert_eq!(true, result[34]);
        assert_eq!(false, result[0]);
        assert_eq!(false, result[1]);
        assert_eq!(true, result[2]);
        assert_eq!(true, result[10]);
        assert_eq!(true, result[20]);
        assert_eq!(true, result[30]);
        assert_eq!(true, result[40]);
        assert_eq!(true, result[50]);
        assert_eq!(false, result[60]);
    }

    #[test]
    fn test_neighbours() {
        let result = neighbours(&(5, 10));
        assert_eq!(
            result,
            vec![
                (6, 11),
                (5, 11),
                (4, 11),
                (6, 10),
                (5, 10),
                (4, 10),
                (6, 9),
                (5, 9),
                (4, 9)
            ]
        );
    }

    #[test]
    fn day20_example1() {
        if let Ok(lines) = read_lines("./input/example20.txt") {
            assert_eq!(35, part1(&lines));
        }
    }

    #[test]
    fn day20_example2() {
        if let Ok(lines) = read_lines("./input/example20.txt") {
            assert_eq!(3351, part2(&lines));
        }
    }
}
