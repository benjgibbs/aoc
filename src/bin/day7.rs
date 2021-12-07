
use aoc::{get_nums, read_lines};

fn main() {
    if let Ok(lines) = read_lines("./input/day7.txt") {
        let line = lines.get(0).unwrap();
        let nums = get_nums(line);
        let mut min_cost = i32::MAX;
        let mut min_pos = 0;
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        for p in min..=max {
            let mut cost = 0;
            for i in nums.iter() {
                cost += (p - i).abs();
            }
            if cost < min_cost {
                min_pos = p;
                min_cost = cost;
            }
        }
        println!("Part1 (341558): {}, {}", min_pos, min_cost);
        
        let mut min_cost = i32::MAX;
        let mut min_pos = 0;
        for p in min..=max {
            let mut cost = 0;
            for i in nums.iter() {
                cost += calc_cost((p - i).abs());
            }
            if cost < min_cost {
                min_pos = p;
                min_cost = cost;
                
            }
        }
        println!("Part2 (93214037): {}, {}", min_pos, min_cost);
    }
}

fn calc_cost(m : i32) -> i32 {
    return m*(m+1)/2;
}
