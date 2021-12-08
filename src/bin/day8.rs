use std::collections::{BTreeSet, HashMap, HashSet};

use aoc::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./input/day8.txt") {
        let mut count = 0;
        for line in lines.iter() {
            let m = line
                .split(" | ")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .split(" ")
                .filter(|s| HashSet::from([2, 4, 3, 7]).contains(&s.len()))
                .count();
            count += m;
        }
        println!("Part1 (530): {}", count);

        let mut sum = 0;
        for line in lines.iter() {
            let num = decode_line(line);
            sum += num;
        }
        println!("Part2: {}", sum);
    }
}

fn decode_line(line: &str) -> i32 {
    let mut counts: HashMap<usize, HashSet<BTreeSet<char>>> = HashMap::new();
    for w in line.replace(" | ", " ").split(" ") {
        let s: BTreeSet<char> = w.chars().collect();
        counts
            .entry(w.len())
            .or_insert(HashSet::<BTreeSet<char>>::new())
            .insert(s);
    }
    let one = counts.get(&2).unwrap().iter().nth(0).unwrap();
    let four = counts.get(&4).unwrap().iter().nth(0).unwrap();
    let seven = counts.get(&3).unwrap().iter().nth(0).unwrap();
    let eight = counts.get(&7).unwrap().iter().nth(0).unwrap();

    let nine = counts
        .get(&6)
        .unwrap()
        .iter()
        .filter(|s| s.is_superset(four))
        .nth(0)
        .unwrap();

    let three = counts
        .get(&5)
        .unwrap()
        .iter()
        .filter(|s| s.is_superset(one))
        .nth(0)
        .unwrap();

    let five = counts
        .get(&5)
        .unwrap()
        .iter()
        .filter(|s| s.is_subset(nine) && *s != three)
        .nth(0)
        .unwrap();

    

    let two = counts
        .get(&5)
        .unwrap()
        .iter()
        .filter(|s| *s != three && *s != five)
        .nth(0)
        .unwrap();

    let top_right = one.intersection(two).nth(0).unwrap();

    let zero = counts
        .get(&6)
        .unwrap()
        .iter()
        .filter(|s| *s != nine && s.contains(top_right))
        .nth(0)
        .unwrap();

    let six = counts
        .get(&6)
        .unwrap()
        .iter()
        .filter(|s| *s != zero && *s != nine)
        .nth(0)
        .unwrap();

    let lookup = HashMap::from([
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
        (zero, 0),
    ]);
    println!("0: {:?}", zero);
    println!("1: {:?}", one);
    println!("2: {:?}", two);
    println!("3: {:?}", three);
    println!("4: {:?}", four);
    println!("5: {:?}", five);
    println!("6: {:?}", six);
    println!("7: {:?}", seven);
    println!("8: {:?}", eight);
    println!("9: {:?}", nine);
    println!();

    let mut pos = 1;
    let mut num = 0;
    for result in line
        .split("|")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .split(" ")
        .filter(|s| s.trim().len() > 0)
        .collect::<Vec<&str>>()
        .iter()
        .rev()
    {
        let resultid: BTreeSet<char> = result.chars().collect();
        println!("{:?}", resultid);
        num += pos * lookup[&resultid];
        pos *= 10;
    }
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part2() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        assert_eq!(8394, decode_line(input));
    }

    #[test]
    fn question_example() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(5353, decode_line(input));
    }

    #[test]
    fn full_part_2() {
        if let Ok(lines) = read_lines("./input/example8.txt") {
            let mut sum = 0;
            for line in lines.iter() {
                let num = decode_line(line);
                sum += num;
            }
            assert_eq!(61229, sum);
        }
    }
}
