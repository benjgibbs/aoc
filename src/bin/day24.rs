use std::{fmt::Display};

use aoc::read_lines;

struct ALU {
    reg: [isize; 4],
    program: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
enum Register {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl Display for ALU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "w={:<10} x={:<10} y={:<10} z={:<10}",
            self.reg[0], self.reg[1], self.reg[2], self.reg[3]
        )
    }
}

impl ALU {
    fn input(&mut self, a: &Register, v: isize) {
        //println!("alu z: {}", self.reg[3]);
        self.reg[*a as usize] = v;
    }

    fn add(&mut self, a: &Register, b: &str) {
        self.reg[*a as usize] = self.reg[*a as usize] + self.deref(b);
    }

    fn mul(&mut self, a: &Register, b: &str) {
        self.reg[*a as usize] = self.reg[*a as usize] * self.deref(b);
    }

    fn div(&mut self, a: &Register, b: &str) {
        self.reg[*a as usize] = self.reg[*a as usize] / self.deref(b);
    }

    fn modulo(&mut self, a: &Register, b: &str) {
        self.reg[*a as usize] = self.reg[*a as usize] % self.deref(b);
    }

    fn eql(&mut self, a: &Register, b: &str) {
        self.reg[*a as usize] = if self.reg[*a as usize] == self.deref(b) {
            1
        } else {
            0
        }
    }

    fn str_to_reg(&self, a: &str) -> Register {
        match a {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => panic!("Failed to parse: {}", a),
        }
    }

    fn deref(&self, b: &str) -> isize {
        if let Ok(b_val) = isize::from_str_radix(b, 10) {
            return b_val;
        } else {
            self.reg[self.str_to_reg(b) as usize]
        }
    }

