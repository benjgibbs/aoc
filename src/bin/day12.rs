use std::collections::{HashMap, HashSet};

use aoc::read_lines;

fn is_small_cave(cave: &str) -> bool {
    cave == cave.to_lowercase() 
}

fn search(from: &str, seen: &HashSet<String>, al: &HashMap<String, Vec<String>>) -> i32 {
    let mut count = 0;
    for n in al[from].iter().filter(|c| !seen.contains(*c)) {
        if n == "start" {
            count += 1;
        } else if is_small_cave(n) {
            let mut seen_copy = seen.clone();
            seen_copy.insert(n.to_string());
            count += search(n, &seen_copy, al);
        } else {
            count += search(n, seen, al);
        }
    }
    return count;
}

fn search2(from: &str, small_cave: &str, scc: i32,  seen: &HashSet<String>, al: &HashMap<String, Vec<String>>) -> HashSet<Vec<String>> {
    let mut paths = HashSet::new();
    for n in al[from].iter().filter(|c| !seen.contains(*c)) {
        if n == "start" {
            paths.insert(Vec::from(["start".to_string(), from.to_string()]));
        } else if is_small_cave(n) {
            if n == small_cave {
                if scc == 0 {
                    for p in search2(n, small_cave, 1, seen, al) {
                        let mut p2 = p.clone();
                        p2.push(from.to_string());
                        paths.insert(p2);
                    }
                } else {
                    let mut seen_copy = seen.clone();
                    seen_copy.insert(n.to_string());
                    for p in search2(n, small_cave, 1, &seen_copy, al) {
                        let mut p2 = p.clone();
                        p2.push(from.to_string());
                        paths.insert(p2);
                    }
                }
            } else {
                let mut seen_copy = seen.clone();
                seen_copy.insert(n.to_string());
                for p in search2(n, small_cave, scc, &seen_copy, al) {
                    let mut p2 = p.clone();
                    p2.push(from.to_string());
                    paths.insert(p2);
                }
            }
        } else {
            for p in search2(n, small_cave, scc, seen, al) {
                let mut p2 = p.clone();
                p2.push(from.to_string());
                paths.insert(p2);
            }
        }
    }
    return paths;
}

fn get_adjacency_list(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut al : HashMap<String, Vec<String>> = HashMap::new();

    for l in lines {
        let edge : Vec<String> = l.split("-").map(|s| s.to_string()).collect();
        al.entry(edge[0].clone()).or_insert(Vec::new()).push(edge[1].clone());
        al.entry(edge[1].clone()).or_insert(Vec::new()).push(edge[0].clone());
    }
    return al;
}

fn part1(lines: &Vec<String>) -> i32 {
    let al = get_adjacency_list(lines);    

    let mut seen: HashSet<String> = HashSet::new();
    seen.insert("end".to_string());
    return search("end", &seen, &al);
}

fn part2(lines: &Vec<String>) -> i32 {
    let al = get_adjacency_list(lines);
    let mut small_caves : HashSet<&String> = al.keys().filter(|k| is_small_cave(k)).collect();
    small_caves.remove(&"start".to_string());
    small_caves.remove(&"end".to_string());
    println!("{:?}", small_caves);

    let mut paths : HashSet<Vec<String>> = HashSet::new();
    for sc in small_caves {
        let mut seen: HashSet<String> = HashSet::new();
        seen.insert("end".to_string());
        paths.extend(search2("end", sc, 0, &seen, &al));
    }

    return paths.len() as i32;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day12.txt") {
        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_example1() {
        if let Ok(lines) = read_lines("./input/example12.txt") {
            assert_eq!(19, part1(&lines));
        }
    }

    #[test]
    fn day12_example2() {
        if let Ok(lines) = read_lines("./input/example12.txt") {
            assert_eq!(103, part2(&lines));
        }
    }

    #[test]
    fn is_small_cave_test() {
        assert!(is_small_cave("ab"));
        assert!(!is_small_cave("AB"));
    }
}