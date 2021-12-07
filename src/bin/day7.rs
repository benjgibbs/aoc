use aoc::{get_nums, read_lines};

fn main() {
    if let Ok(lines) = read_lines("./input/day7.txt") {
        let line = lines.get(0).unwrap();
        let nums = get_nums(line);

        let part1 = calc(&nums, |x|x);
        println!("Part1 (341558): {}",  part1);

        let part2 = calc(&nums, |x|x*(x+1)/2);
        println!("Part2 (93214037): {}",  part2);
    }
}

fn calc<P>(crabs: &Vec<i32>, cost_func: P) -> i32
where
    P: Fn(i32) -> i32,
{
    let mut min_cost = i32::MAX;
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    for p in min..=max {
        let mut cost = 0;
        for i in crabs.iter() {
            cost += cost_func((p - i).abs());
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }
    return min_cost;
}
