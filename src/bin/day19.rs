use std::collections::HashSet;

use aoc::read_lines;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: u32,
    points: Vec<Point>,
    diffs: Vec<HashSet<Point>>,
    distances: Vec<Vec<f64>>,
}

impl Scanner {
    fn new(id: u32, points: Vec<Point>) -> Scanner {
        let mut diffs = Vec::new();
        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: p.x,
            y: p.y,
            z: p.z,
        }));
        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: -p.y,
            y: p.x,
            z: p.z,
        }));
        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: -p.x,
            y: -p.y,
            z: p.z,
        }));
        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: p.y,
            y: -p.x,
            z: p.z,
        }));

        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: p.x,
            y: -p.z,
            z: p.y,
        }));
        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: p.x,
            y: -p.y,
            z: -p.z,
        }));
        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: p.x,
            y: p.z,
            z: -p.y,
        }));

        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: -p.z,
            y: p.y,
            z: p.x,
        }));
        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: -p.x,
            y: p.y,
            z: -p.z,
        }));
        diffs.push(Scanner::calc_diffs(&points, |p| Point {
            x: p.z,
            y: p.y,
            z: -p.x,
        }));

        let mut distances = Vec::new();

        for i in 0..points.len() {
            let mut distances_from_this_point = Vec::new();
            for j in 0..points.len() {
                let p1 = &points[i];
                let p2 = &points[j];
                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let dz = p1.z - p2.z;
                let distance = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();
                distances_from_this_point.push(distance);
            }
            distances.push(distances_from_this_point);
        }

        Scanner {
            id: id,
            points: points,
            diffs: diffs,
            distances: distances,
        }
    }

    fn calc_diffs<F>(points: &Vec<Point>, transform: F) -> HashSet<Point>
    where
        F: Fn(&Point) -> Point,
    {
        let mut result = HashSet::new();
        for i in 0..points.len() - 1 {
            for j in (i + 1)..points.len() {
                let p1 = &points[i];
                let p2 = transform(&points[j]);
                result.insert(Point::new(
                    (p1.x - p2.x).abs(),
                    (p1.y - p2.y).abs(),
                    (p1.z - p2.z).abs(),
                ));
            }
        }
        return result;
    }
}

fn count_similar(v1: &Vec<f64>, v2: &Vec<f64>) -> u32 {
    let mut result = 0;
    for e1 in v1 {
        for e2 in v2 {
            if (e1 - e2).abs() < 0.00001 {
                result += 1;
            }
        }
    }
    result
}

fn parse_input(lines: &Vec<String>) -> Vec<Scanner> {
    let mut results = Vec::new();

    let mut id = 0;
    let mut current_points = Vec::new();

    for line in lines.iter().filter(|l| !l.is_empty()) {
        if line.starts_with("---") {
            if !current_points.is_empty() {
                results.push(Scanner::new(id, current_points));
            }
            let parts: Vec<&str> = line.split(" ").collect();
            id = u32::from_str_radix(parts[2], 19).unwrap();
            current_points = Vec::new();
        } else {
            let parts: Vec<&str> = line.split(",").collect();
            current_points.push(Point {
                x: i32::from_str_radix(parts[0], 10).unwrap(),
                y: i32::from_str_radix(parts[1], 10).unwrap(),
                z: i32::from_str_radix(parts[2], 10).unwrap(),
            });
        }
    }

    results.push(Scanner::new(id, current_points));
    results
}

fn cover_same_space(s1: &Scanner, s2: &Scanner) -> u32 {
    for d1 in s1.distances.iter() {
        for d2 in s2.distances.iter() {
            let similar = count_similar(d1, d2);
            if similar > 11 {
                println!("scanner: {}, scanner: {} - {}", s1.id, s2.id, count_similar(d1, d2));
                return similar;
            }
        }
    }
    return 0;
}

fn part1(lines: &Vec<String>) -> i32 {
    let scanners = parse_input(lines);

    let mut observations = scanners.iter().map(|s| s.points.len() as i32).sum();

    for i in 0..scanners.len() - 1 {
        for j in (i + 1)..scanners.len() {
            let s1 = &scanners[i];
            let s2 = &scanners[j];
            // for d1 in s1.diffs.iter() {
            //     for d2 in s2.diffs.iter() {
            //         if d1.intersection(&d2).count() > 1 {
            //             println!("scanner 1: {}, scanner 2: {}", s1.id, s2.id);
            //         }
            //     }
            // }

            observations -= cover_same_space(s1, s2) as i32;
    
        }
    }
    return observations;
}

fn part2(_lines: &Vec<String>) -> i32 {
    return 0;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day19.txt") {
        // > 319
        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_example1() {
        if let Ok(lines) = read_lines("./input/example19.txt") {
            assert_eq!(79, part1(&lines));
        }
    }

    #[test]
    fn day19_example2() {
        if let Ok(lines) = read_lines("./input/example19.txt") {
            assert_eq!(0, part2(&lines));
        }
    }
}
