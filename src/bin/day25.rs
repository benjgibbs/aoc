use std::collections::HashMap;

use aoc::read_lines;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Herd {
    East,
    South,
}

fn read_input(lines: &Vec<String>) -> ((usize, usize), HashMap<(usize, usize), Herd>) {
    let mut result = HashMap::new();
    let mut max_x = usize::MIN;
    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();
        max_x = max_x.max(line.len());
        for x in 0..line.len() {
            match line[x] {
                '>' => result.insert((x, y), Herd::East),
                'v' => result.insert((x, y), Herd::South),
                _ => None,
            };
        }
    }

    return ((max_x, lines.len()), result);
}

fn move_east(ocean: &HashMap<(usize, usize), Herd>, max_x: usize) -> HashMap<(usize, usize), Herd> {
    let mut result = HashMap::<(usize, usize), Herd>::new();
    for ((x, y), h) in ocean.iter() {
        result.insert(
            match *h {
                Herd::South => (*x, *y),
                Herd::East => {
                    let x2 = (x + 1) % max_x;
                    if ocean.contains_key(&(x2, *y)) {
                        (*x, *y)
                    } else {
                        (x2, *y)
                    }
                }
            },
            *h,
        );
    }
    result
}

fn move_south(ocean: &HashMap<(usize, usize), Herd>, max_y: usize) -> HashMap<(usize, usize), Herd> {
    let mut result = HashMap::<(usize, usize), Herd>::new();
    for ((x, y), h) in ocean.iter() {
        result.insert(
            match *h {
                Herd::East => (*x, *y),
                Herd::South => {
                    let y2 = (y + 1) % max_y;
                    if ocean.contains_key(&(*x, y2)) {
                        (*x, *y)
                    } else {
                        (*x, y2)
                    }
                }
            },
            *h,
        );
    }
    result
}

fn part1(lines: &Vec<String>) -> i32 {
    let ((max_x, max_y), mut ocean) = read_input(lines);

    let mut step_count = 0;
    loop {
        let start = ocean.clone();
        let move1 = move_east(&start, max_x);
        let end = move_south(&move1, max_y);
        step_count += 1;
        if start == end {
            break;
        }
        ocean = end;
    }
    return step_count;
}


fn main() {
    if let Ok(lines) = read_lines("./input/day25.txt") {
        println!("Part 1: {}", part1(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_example1() {
        if let Ok(lines) = read_lines("./input/example25.txt") {
            assert_eq!(58, part1(&lines));
        }
    }

    #[test]
    fn day25_example2() {
        if let Ok(lines) = read_lines("./input/example25.txt") {
            assert_eq!(0, part2(&lines));
        }
    }
}
