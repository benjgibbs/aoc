use aoc::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./input/day1.txt") {
        let depths: Vec<i32> = lines
            .filter(|br| br.is_ok())
            .map(|br| br.unwrap().parse::<i32>().unwrap())
            .collect();

        let mut last = 0;
        let mut count = 0;
        let mut first = true;

        for current in depths.iter() {
            if first {
                first = false;
            } else {
                if *current > last {
                    count += 1;
                }
            }
            last = *current;
        }
        println!("Part 1: {}", count);

        count = 0;
        for i in 0..depths.len() - 3 {
            if depths[i] < depths[i + 3] {
                count += 1;
            }
        }
        println!("Part 2: {}", count);
    }
}