    fn new<T>(lines: &Vec<T>) -> ALU
    where
        T: ToString,
    {
        ALU {
            reg: [0, 0, 0, 0],
            program: lines.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn run(&mut self, input: isize) -> isize {
        let mut input_remainder = input;

        for l in self.program.clone().iter() {
            let parts: Vec<&str> = l.split(" ").collect();
            match parts[0] {
                "inp" => {
                    let (digit, pow) = most_signficant(input_remainder);
                    input_remainder -= digit * 10isize.pow(pow as u32 - 1);

                    //println!("Inputting: {}", digit);

                    self.input(&self.str_to_reg(parts[1]), digit);
                }
                "add" => self.add(&self.str_to_reg(parts[1]), parts[2]),
                "mul" => self.mul(&self.str_to_reg(parts[1]), parts[2]),
                "div" => self.div(&self.str_to_reg(parts[1]), parts[2]),
                "mod" => self.modulo(&self.str_to_reg(parts[1]), parts[2]),
                "eql" => self.eql(&self.str_to_reg(parts[1]), parts[2]),
                _ => panic!("Bad instruction: {}", parts[0]),
            }
            //println!("{:10} => {}", l, alu);
        }
        return self.reg[Register::Z as usize];
    }
}

fn most_signficant(mut n: isize) -> (isize, isize) {
    let mut pow = 1;
    while n > 9 {
        n /= 10;
        pow += 1
    }
    (n, pow)
}

fn part1(lines: &Vec<&str>) -> i32 {

    let mut search = Search::new();
    let result = search.step(0, 0);
    println!("Part1: {:?}", result);
    let mut alu = ALU::new(lines);
    if  alu.run(result.unwrap() as isize) == 0 {
        println!("Part1: {:?}", result);
    } else {
        println!("Not okay");
    }
    return result.unwrap() as i32;
}

struct Search {
    v1s: [isize; 14],
    v2s: [isize; 14],
    v3s: [isize; 14],
}


impl Search {
    fn new() -> Search {
        Search {
            v1s: [1,  1,  1,   26, 1,   26, 26, 1,  1,  1,   26, 26, 26, 26],
            v2s: [14, 15, 13, -10, 14, -3, -14, 12, 14, 12, -6, -6, -2, -9],
            v3s: [8,  11, 2,   11, 1,   5,  10, 6,  1,  11,  9,  14, 11, 2],
        }
    }

    fn step(&mut self, v: usize, initial_z: isize) -> Option<usize> {

        for w in 1..=9 {
            let out_z = self.run_for_single_input(w as isize, initial_z, v);
            if out_z > 1000000 { //Prune
                continue;
            }
            if v == 13 {
                if out_z == 0 {
                    println!("v={}, w={}, initial_z: {}, out_z: {}", v, w, initial_z, out_z);
                    return Some(w);
                }
            } else {
                if let Some(n2) = self.step(v+1, out_z) {
                    println!("v={}, w={}, initial_z: {}, out_z: {}", v, w, initial_z, out_z);
                    return Some(n2 + 10usize.pow((13-v) as u32) * w);
                }
            }
        }

        //println!("size: {}, vals: {:?}", digit_to_highest_z.len(), digit_to_highest_z);
        None
    }

    fn run_for_single_input(&self, w: isize, z: isize, v: usize) -> isize {
        let mut x = z;
        x = x % 26;
    
        let mut z2 = z / self.v1s[v];
        x += self.v2s[v];
    
        if x != w {
            x = 1;
        } else {
            x = 0;
        }
    
        let mut y = 25isize;
        y *= x;
        y += 1;
        z2 *= y;
    
        y = w + self.v3s[v];
        y *= x;
        return z2 + y;
    }
}

fn part2(_lines: &Vec<String>) -> i32 {
    return 0;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day24.txt") {
        println!(
            "Part 1: {}",
            part1(&lines.iter().map(|s| s.as_str()).collect())
        );
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[test]
    fn day24_search_test() {
        // 97995759989998 too low
        // 89494956791999 too low
        let mut search = Search::new();
        let result = search.step(0, 0);
        println!("Part1: {:?}", result);
    }

    #[test]
    fn day24_check_single() {
        let search = Search::new();
        let r = search.run_for_single_input(8, 17, 13);
        println!("Out: {}", r);
    }

    #[test]
    fn day24_compare_partial_to_full() {
        let input = 89494956791999isize;

        if let Ok(lines) = read_lines("./input/day24.txt") {
            let mut alu = ALU::new(&lines.iter().map(|s| s.as_str()).collect());
            let alu_result = alu.run(input);

            let search = Search::new();
            let mut search_result = 0;
            for i in 0..=13 {
                let w = (input / 10isize.pow(13-i)) % 10;
                search_result = search.run_for_single_input(w, search_result, i as usize);    
                println!("z={}, w={}", search_result, w);
            }
            println!("alu result: {}", alu_result);
            println!("search result: {}", search_result);
            assert_eq!(alu_result, search_result);
        }
    }

    #[test]
    fn day24_check_number() {
        if let Ok(lines) = read_lines("./input/day24.txt") {
            let mut alu = ALU::new(&lines.iter().map(|s| s.as_str()).collect());
            let result = alu.run(15161611131942);
            println!("Result: {}", result);
        }
        

    }

    #[test]
    fn day24_most_sig_test() {
        let search = Search::new();
        for z in (-50)..50 {
            for w in 1..=9 {
                let result = search.run_for_single_input(w, z, 0);
                println!("z={}, i={}, result: {:?}", z, w, result);
            }
        }
    }

    #[test]
    fn day24_partial() {
        if let Ok(lines) = read_lines("./input/partial24.txt") {
            for input in 11..=99 {
                let mut alu = ALU::new(&lines);
                let result = alu.run(input as isize);
                if result == 0 {
                    println!("\n** {}", input);
                }
            }
        }
    }

    fn run_for(input: isize, z: isize, v1: i32, v2: i32, v3: i32) -> isize {
        let i1 = format!("div z {}", v1);
        let i2 = &format!("add x {}", v2);
        let i3 = &format!("add y {}", v3);
        let lines = Vec::from([
            "inp w", "mul x 0", "add x z", "mod x 26", &i1, &i2, "eql x w", "eql x 0", "mul y 0",
            "add y 25", "mul y x", "add y 1", "mul z y", "mul y 0", "add y w", &i3, "mul y x",
            "add z y",
        ]);
    
        let mut alu = ALU::new(&lines);
        alu.reg[Register::Z as usize] = z;
        return alu.run(input);
    }

    #[test]
    fn check_run_for_single() {
        let v1s = [1, 1, 1, 26, 1, 26, 26, 1, 1, 1, 26, 26, 26, 26];
        let v2s = [14, 15, 13, -10, 14, -3, -14, 12, 14, 12, -6, -6, -2, -9];
        let v3s = [8, 11, 2, 11, 1, 5, 10, 6, 1, 11, 9, 14, 11, 2];

        let search = Search::new();
        for w in 1..=9 {
            for v in 0..v1s.len() {
                for z in 0..100 {
                    let r1 = run_for(w, z, v1s[v], v2s[v], v3s[v]);
                    let r2 = search.run_for_single_input(w,z,v);
                    //println!("{} {} {} => {} {}", w, v, z, r1, r2);
                    assert_eq!(r1, r2);
                }
            }
        }
    }

    #[test]
    fn day24_() {
        let v1s = [1, 1, 1, 26, 1, 26, 26, 1, 1, 1, 26, 26, 26, 26];
        let v2s = [14, 15, 13, -10, 14, -3, -14, 12, 14, 12, -6, -6, -2, -9];
        let v3s = [8, 11, 2, 11, 1, 5, 10, 6, 1, 11, 9, 14, 11, 2];

        let mut max_digits = HashMap::<usize, isize>::new();

        let mut needed_zs = HashSet::from([0isize]);

        for v in (0..v1s.len()).rev() {
            let mut next_zs = HashSet::new();
            for z in 0..100 {
                for i in 1..=9 {
                    let result = run_for(i, z, v1s[v], v2s[v], v3s[v]);
                    if needed_zs.contains(&result) {
                        // println!("v={}, z={}, i={}", v, z, i);
                        let cur_max = *max_digits.get(&v).unwrap_or(&0);
                        if i > cur_max {
                            max_digits.insert(v, i);
                        }
                        next_zs.insert(z);
                    }
                }
            }
            needed_zs = next_zs;
        }
        println!("{:?}", max_digits);
    }
}
