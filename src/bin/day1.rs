use aoc::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("./input/day1.txt").unwrap();
    let depths: Vec<i32> = lines
        .into_iter()
        .map(|br| br.parse::<i32>().unwrap())
        .collect();

    let mut last = 0;
    let mut count = 0;
    let mut done_first = false;

    for current in depths.iter() {
        if done_first {
            if *current > last {
                count += 1;
            }
        } else {
            done_first = true;
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
