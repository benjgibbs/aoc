use aoc::read_lines;


fn part1(lines: &Vec<String>) -> i32 {

    return 0;
}

fn part2(lines: &Vec<String>) -> i32 {

    return 0;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day10.txt") {
        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_example1() {
        if let Ok(lines) = read_lines("./input/example10.txt") {
            assert_eq!(0, part1(&lines));
        }
    }

    #[test]
    fn day10_example2() {
        if let Ok(lines) = read_lines("./input/example10.txt") {
            assert_eq!(0, part2(&lines));
        }
    }
}