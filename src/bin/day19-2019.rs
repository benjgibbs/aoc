use std::collections::{HashSet, VecDeque};

use aoc::read_lines;

const GRID_SIZE: u32 = 5;

fn count_adjacent(pos: (u32, u32), tile: u32) -> usize {
    let adjacents: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    return adjacents
        .iter()
        .map(|a| ((pos.0 as i32) + a.0, (pos.1 as i32) + a.1))
        .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < GRID_SIZE as i32 && p.1 < GRID_SIZE as i32)
        .filter(|p| get((p.0 as u32, p.1 as u32), tile))
        .count();
}

fn count_adjacent2(pos: (u32, u32), tile: usize, universe: &VecDeque<u32>) -> usize {
    let mut adjacent = count_adjacent(pos, *universe.get(tile).unwrap());
    if tile > 0 {
        let tile = *universe.get(tile - 1).unwrap();
        if pos.0 == 0 {
            if get((1, 2), tile) {
                adjacent += 1;
            }
        } else if pos.0 == 4 {
            if get((3, 2), tile) {
                adjacent += 1;
            }
        }

        if pos.1 == 0 {
            if get((2, 1), tile) {
                adjacent += 1;
            }
        } else if pos.1 == 4 {
            if get((2, 3), tile) {
                adjacent += 1;
            }
        }
    }
    if tile < universe.len() - 1 {
        let tile = *universe.get(tile + 1).unwrap();
        if pos == (2, 1) {
            for x in 0..GRID_SIZE {
                if get((x, 0), tile) {
                    adjacent += 1;
                }
            }
        } else if pos == (1, 2) {
            for y in 0..GRID_SIZE {
                if get((0, y), tile) {
                    adjacent += 1;
                }
            }
        } else if pos == (3, 2) {
            for y in 0..GRID_SIZE {
                if get((4, y), tile) {
                    adjacent += 1;
                }
            }
        } else if pos == (2, 3) {
            for x in 0..GRID_SIZE {
                if get((x, 4), tile) {
                    adjacent += 1;
                }
            }
        }
    }
    return adjacent;
}

fn get(pos: (u32, u32), bitset: u32) -> bool {
    let bit = 5 * pos.1 + pos.0;
    return bitset & 2u32.pow(bit) > 0;
}

fn set(pos: (u32, u32), v: bool, bitset: u32) -> u32 {
    let bit = GRID_SIZE * pos.1 + pos.0;
    if v {
        return bitset ^ 2u32.pow(bit);
    } else {
        return bitset & !2u32.pow(bit);
    }
}

fn next_tile(tile: u32) -> u32 {
    let mut result = 0u32;
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if get((x as u32, y as u32), tile) {
                if count_adjacent((x, y), tile) == 1 {
                    result = set((x, y), true, result);
                }
            } else {
                let adjacent = count_adjacent((x, y), tile);
                if adjacent == 1 || adjacent == 2 {
                    result = set((x, y), true, result);
                }
            }
        }
    }
    return result;
}

fn print_tile(tile: u32) {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            print!("{}", if get((x, y), tile) { "#" } else { "." });
        }
        println!();
    }
    println!();
}

fn biodiversity(tile: u32) -> i64 {
    let mut pow = 1;
    let mut result = 0;
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if get((x, y), tile) {
                result += pow;
            }
            pow *= 2;
        }
    }
    return result;
}

fn init_tile(input: &Vec<String>) -> u32 {
    let mut tile: u32 = 0;
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if input[y as usize].chars().nth(x as usize) == Some('#') {
                tile = set((x, y), true, tile);
            }
        }
    }
    return tile;
}

fn part1(input: &Vec<String>) -> i64 {
    let mut tile = init_tile(input);

    let mut seen = HashSet::new();
    //print_tile(tile);
    while !seen.contains(&tile) {
        seen.insert(tile);
        tile = next_tile(tile);
        //print_tile(tile);
    }
    return biodiversity(tile);
}

fn part2(input: &Vec<String>, iterations: i32) -> i32 {
    let tile = init_tile(input);

    let mut universe = VecDeque::new();
    universe.push_back(0);
    universe.push_back(tile);
    universe.push_back(0);

    for _t in 0..iterations {
        let mut new_universe = VecDeque::new();
        for i in 0..universe.len() {
            let mut new_tile = 0;
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let adjacent = count_adjacent2((x, y), i, &universe);
                    if get((x, y), *universe.get(i).unwrap()) {
                        if adjacent == 1 {
                            new_tile = set((x, y), true, new_tile);
                        }
                    } else {
                        if adjacent == 1 || adjacent == 2 {
                            new_tile = set((x, y), true, new_tile);
                        }
                    }
                }
            }
            new_universe.push_back(new_tile);
        }
        if *new_universe.front().unwrap() > 0 {
            new_universe.push_front(0);
        }
        if *new_universe.back().unwrap() > 0 {
            new_universe.push_back(0);
        }
        universe = new_universe;
    }

    let mut count = 0;
    for grid in universe.iter() {
        for i in 0..GRID_SIZE * GRID_SIZE {
            if get((i % GRID_SIZE, i / GRID_SIZE), *grid) {
                count += 1;
            }
        }
    }

    // for i in universe.iter() {
    //     print_tile(*i);
    //     println!();
    // }

    return count;
}

fn main() {
    if let Ok(lines) = read_lines("./input/2019-24.txt") {
        println!("Part 1 (32526865): {}", part1(&lines));
        println!("Part 2 (2009): {}", part2(&lines, 200));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        if let Ok(lines) = read_lines("./input/2019-24-example.txt") {
            assert_eq!(2129920, part1(&lines));
        }
    }

    #[test]
    fn example_part2() {
        if let Ok(lines) = read_lines("./input/2019-24-example.txt") {
            assert_eq!(99, part2(&lines, 10));
        }
    }

    #[test]
    fn test_indexing() {
        for i in 0..5 {
            for j in 0..5 {
                let mut bitset = 0u32;
                bitset = set((j, i), true, bitset);
                assert!(get((j, i), bitset));
                bitset = set((j, i), false, bitset);
                assert_eq!(false, get((j, i), bitset));
                assert_eq!(0, bitset);
            }
        }
    }
}
