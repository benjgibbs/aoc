use std::collections::HashSet;

use aoc::read_lines;
use regex::Regex;


fn part1(lines: &Vec<String>) -> i32 {

    let pattern =  Regex::new(r"-?\d+").unwrap();
    let mut on = HashSet::new();
    //for line in lines[..lines.len()-2].iter() {
    for line in lines[..20].iter() {
        let numbers : Vec<i32> =  pattern.find_iter(line).map(
            |s| i32::from_str_radix(s.as_str(), 10).unwrap()).collect();

        let turn_on = line.starts_with("on");
        for x in numbers[0]..=numbers[1] {
            for y in numbers[2]..=numbers[3] {
                for z in numbers[4]..=numbers[5] {
                    if turn_on {
                        on.insert((x,y,z));
                    } else {
                        on.remove(&(x,y,z));
                    }
                }
            }
        }
        
    }
    return on.len() as i32;
}


#[derive(Hash, PartialEq, Eq, Debug)]
struct Area {
    x1: i32,
    x2: i32, 
    y1: i32, 
    y2: i32,
    z1: i32,
    z2: i32,
    on: bool
}


fn part2(lines: &Vec<String>) -> usize {

    let pattern =  Regex::new(r"-?\d+").unwrap();
    let mut instructions = Vec::new();
    //for line in lines[..lines.len()-2].iter() {
    for line in lines.iter() {
        let numbers : Vec<i32> =  pattern.find_iter(line).map(
            |s| i32::from_str_radix(s.as_str(), 10).unwrap()).collect();

        let turn_on = line.starts_with("on");
        instructions.push(Area {
                            x1: numbers[0], 
                            x2: numbers[1], 
                            y1: numbers[2], 
                            y2: numbers[3], 
                            z1: numbers[4], 
                            z2: numbers[5], 
                            on: turn_on});      
    }

    for i in 0..instructions.len() - 1 {
        let a1 = &instructions[i];
        for j in i+1..instructions.len() {
            let a2 = &instructions[j];
            if ((a1.x1 > a2.x1 && a1.x1 < a2.x2) || (a1.x2 > a2.x1 && a1.x2 < a2.x2 )) &&
                ((a1.y1 > a2.y1 && a1.y1 < a2.y2) || (a1.y2 > a2.y1 && a1.y2 < a2.y2 )) &&
                ((a1.z1 > a2.z1 && a1.z1 < a2.z2) || (a1.z2 > a2.z1 && a1.z2 < a2.z2 )) {
                    println!("Clash {} {}", i, j);
                }
        }
    }   


    return 0;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day22.txt") {
        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_example1() {
        if let Ok(lines) = read_lines("./input/example22.txt") {
            assert_eq!(590784, part1(&lines));
        }
    }

    #[test]
    fn day22_example2() {
        if let Ok(lines) = read_lines("./input/example22-2.txt") {
            assert_eq!(2758514936282235, part2(&lines));
        }
    }
}