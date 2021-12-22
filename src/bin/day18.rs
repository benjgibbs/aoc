use std::{collections::VecDeque, fmt::Display};

use aoc::read_lines;

fn part1(lines: &Vec<&str>) -> u64 {
    let current = sum_list(&lines);
    current.magnitude()
}

fn part2(_lines: &Vec<String>) -> i32 {
    return 0;
}

fn sum_list(lines: &Vec<&str>) -> Node {
    let mut current = parse(&lines[0]).unwrap();
    for line in lines.iter().skip(1) {
        let next =parse(line).unwrap();
        current = add(current, next);
        reduce(&mut current);
    }
    current
}
fn main() {
    if let Ok(lines) = read_lines("./input/day18.txt") {
        println!("Part 1: {}", part1(&lines.iter().map(|l| l.as_str()).collect()));
        println!("Part 2: {}", part2(&lines));
    }
}

#[derive(Debug, Eq, Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: Option<u32>,
}

impl Node {
    fn is_pair(&self) -> bool {
        let null_val = Box::new(Node {
            left: None,
            right: None,
            val: None,
        });
        self.left.as_ref().unwrap_or(&null_val).val.is_some()
            && self.right.as_ref().unwrap_or(&null_val).val.is_some()
    }

    fn get_pair(&self) -> (u32, u32) {
        assert!(self.is_pair());
        return (
            self.left.as_ref().unwrap().val.unwrap(),
            self.right.as_ref().unwrap().val.unwrap(),
        );
    }

    fn is_regular(&self) -> bool {
        self.val.is_some()
    }

    fn new_val(val: u32) -> Option<Box<Node>> {
        Some(Box::new(Node { left: None, right: None, val: Some(val)}))
    }

    fn display_iter(&self, buffer: &mut String) {
        if self.is_regular() {
            buffer.push_str(self.val.unwrap().to_string().as_str());
        } else {
            buffer.push('[');
            self.left.as_ref().unwrap().display_iter(buffer);
            buffer.push(',');
            self.right.as_ref().unwrap().display_iter(buffer);
            buffer.push(']');
        }
    }

    fn magnitude(&self) -> u64 {
        if self.is_regular() {
            return self.val.unwrap() as u64;
        }
        return 3 * self.left.as_ref().unwrap().magnitude() + 2 * self.right.as_ref().unwrap().magnitude();
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right && self.val == other.val
    }
}

impl Display for Node {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();

        self.display_iter(&mut buffer);

        write!(f, "{}", buffer)
    }
}

fn parse(s: &str) -> Option<Node> {
    let mut stack = VecDeque::new();
    for d in s.chars() {
        match d {
            '[' => stack.push_back(Node {
                left: None,
                right: None,
                val: None,
            }),
            ']' => {
                let rhs = stack.pop_back().unwrap();
                let lhs = stack.pop_back().unwrap();
                let mut parent = stack.pop_back().unwrap();
                parent.left = Some(Box::new(lhs));
                parent.right = Some(Box::new(rhs));
                stack.push_back(parent);
            }
            ',' => {}
            _ => {
                let node = Node {
                    left: None,
                    right: None,
                    val: Some(d.to_digit(10).unwrap()),
                };
                stack.push_back(node);
            }
        }
    }
    stack.pop_back()
}

fn add(n1: Node, n2: Node) -> Node {
    Node {
        left: Some(Box::new(n1)),
        right: Some(Box::new(n2)),
        val: None,
    }
}


fn update_left(mut node: &mut Node, val: u32) {
    while !node.is_regular() {
        node = node.right.as_mut().unwrap();
    }
    if let Some(v) = node.val {
        node.val = Some(v + val);
    }
}

fn update_right(mut node: &mut Node, val: u32) {
    while !node.is_regular() {
        node = node.left.as_mut().unwrap();
    }
    if let Some(v) = node.val {
        node.val = Some(v + val);
    }
}

