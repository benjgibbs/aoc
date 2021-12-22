use std::{fmt::Display, collections::VecDeque};

use aoc::read_lines;

fn part1(lines: &Vec<&str>) -> usize {
    let current = sum_list(&lines);
    current.magnitude()
}

fn part2(lines: &Vec<String>) -> u32 {
    let numbers : Vec<Number> = lines.iter().map(|l| Number::parse(l)).collect();

    let mut max = u32::MIN;

    for i in 0..numbers.len()  {
        for j in 1..numbers.len() {
            let n1 = &numbers[i];
            let n2 = &numbers[j];
            let s1 = n1.add(n2);
            let s2 = n2.add(n1);
            max = max.max(s1.magnitude() as u32);
            max = max.max(s2.magnitude() as u32);

        }
    }

    return max;
}

fn sum_list(lines: &Vec<&str>) -> Number {
    let mut current = Number::parse(&lines[0]);
    for line in lines.iter().skip(1) {
        let next = Number::parse(line);
        current = current.add(&next);
        current.reduce();
    }
    current
}
fn main() {
    if let Ok(lines) = read_lines("./input/day18.txt") {
        println!("Part 1: {}", part1(&lines.iter().map(|l| l.as_str()).collect()));
        println!("Part 2: {}", part2(&lines));
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Token {
    Num(u32),
    Open,
    Close
}

#[derive(PartialEq, Eq, Debug)]
struct Number {
    number: Vec<Token>
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();

        for token in self.number.iter() {
            match token {
                Token::Open => {
                    if let Some(c) = buffer.chars().last() {
                        if c != '[' {
                            buffer.push(',');
                        }
                    }
                    buffer.push_str("[")
                },
                Token::Close => buffer.push(']'),
                Token::Num(v) => {
                    if let Some(c) = buffer.chars().last() {
                        if c != '[' {
                            buffer.push(',');
                        }
                    }
                    buffer.push_str(&v.to_string())
                }
            }
        }
        

        write!(f, "{}", buffer)
    }
}

impl Number {
    fn parse(input :&str) -> Number {
        let mut result = Vec::new();
        let chars : Vec<char> = input.chars().collect();
        let mut idx = 0;
        while idx < chars.len() {
            match chars[idx] {
                '[' => result.push(Token::Open),
                ']' => result.push(Token::Close),
                ',' => {},
                _ => {
                    let start = idx; 
                    while chars[idx].is_digit(10) {
                        idx += 1;
                    }
                    let num_string : String = chars[start..idx].iter().collect();
                    let num = u32::from_str_radix(&num_string, 10).unwrap();
                    
                    result.push(Token::Num(num));
                    idx -= 1;
                }
            }
            idx += 1;
        }
        Number { number: result }
    }
    fn add(&self, n2: &Number) -> Number {
        let mut result = Vec::new();
        result.push(Token::Open);
        result.extend(self.number.iter());
        result.extend(n2.number.iter());
        result.push(Token::Close);

        let mut n2 = Number {number: result};
        n2.reduce();
        
        n2
    }
    fn explode(&mut self) -> bool {
        let mut depth = 0;
        for i in 0..self.number.len() {
            let t = self.number[i];
            match t {
                Token::Open => {
                    depth += 1;
                },
                Token::Close => {
                    depth -= 1;
                },
                Token::Num(n2) => {
                    if depth > 4 {
                        if let Token::Num(n1) = self.number[i-1] {
                            // second in pair

                            let mut new_number = Vec::new();
                            new_number.extend(&self.number[0..i-2]);
                            new_number.push(Token::Num(0));
                            new_number.extend(&self.number[i+2..]);
                            

                            for j in (0..i-2).rev() {
                                if let Token::Num(n3) = new_number[j] {
                                    new_number[j] = Token::Num(n3 + n1);
                                    break;
                                }
                            }

                            for j in (i-1)..new_number.len() {
                                if let Token::Num(n3) = new_number[j] {
                                    new_number[j] = Token::Num(n3 + n2);
                                    break;
                                }
                            }
                            self.number = new_number;
                            return true;

                        }
                    }
                }
            }
        }            
        
        false
    }

    fn split(&mut self) -> bool {
        let mut done_replace = false;
        let mut new_number = Vec::new();
        for n in self.number.iter() {
            match n {
                Token::Num(val) => {
                    if *val >= 10 &&  !done_replace {
                        let a = val / 2;
                        let b = val - a;
                        new_number.push(Token::Open);
                        new_number.push(Token::Num(a));
                        new_number.push(Token::Num(b));
                        new_number.push(Token::Close);
                        done_replace = true;

                    } else {
                        new_number.push(*n)
                    }
                },
                _ => new_number.push(*n)
            }
        }
        self.number = new_number;
        done_replace
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() {
                if !self.split() {
                    break;
                }
            }
        }
    }

    fn magnitude(&self) -> usize {
        let mut stack = VecDeque::new();
        for p in self.number.iter() {
            match p {
                Token::Num(n) => stack.push_back(*n as usize),
                Token::Close => {
                    if let Some(rhs) = stack.pop_back() {
                        if let Some(lhs) = stack.pop_back() {
                            let a = 3 * lhs as usize;
                            let b = 2 * rhs as usize;
                            stack.push_back( a + b);
                        }
                    }
                }
                _ => {}
            }
        }
        stack.pop_back().unwrap()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day18_example1() {
        if let Ok(lines) = read_lines("./input/example18.txt") {
            assert_eq!(4140, part1(&lines.iter().map(|s| s.as_str()).collect()));
        }
    }

    #[test]
    fn day18_example2() {
        if let Ok(lines) = read_lines("./input/example18.txt") {
            assert_eq!(0, part2(&lines));
        }
    }

    #[test]
    fn test_explode_1() {
        let mut root = Number::parse("[[[[[9,8],1],2],3],4]");
        let expect = Number::parse("[[[[0,9],2],3],4]");

        let result = root.explode();
        assert_eq!(true, result);
        assert_eq!(root, expect)
    }

    #[test]
    fn test_explode_2() {
        let mut root = Number::parse("[7,[6,[5,[4,[3,2]]]]]");
        let expect = Number::parse("[7,[6,[5,[7,0]]]]");

        let result = root.explode();
        assert_eq!(true, result);
        assert_eq!(root, expect)
    }

    #[test]
    fn test_explode_3() {
        let mut root = Number::parse("[[6,[5,[4,[3,2]]]],1]");
        let expect = Number::parse("[[6,[5,[7,0]]],3]");

        let result = root.explode();
        assert_eq!(true, result);
        assert_eq!(root, expect);
    }

    #[test]
    fn test_explode_4() {
        let mut root = Number::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let expect = Number::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let result = root.explode();
        assert_eq!(true, result);
        assert_eq!(root, expect)
    }

    #[test]
    fn test_explode_5() {
        let mut root = Number::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let expect = Number::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

        let result = root.explode();
        assert_eq!(true, result);
        assert_eq!(root, expect);
    }

    #[test]
    fn test_explode_6() {
        let mut root = Number::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let expect = Number::parse("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");

        let result = root.explode();
        assert_eq!(true, result);
        assert_eq!(root, expect);
    }

    #[test]
    fn test_explode_7() {
        let mut root = Number::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[6,6],[5,0]],[[6,6],[1,[[5,6],1]]]]]");
        let expect =   Number::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[6,6],[5,0]],[[6,6],[6,[0,7]]]]]");

        let result = root.explode();
        assert_eq!(true, result);
        assert_eq!(root, expect);
    }


    #[test]
    fn test_add() {
        let lhs = Number::parse("[1,2]");
        let rhs = Number::parse("[[3,4],5]");
        let expect = Number::parse("[[1,2],[[3,4],5]]");
        assert_eq!(lhs.add(&rhs), expect);
    }

    
    #[test]
    fn day18_reduce_test() {
        let n1 = Number::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let n2 = Number::parse("[1,1]");
        let mut root = n1.add(&n2);
        root.reduce();
        let expect = Number::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(root, expect);
    }

    #[test]
    fn day18_magnitude_tests() {
        assert_eq!(143, Number::parse("[[1,2],[[3,4],5]]").magnitude());
        assert_eq!(1384, Number::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude());
        assert_eq!(445, Number::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude());
        assert_eq!(791, Number::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude());
        assert_eq!(1137, Number::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude());
        assert_eq!(3488, Number::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude());
    }

    #[test]
    fn day18_sum_0() {
        let vals = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]"
            ];
        
        let expect = Number::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(expect, sum_list(&vals));
    }

    #[test]
    fn day18_sum_1() {
        let vals = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
            "[5,5]",
            ];
        
        let expect = Number::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(expect, sum_list(&vals));
    }

    #[test]
    fn day18_sum_2() {
        let vals = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
            "[5,5]",
            "[6,6]",
            ];
        
        let expect = Number::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(expect, sum_list(&vals));
    }


    #[test]
    fn day18_sum_3() {
        let v1 = Number::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let v2 = Number::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        let expect = Number::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        let mut result = v1.add(&v2);
        result.reduce();
        assert_eq!(expect, result);
    }

    #[test]
    fn day18_sum_4() {
        let v1 = Number::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        let v2 = Number::parse("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]");
        let expect = Number::parse("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
        let mut result = v1.add(&v2);
        result.reduce();
        assert_eq!(expect, result);
    } 

    #[test]
    fn day18_sum_11() {
        let v1 = Number::parse("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]");
        let v2 = Number::parse("[[[[4,2],2],6],[8,7]]");
        let expect = Number::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        let mut result = v1.add(&v2);
        result.reduce();
        assert_eq!(expect, result);
    } 
  

    #[test]
    fn day18_explode_1() {
        let input = "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]";
        let mut n1 = Number::parse(input);
        n1.explode();
        let expect = Number::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        println!("Start: {}", input);
        println!("Got:   {}", n1);
        println!("Want:  {}", expect);

        assert_eq!(n1, expect);
    }
}
