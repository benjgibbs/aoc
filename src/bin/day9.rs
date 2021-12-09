use std::collections::{HashSet, VecDeque};

use aoc::read_lines;

fn nbs(p: (usize, usize), map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let search = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let max_y = map.len() as i32;
    let max_x = map.get(p.1 as usize).unwrap().len() as i32;
    let neighbours = search
        .iter()
        .map(|s| ((p.0 as i32) + s.0, (p.1 as i32) + s.1))
        .filter(|s| s.0 >= 0 && s.1 >= 0 && s.0 < max_x && s.1 < max_y)
        .map(|s| (s.0 as usize, s.1 as usize))
        .collect();
    return neighbours;
}

fn neighbours(p: (u32, u32), map: &Vec<Vec<u32>>) -> Vec<u32> {
    let search = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let max_y = map.len() as i32;
    let max_x = map.get(p.1 as usize).unwrap().len() as i32;
    let neighbours: Vec<u32> = search
        .iter()
        .map(|s| ((p.0 as i32) + s.0, (p.1 as i32) + s.1))
        .filter(|s| s.0 >= 0 && s.1 >= 0 && s.0 < max_x && s.1 < max_y)
        .map(|s| *map.get(s.1 as usize).unwrap().get(s.0 as usize).unwrap())
        .collect();
    return neighbours;
}

fn make_map(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let row: Vec<u32> = line.chars().map(|d| d.to_digit(10).unwrap()).collect();
        map.push(row);
    }
    return map;
}

fn low_points(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let max_y = map.len();
    let max_x = map.get(0).unwrap().len();
    for y in 0..max_y {
        for x in 0..max_x {
            let height = map.get(y as usize).unwrap().get(x).unwrap();
            let nbs = neighbours((x as u32, y as u32), &map);
            if nbs.iter().filter(|nh| height >= nh).count() == 0 {
                result.push((x, y));
            }
        }
    }
    return result;
}

fn part1(lines: &Vec<String>) -> u32 {
    let map = make_map(lines);

    return low_points(&map)
        .iter()
        .map(|p| *map.get(p.1).unwrap().get(p.0).unwrap() + 1)
        .sum();
}

fn part2(lines: &Vec<String>) -> usize {
    let map = make_map(lines);
    let low_points = low_points(&map);
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();

    for lp in low_points {
        let mut basin = HashSet::new();
        let mut to_explore = VecDeque::new();
        to_explore.push_back(lp);
        basin.insert(lp);
        while !to_explore.is_empty() {
            let next = to_explore.pop_back().unwrap();
            let unseen : Vec<(usize,usize)> = nbs(next, &map)
                .iter()
                .filter(|n| !basin.contains(n))
                .filter(|n| *map.get(n.1).unwrap().get(n.0).unwrap() != 9)
                .map(|n|(n.0, n.1))
                .collect();

            for b in unseen.iter() {
                to_explore.push_back(*b);
                basin.insert(*b);
            }
        }
        basins.push(basin);
    }

    basins.sort_by(|a,b| b.len().cmp(&a.len()));

    return basins.get(0).unwrap().len() * 
        basins.get(1).unwrap().len() * 
        basins.get(2).unwrap().len();
}

fn main() {
    if let Ok(lines) = read_lines("./input/day9.txt") {
        println!("Part1: {}", part1(&lines));
        println!("Part2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1_example() {
        if let Ok(lines) = read_lines("./input/example9.txt") {
            assert_eq!(15, part1(&lines));
        }
    }

    #[test]
    fn day9_part2_example() {
        if let Ok(lines) = read_lines("./input/example9.txt") {
            assert_eq!(1134, part2(&lines));
        }
    }
}