fn do_explode(node: &mut Node, depth: u32) -> (bool, u32, u32) {
    if node.is_regular() {
        return (false, 0, 0);
    }

    //println!("{} => {}", depth, node);
    if depth > 2 {
        if let Some(left_node) = &node.left {
            if let Some(right_node) = &node.right {
                
                if left_node.is_pair() && right_node.is_regular() {
                    let left_pair =left_node.get_pair();
                    node.left = Node::new_val(0);
                    node.right = Node::new_val(right_node.val.unwrap() + left_pair.1);
                    return (true, left_pair.0, 0);
                } else if left_node.is_regular() && right_node.is_pair() {
                    let right_pair =right_node.get_pair();
                    node.left = Node::new_val(left_node.val.unwrap() + right_pair.0);
                    node.right = Node::new_val(0);
                    return (true, 0, right_pair.1);
                } else if left_node.is_pair() && right_node.is_pair() {
                    let left_pair =left_node.get_pair();
                    node.left = Node::new_val(0);
                    update_right(&mut node.right.as_mut().unwrap(), left_pair.1);
                    return (true, left_pair.0, 0);
                } else if left_node.is_regular() && right_node.is_regular() {
                    return (false, 0, 0);
                } 
            }
        }
    }


    let left_result = do_explode(&mut node.left.as_mut().unwrap(), depth + 1);
    if left_result.0 {
        // if left_result.1 > 0 {
        //     return (true, left_result.1, left_result.2);
        // }
        if left_result.2 > 0 {
            if let Some(right) = node.right.as_mut() {
                if right.is_regular() {
                    node.right = Node::new_val(right.val.unwrap() + left_result.2);
                } else {
                    update_right(right, left_result.2);
                }
                return (true, left_result.1, 0);
            }
        }
        return (left_result.0, left_result.1, 0)
    } else {
        let right_result = do_explode(&mut node.right.as_mut().unwrap(), depth + 1);
        // if right_result.2 > 0 {
        //     return (true, right_result.1, right_result.2);
        // }
        if right_result.1 > 0 {
            if let Some(left) = node.left.as_mut() {
                if left.is_regular() {
                    node.left = Node::new_val(left.val.unwrap() + right_result.1);
                } else {
                    update_left(left, right_result.1);
                }
                return (true, 0, right_result.2);
            }
        }
        return (right_result.0, 0, right_result.2);
    }
    //return (left_result.0, 0, 0);
}

fn split(node: &mut Node) -> bool {

    if let Some(val) = node.val {
        if val >= 10 {
            let l = val / 2;
            let r = val - l;
            node.val = None;
            node.left = Node::new_val(l);
            node.right = Node::new_val(r);
            return true;
        }
        return false;
    } else {
        return split(node.left.as_mut().unwrap()) || split(node.right.as_mut().unwrap());
    }
}

