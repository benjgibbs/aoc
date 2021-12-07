use std::collections::HashSet;

use aoc::read_lines;

const GRID_SIZE : i32 = 5;

fn count_adjacent(pos: (i32, i32), tile: &Vec<Vec<bool>>) -> usize {
    let adjacents : Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    return adjacents.iter()
        .map(|a| (pos.0 + a.0, pos.1 + a.1))
        .filter(|p| p.0 >= 0 && p.1 >=0 && p.0 < GRID_SIZE && p.1 < GRID_SIZE)
        .filter(|p| tile[p.1 as usize][p.0 as usize])
        .count();
}

fn get(pos: (i32, i32), tile: u32) -> bool {

}

fn next_tile(tile: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    for y in 0..GRID_SIZE {
        let mut row = Vec::new();
        for x in 0..GRID_SIZE {
            if tile[y as usize][x as usize] {
                row.push(count_adjacent((x,y), tile) == 1);
            } else {
                let adjacent = count_adjacent((x,y), tile);
                row.push( adjacent == 1 || adjacent == 2);
            }
        }
        result.push(row);
    }
    return  result;
}

fn print_tile(tile: &Vec<Vec<bool>>) {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            print!("{}", if tile[y as usize][x as usize] {"#"} else {"."});
        }
        println!();
    }
    println!(); 
}

fn biodiversity(tile: &Vec<Vec<bool>>) -> i64 {
    let mut pow =1;
    let mut result = 0;
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if tile[y as usize][x as usize] {
                result += pow;
            }
            pow *= 2;
        }
    }
    return result;
}


fn solve(input: Vec<String>) ->  i64 {
    let mut tile : Vec<Vec<bool>> = input.iter().map(
            |l| l.chars().map(|c| c == '#').collect()).collect();

    
    let mut seen = HashSet::new();
    print_tile(&tile);
    while !seen.contains(&tile) {
        seen.insert(tile.clone());
        tile = next_tile(&tile);
        print_tile(&tile);
    }
    return biodiversity(&tile);
}


fn main() {
    if let Ok(lines) = read_lines("./input/2019-24.txt") {
        println!("Part 1: {}", solve(lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        if let Ok(lines) = read_lines("./input/2019-24-example.txt") {
            assert_eq!(2129920, solve(lines));
        }
    }

}