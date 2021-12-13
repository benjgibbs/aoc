use std::collections::{HashMap, BTreeSet};

use aoc::read_lines;

fn is_small_cave(cave: &str) -> bool {
    cave == cave.to_lowercase() 
}

fn search(from: &str, seen: &BTreeSet<String>, al: &HashMap<String, Vec<String>>) -> i32 {
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

fn append(from: &str, part: &BTreeSet<Vec<String>>, result: &mut BTreeSet<Vec<String>>) {
    for p in part {
        let mut p2 = p.clone();
        p2.push(from.to_string());
        result.insert(p2);
    }
}

fn search2(from: &str, small_cave: &str, scc: i32,  seen: &BTreeSet<String>, al: &HashMap<String, Vec<String>>) -> BTreeSet<Vec<String>> {

    let mut paths = BTreeSet::new();
    for n in al[from].iter().filter(|c| !seen.contains(*c)) {
        if n == "start" {
            paths.insert(Vec::from(["start".to_string(), from.to_string()]));
        } else if is_small_cave(n) {
            if n == small_cave {
                if scc == 0 {
                    append(from,&search2(n, small_cave, 1, seen, al), &mut paths);
                } else {
                    let mut seen_copy = seen.clone();
                    seen_copy.insert(n.to_string());
                    append(from,&search2(n, small_cave, 1, &seen_copy, al), &mut paths);
                }
            } else {
                let mut seen_copy = seen.clone();
                seen_copy.insert(n.to_string());
                append(from,&search2(n, small_cave, scc, &seen_copy, al), &mut paths);
            }
        } else {
            append(from,&search2( n, small_cave, scc, seen, al), &mut paths)
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

    let mut seen: BTreeSet<String> = BTreeSet::new();
    seen.insert("end".to_string());
    return search("end", &mut seen, &al);
}

fn part2(lines: &Vec<String>) -> i32 {
    let al = get_adjacency_list(lines);
    let small_caves : BTreeSet<&String> = al.keys().filter(|k| is_small_cave(k) && **k != "start".to_string() && **k != "end".to_string()).collect();

    let mut paths : BTreeSet<Vec<String>> = BTreeSet::new();
    for sc in small_caves {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        seen.insert("end".to_string());
        paths.extend(search2( "end", sc, 0, &seen, &al));
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
    fn day12_part1() {
        if let Ok(lines) = read_lines("./input/day12.txt") {
            assert_eq!(3485, part1(&lines));
        }
    }

    #[test]
    fn day12_part2() {
        if let Ok(lines) = read_lines("./input/day12.txt") {
            assert_eq!(85062, part2(&lines));
        }
    }

    #[test]
    fn is_small_cave_test() {
        assert!(is_small_cave("ab"));
        assert!(!is_small_cave("AB"));
    }
}