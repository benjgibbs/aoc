use std::fmt::Display;

use aoc::read_lines;

struct ALU {
    reg: [isize; 4],
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
        write!(f, "w={:<10} x={:<10} y={:<10} z={:<10}", self.reg[0], self.reg[1], self.reg[2], self.reg[3])
    }
}

impl ALU {
    fn new() -> ALU {
        ALU { reg: [0, 0, 0, 0] }
    }

    fn input(&mut self, a: &Register, v: isize) {
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
            _ => panic!("Failed to parse: {}", a)
        }
    }

    fn deref(&self, b: &str) -> isize {
        if let Ok(b_val) = isize::from_str_radix(b, 10) {
            return b_val;
        } else {
            self.reg[self.str_to_reg(b) as usize]
        }
    }
}
// fn input_nums() -> Vec<isize> {
//     Vec::from([12345678911234, 11111111111111])
// }
fn most_signficant(mut n: isize) -> (isize, isize) {
    let mut pow = 1;
    while n > 9 {
        n /= 10;
        pow += 1
    }
    (n, pow)
}

fn part1(lines: &Vec<&str>) -> i32 {
    for input in (1111111111111..99999999999999).rev() {
    //let v = 11111111111113;
    //for input in 1..9  {
        if input.to_string().contains("0") {
            continue;
        }
        let result = run_prog(lines,input);
        if result == 0 {
            println!("{:15},{}", input, result);
        }
        
    }
    return 0
}
fn run_prog(lines: &Vec<&str>, input: isize) -> isize {
        let mut input_remainder = input;

        let mut alu = ALU::new();
        for l in lines.iter() {
            let parts : Vec<&str> = l.split(" ").collect();
            match parts[0] {
                "inp" => {
                    let (digit,pow) = most_signficant(input_remainder);
                    input_remainder -= digit * 10isize.pow(pow as u32 - 1);

                    //println!("Inputting: {}", digit);

                    alu.input(&alu.str_to_reg(parts[1]), digit);
                },
                "add" => {
                    alu.add(&alu.str_to_reg(parts[1]), parts[2])
                },
                "mul" => {
                    alu.mul(&alu.str_to_reg(parts[1]), parts[2])
                },
                "div" => {
                    alu.div(&alu.str_to_reg(parts[1]), parts[2])
                },
                "mod" => {
                    alu.modulo(&alu.str_to_reg(parts[1]), parts[2])
                },
                "eql" => {
                    alu.eql(&alu.str_to_reg(parts[1]), parts[2])
                }
                _ => panic!("Bad instruction: {}", parts[0])
            }
            //println!("{:10} => {}", l, alu);
        }
    return alu.reg[Register::Z as usize];
}

fn part2(_lines: &Vec<String>) -> i32 {
    return 0;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day24.txt") {
        println!("Part 1: {}", part1(&lines.iter().map(|s|s.as_str()).collect()));
        println!("Part 2: {}", part2(&lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24_partial() {
        if let Ok(lines) = read_lines("./input/partial24.txt") {
            for input in 11..=99 {
                let result = run_prog(&lines.iter().map(|s|s.as_str()).collect(), input as isize);
                if result == 0 {
                    println!("\n** {}", input);
                }
                
            }
        }
    }
    

    #[test]
    fn day24_input1() {
        let lines =  Vec::from( [
            "inp w",
            "mul x 0",
            "add x z",
            "mod x 26",
            "div z 1",
            "add x 14",
            "eql x w",
            "eql x 0",
            "mul y 0",
            "add y 25",
            "mul y x",
            "add y 1",
            "mul z y",
            "mul y 0",
            "add y w",
            "add y 8",
            "mul y x",
            "add z y",
        ]);

        for i in 1..=9 {
            let input = i * 10isize.pow(1);
            println!("\n{} => {}\n", i, run_prog(&lines, input));
        }
    }
}
