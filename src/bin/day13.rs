use std::collections::HashSet;

use aoc::{read_lines, get_nums};

fn get_input(lines: &Vec<String>) -> (HashSet<(i32,i32)>, Vec<(char, i32)>) {
    let mut points = HashSet::new();
    let mut folds = Vec::new();

    for line in lines {
        let digits = get_nums(line);

        if digits.len() == 2 {
            points.insert((digits[0], digits[1]));
            
        } else if digits.len() == 1{
            folds.push((line.as_bytes()[11] as char, digits[0]));
        }
    }
    (points, folds)
}

fn do_fold(fold: &(char, i32), points: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_points = HashSet::new();
    if fold.0 == 'x' {
        let fx = fold.1;
        for p in points {
            if p.0 > fx {
                let nx = fx - (p.0-fx);
                new_points.insert((nx, p.1));
            } else {
                new_points.insert(*p);
            }
        }
    } else {
        let fy = fold.1;
        for p in points {
            if p.1 > fy {
                let ny = fy - (p.1-fy);
                new_points.insert((p.0, ny));
            } else {
                new_points.insert(*p);
            }
        }
    }
    new_points
}

fn part1(lines: &Vec<String>) -> usize {

    let (points, folds) = get_input(lines);

    let fold = folds.get(0).unwrap();
    let new_points = do_fold(fold, &points);
    return new_points.len();
}

fn part2(lines: &Vec<String>) -> i32 {
    let (points, folds) = get_input(lines);
    let mut points = points.clone();
    for fold in folds {
        points = do_fold(&fold, &points);
    }

    for y in 0..10 {
        for x in 0..100 {
            if points.contains(&(x,y)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    return 0;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day13.txt") {
        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_example1() {
        if let Ok(lines) = read_lines("./input/example13.txt") {
            assert_eq!(17, part1(&lines));
        }
    }
}