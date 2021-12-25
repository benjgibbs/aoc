use std::collections::HashSet;

use aoc::read_lines;

struct Ocean {
    south_herd: HashSet<(usize, usize)>,
    east_herd: HashSet<(usize, usize)>,
    max_x: usize, 
    max_y: usize
}

impl Ocean {
    fn read_input(lines: &Vec<String>) ->Ocean {
        let mut south_herd = HashSet::new();
        let mut east_herd = HashSet::new();

        let mut max_x = usize::MIN;
        for y in 0..lines.len() {
            let line: Vec<char> = lines[y].chars().collect();
            max_x = max_x.max(line.len());
            for x in 0..line.len() {
                match line[x] {
                    '>' => east_herd.insert((x, y)),
                    'v' => south_herd.insert((x, y)),
                    _ => false
                };
            }
        }
    
        Ocean { south_herd: south_herd, east_herd: east_herd, max_x: max_x, max_y: lines.len() }
    }

    fn move_east(&mut self) -> bool {
        let mut result = HashSet::<(usize, usize)>::new();
        let mut has_moved = false;

        for (x, y) in self.east_herd.iter() {
            let x2 = (x + 1) % self.max_x;
            if self.contains(&(x2, *y)) {
                result.insert((*x,*y));
            } else {
                has_moved = true;
                result.insert((x2,*y));
            }
        }
        self.east_herd = result;
        has_moved
    }

    fn move_south(&mut self) -> bool {
        let mut result = HashSet::<(usize, usize)>::new();
        let mut has_moved = false;

        for (x, y) in self.south_herd.iter() {
            let y2 = (y + 1) % self.max_y;
            if self.contains(&(*x, y2)) {
                result.insert((*x,*y));
            } else {
                has_moved = true;
                result.insert((*x,y2));
            }
        }
        self.south_herd = result;
        has_moved
    }

    fn run(&mut self) -> i32 {
        let mut step_count = 0;
        loop {
            let moved_east = self.move_east();
            let moved_south = self.move_south();
            step_count += 1;
            if !(moved_east || moved_south) {
                break;
            }
        }
        return step_count;
    }
    fn contains(&self, pos: &(usize, usize)) -> bool {
        self.east_herd.contains(pos) || self.south_herd.contains(pos)
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut ocean = Ocean::read_input(lines);
    ocean.run()
}

fn main() {
    if let Ok(lines) = read_lines("./input/day25.txt") {
        println!("Part 1: {}", part1(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_example1() {
        if let Ok(lines) = read_lines("./input/example25.txt") {
            assert_eq!(58, part1(&lines));
        }
    }

    #[test]
    fn day25_problem() {
        if let Ok(lines) = read_lines("./input/day25.txt") {
            assert_eq!(523, part1(&lines));
        }
    }
}
