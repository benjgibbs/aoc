use aoc::{get_nums, read_lines};
use std::collections::HashMap;

fn iterate(fish: &Vec<i32>, iters: i32) -> i64 {
    let mut fish_ages = HashMap::new();    
    for f in fish {
        *fish_ages.entry(*f).or_insert(0i64) += 1;
    }
    for _i in 0.. iters {
        let mut new_ages = HashMap::new();
        for a in 0..9 {
            let count = *fish_ages.get(&a).or(Some(&0)).unwrap();
            if a == 0 {
                *new_ages.entry(6).or_insert(0) += count;
                *new_ages.entry(8).or_insert(0) += count;
            } else {
                *new_ages.entry(a-1).or_insert(0) += count;
            }
        }
        fish_ages = new_ages;
    }
    return fish_ages.iter().map(|kvp| *kvp.1).sum();
}

fn main() {
    if let Ok(lines) = read_lines("./input/day6.txt") {
        let fish = get_nums(lines.get(0).unwrap());
        
        println!("Part1 (345387): {}", iterate(&fish, 80));
        println!("Part2 (1574445493136): {}", iterate(&fish, 256));
    }
}