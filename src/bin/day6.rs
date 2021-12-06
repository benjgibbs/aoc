use aoc::{get_nums, read_lines};
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("./input/day6.txt") {
        let mut fish = get_nums(lines.get(0).unwrap());
        println!("{:?}", fish);
        let iters = 80;
        for _i in 0..iters {
            // if _i % 5 == 0 {
            //     println!("{}: {}", _i, fish.len());
            // }
            let mut new_fish : Vec<i32> = Vec::new();
            for f in 0..fish.len() {
                if fish[f] == 0 {
                    new_fish.push(8);
                    fish[f] = 6;
                } else {
                    fish[f] -= 1;
                }
            }
            fish.append(&mut new_fish);
        }
        println!("Part1: {}", fish.len());

        let mut fish_ages = HashMap::new();
        let fish = get_nums(lines.get(0).unwrap());
        for f in fish {
            *fish_ages.entry(f).or_insert(0i64) += 1;
        }
        let iters = 256;
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
        let count : i64 = fish_ages.iter().map(|kvp| *kvp.1).sum();
        println!("{:?}", fish_ages);
        println!("Part2: {}", count);
    }
}