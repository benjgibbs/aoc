use std::{collections::HashMap, cmp::{max, min}};

use aoc::{get_nums, read_lines};

fn main() {
    if let Ok(lines) = read_lines("./input/day5.txt") {
        let mut vents : HashMap<(i32,i32), i32> = HashMap::new();
        for line in lines.iter() {
            let nums = get_nums(&line);
            let start = (nums.get(0).unwrap(), nums.get(1).unwrap());
            let end = (nums.get(2).unwrap(), nums.get(3).unwrap());
            if start.0 == end.0 || start.1 == end.1 {
                for x in min(*start.0,*end.0)..=max(*start.0,*end.0) {
                    for y in min(*start.1,*end.1)..=max(*start.1,*end.1) {
                        let e  = vents.entry((x,y)).or_insert(0);
                        *e += 1;
                    }
                }
            }
        }

        let more_than_one = vents.iter()
                .filter(|kvp| *kvp.1 > 1)
                .count();
        
        println!("Part1 (6841): {}", more_than_one);

        for line in lines.iter() {
            let nums = get_nums(&line);
            let start = (nums.get(0).unwrap(), nums.get(1).unwrap());
            let end = (nums.get(2).unwrap(), nums.get(3).unwrap());
            if !(start.0 == end.0 || start.1 == end.1) {
                let x_inc = if start.0 > end.0 {-1} else {1};
                let y_inc = if start.1 > end.1 {-1} else {1};
                let mut pos = (*start.0, *start.1);
                while pos != (*end.0 + x_inc, *end.1 + y_inc) {
                    let e  = vents.entry(pos).or_insert(0);
                    *e += 1;
                    pos = (pos.0 + x_inc, pos.1 + y_inc);
                };
            }
        }

        let more_than_one = vents.iter()
                .filter(|kvp| *kvp.1 > 1)
                .count();

        println!("Part2 (19258): {}", more_than_one);
    }
}