fn reduce(node: &mut Node) {
     loop {
         let (r1, _, _) = do_explode(node, 0);
         println!("After Explode {}: {}", r1,  node);
         let r2 = split(node);
         println!("After Split {}: {}", r2,  node);
         if !r1 && !r2 {
            break;
        }
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
        let mut root = parse("[[[[[9,8],1],2],3],4]").unwrap();
        let expect = parse("[[[[0,9],2],3],4]");

        let result = do_explode(&mut root, 0);
        assert_eq!(true, result.0);
        assert_eq!(root, expect.unwrap())
    }

    #[test]
    fn test_explode_2() {
        let mut root = parse("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let expect = parse("[7,[6,[5,[7,0]]]]");

        let result = do_explode(&mut root, 0);
        assert_eq!(true, result.0);
        assert_eq!(root, expect.unwrap())
    }

    #[test]
    fn test_explode_3() {
        let mut root = parse("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let expect = parse("[[6,[5,[7,0]]],3]");

        let result = do_explode(&mut root, 0);
        assert_eq!(true, result.0);
        assert_eq!(root, expect.unwrap())
    }

    #[test]
    fn test_explode_4() {
        let mut root = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let expect = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let result = do_explode(&mut root, 0);
        assert_eq!(true, result.0);
        assert_eq!(root, expect.unwrap())
    }

    #[test]
    fn test_explode_5() {
        let mut root = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        let expect = parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

        let result = do_explode(&mut root, 0);
        assert_eq!(true, result.0);
        assert_eq!(root, expect.unwrap())
    }

    #[test]
    fn test_explode_6() {
        let mut root = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        let expect = parse("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");

        let result = do_explode(&mut root, 0);
        assert_eq!(true, result.0);
        assert_eq!(root, expect.unwrap())
    }

    #[test]
    fn test_explode_7() {
        let mut root = parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[6,6],[5,0]],[[6,6],[1,[[5,6],1]]]]]").unwrap();
        let expect =   parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[6,6],[5,0]],[[6,6],[6,[0,7]]]]]").unwrap();

        let result = do_explode(&mut root, 0);
        assert_eq!(true, result.0);
        assert_eq!(root, expect);
    }

    #[test]
    fn parse_test_1() {
        let result = parse("[1,2]");
        let expect = Some(Node {
            left: Node::new_val(1),
            right: Node::new_val(2),
            val: None,
        });

        assert_eq!(result, expect);
    }

    #[test]
    fn parse_test_2() {
        let result = parse("[[1,2],3]");
        let expect = Some(Node {
            left: Some(Box::new(Node {
                left: Node::new_val(1),
                right: Node::new_val(2),
                val: None,
            })),
            right: Node::new_val(3),
            val: None,
        });

        assert_eq!(result, expect);
    }

    #[test]
    fn parse_test_3() {
        let result = parse("[9,[8,7]]");
        let expect = Some(Node {
            left: Node::new_val(9),
            right: Some(Box::new(Node {
                left: Node::new_val(8),
                right: Node::new_val(7),
                val: None,
            })),
            val: None,
        });

        assert_eq!(result, expect);
    }

    #[test]
    fn test_add() {
        let lhs = parse("[1,2]").unwrap();
        let rhs = parse("[[3,4],5]").unwrap();
        let expect = parse("[[1,2],[[3,4],5]]");
        assert_eq!(add(lhs, rhs), expect.unwrap());
    }

    
    #[test]
    fn day18_reduce_test() {
        let n1 = parse("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let n2 = parse("[1,1]").unwrap();
        let mut root = add(n1, n2);
        reduce(&mut root);
        let expect = parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(root, expect);
    }

    #[test]
    fn day18_magnitude_tests() {
        assert_eq!(143, parse("[[1,2],[[3,4],5]]").unwrap().magnitude());
        assert_eq!(1384, parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().magnitude());
        assert_eq!(445, parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap().magnitude());
        assert_eq!(791, parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap().magnitude());
        assert_eq!(1137, parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap().magnitude());
        assert_eq!(3488, parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap().magnitude());
    }

    #[test]
    fn day18_sum_0() {
        let vals = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]"
            ];
        
        let expect = parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
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
        
        let expect = parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
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
        
        let expect = parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        assert_eq!(expect, sum_list(&vals));
    }


    #[test]
    fn day18_sum_3() {
        let v1 = parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]").unwrap();
        let v2 = parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]").unwrap();
        let expect = parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]").unwrap();
        let mut result = add(v1, v2);
        
        reduce(&mut result);
        assert_eq!(expect, result);
    }

    #[test]
    fn day18_sum_4() {
        let v1 = parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]").unwrap();
        let v2 = parse("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]").unwrap();
        let expect = parse("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]").unwrap();
        let mut result = add(v1, v2);
        
        reduce(&mut result);
        assert_eq!(expect, result);
    } 

    #[test]
    fn day18_sum_11() {
        let v1 = parse("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]").unwrap();
        let v2 = parse("[[[[4,2],2],6],[8,7]]").unwrap();
        let expect = parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        let mut result = add(v1, v2);
        
        reduce(&mut result);

        assert_eq!(expect, result);
    } 

    #[derive(PartialEq, Debug)]
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
    
    #[derive(PartialEq, Debug, Clone, Copy)]
    enum Token {
        Num(u32),
        Open,
        Close
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
                println!("Initial        : {}",  self);
                
                let explode_result = self.explode();
                println!("Post Explode {} : {}", explode_result as i32, self);
                let split_result = self.split();
                println!("Post Split   {} : {}", split_result as i32, self);

                if !explode_result && !split_result {
                    break;
                }
            }
        }
    }

    #[test]
    fn day18_linear_structure() {
        let number = Number::parse("[1,2]");
        assert_eq!("[1,2]", number.to_string());
        
        let n2 = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let num2 = Number::parse(n2);
        assert_eq!(n2, num2.to_string());

        let add2 = Number::parse("[[3,4],5]");
        let result = number.add(&add2);

        assert_eq!(result, Number::parse("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn day18_linear_structure_explode_1() {
        let mut explode_1 = Number::parse("[[[[[9,8],1],2],3],4]");
        let b = explode_1.explode();
        assert!(b);

        assert_eq!(explode_1, Number::parse("[[[[0,9],2],3],4]"));
    }
    
    #[test]
    fn day18_linear_structure_explode_2() {
        let mut explode_2 = Number::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let b = explode_2.explode();
        assert!(b);

        assert_eq!(explode_2, Number::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
    }


    #[test]
    fn day18_linear_structure_add_1() {
        let n1 = Number::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let n2 = Number::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        let result = n1.add(&n2);
        let expect = Number::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        println!("Got:  {}", result);
        println!("Want: {}", expect);

        assert_eq!(result, expect);
    }

    #[test]
    fn day18_linear_structure_add_example() {
        let n1 = Number::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let n2 = Number::parse("[1,1]");
        let result = n1.add(&n2);
        let expect = Number::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        println!("Got:  {}", result);
        println!("Want: {}", expect);

        assert_eq!(result, expect);
    }

    #[test]
    fn day18_linear_structure_exp1() {
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
