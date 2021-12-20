use std::collections::VecDeque;

use aoc::read_lines;

fn part1(_lines: &Vec<String>) -> i32 {
    return 0;
}

fn part2(_lines: &Vec<String>) -> i32 {
    return 0;
}

fn main() {
    if let Ok(lines) = read_lines("./input/day10.txt") {
        println!("Part 1: {}", part1(&lines));
        println!("Part 2: {}", part2(&lines));
    }
}

#[derive(Debug, Eq, Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: Option<u32>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right && self.val == other.val
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

fn is_val(result: &(Option<Node>, bool, u32, u32)) -> bool {
    if result.0.is_none() {
        return false;
    }
    result.0.as_ref().unwrap().val.is_some()
}

fn is_branch(result: &(Option<Node>, bool, u32, u32)) -> bool {
    result.0.as_ref().unwrap().left.is_some() || result.0.as_ref().unwrap().right.is_some()
}

fn clone_node(result: &(Option<Node>, bool, u32, u32)) -> Option<Box<Node>> {
    Some(Box::new(result.0.as_ref().unwrap().clone()))
}

fn explode_iter(node: &Node, depth: u32) -> (Option<Node>, bool, u32, u32) {
    /* Cases
        1. Depth ==3 - so L & R should be values - a pair - remove node an return sums
        2. Depth == 3
            Left is a val, right is a pair
                add right.left to left,
                pass right.right up
            right is a val, left is a pair
                add left.right to right
                pass left.left up
            left is val and right is a val
                this is a pair, do nothing
            left is a pair and right is a pair
                new node with left = l.l and r.l and right = l.r and r.r
        3. Dept < 3
            left is a val and right is a Pair
                left adds right.left_overflow
            right is a val and left is a Pair
                right adds left.right_overflow
            left is a val and rights is a val
                this is a pair return as is
            right is a node and left is a node
                return left and right with overflows summed
    */

    if node.val.is_some() {
        return (
            Some(node.clone()),
            false,
            0,
            0
        );
    }

    if depth == 4 {
        return (
            None,
            true,
            node.left.as_ref().unwrap().val.unwrap(),
            node.right.as_ref().unwrap().val.unwrap(),
        );
    } else {

        let left_result = explode_iter(&node.left.as_ref().unwrap(), depth + 1);
        
        
        
        
        
        let right_result = if left_result.1 {
            

            (Some(*node.right.as_ref().unwrap().clone()), false, 0, 0)
        } else {
            explode_iter(&node.right.as_ref().unwrap(), depth + 1)
        };



        //println!("left result: {:?}", left_result);
        //println!("right result: {:?}", right_result);

        if depth == 3 {
            if is_val(&left_result) && right_result.0.is_none() {
                let val = left_result.0.unwrap().val.unwrap() + right_result.2;
                return (
                    Some(Node {
                        left: make_val(val),
                        right: make_val(0),
                        val: None,
                    }),
                    true,
                    0,
                    right_result.3,
                );
            } else if is_val(&right_result) && left_result.0.is_none() {
                let val = right_result.0.unwrap().val.unwrap() + left_result.3;
                return (
                    Some(Node {
                        left: make_val(0),
                        right: make_val(val),
                        val: None,
                    }),
                    true,
                    left_result.2,
                    0,
                );
            } else if is_val(&right_result) && is_val(&left_result) {
                return (
                    Some(Node {
                        left: make_val(left_result.0.unwrap().val.unwrap()),
                        right: make_val(right_result.0.unwrap().val.unwrap()),
                        val: None,
                    }),
                    left_result.1 || right_result.1,
                    0,
                    0,
                );
            } else {
                panic!();
            }
        } else { // depth < 3
            if is_val(&left_result) && !is_val(&right_result) {
                return (Some(
                    Node {
                        left: make_val(left_result.0.unwrap().val.unwrap() + right_result.2),
                        right: clone_node(&right_result),
                        val: None,
                    }),
                    right_result.1,
                    0,
                    right_result.3,
                );
            } else if is_val(&right_result) && !is_val(&left_result) {
                return (Some(
                    Node {
                        left: clone_node(&left_result),
                        right: make_val(right_result.0.unwrap().val.unwrap() + left_result.3),
                        val: None,
                    }),
                    left_result.1,
                    left_result.2,
                    0,
                );
            } else if is_val(&right_result) && is_val(&left_result) {
                return (
                    Some(Node {
                        left: clone_node(&left_result),
                        right: clone_node(&right_result),
                        val: None,
                    }),
                    left_result.1 || right_result.1,
                    0,
                    0,
                );
            } else {
                return (
                    Some(Node {
                        left: clone_node(&left_result),
                        right: clone_node(&right_result),
                        val: None,
                    }),
                    left_result.1 || right_result.1,
                    left_result.2 + right_result.2,
                    left_result.3 + right_result.3,
                );
            }
        }
    }
}
// fn explode(mut node: Option<Box<Node>>) -> bool {
//     return explode_iter(node, 0).0;
// }

// fn split(mut node: Option<Box<Node>>) -> bool {
//     false
// }

// fn reduce(mut node: Option<Box<Node>>) {
//     loop {
//         let r1 = explode(node);
//         let r2 = split(node);
//         if !r1 && !r2 {
//             break;
//         }
//     }
// }

fn make_val(val: u32) -> Option<Box<Node>> {
    Some(Box::new(Node {
        left: None,
        right: None,
        val: Some(val),
    }))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_explode_1() {
        let input = parse("[[[[[9,8],1],2],3],4]");
        let expect = parse("[[[[0,9],2],3],4]");

        let result = explode_iter(&input.unwrap(), 0);
        assert_eq!(result.0.unwrap(), expect.unwrap())
    }

    #[test]
    fn test_explode_2() {
        let input = parse("[7,[6,[5,[4,[3,2]]]]]");
        let expect = parse("[7,[6,[5,[7,0]]]]");

        let result = explode_iter(&input.unwrap(), 0);
        assert_eq!(result.0.unwrap(), expect.unwrap())
    }

    #[test]
    fn test_explode_3() {
        let input = parse("[[6,[5,[4,[3,2]]]],1]");
        let expect = parse("[[6,[5,[7,0]]],3]");

        let result = explode_iter(&input.unwrap(), 0);
        assert_eq!(result.0.unwrap(), expect.unwrap())
    }

    #[test]
    fn test_explode_4() {
        let input = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let expect = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        let result = explode_iter(&input.unwrap(), 0);
        assert_eq!(result.0.unwrap(), expect.unwrap())
    }

    #[test]
    fn test_explode_5() {
        let input = parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let expect = parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

        let result = explode_iter(&input.unwrap(), 0);
        assert_eq!(result.0.unwrap(), expect.unwrap())
    }

    #[test]
    fn parse_test_1() {
        let result = parse("[1,2]");
        let expect = Some(Node {
            left: make_val(1),
            right: make_val(2),
            val: None,
        });

        assert_eq!(result, expect);
    }

    #[test]
    fn parse_test_2() {
        let result = parse("[[1,2],3]");
        let expect = Some(Node {
            left: Some(Box::new(Node {
                left: make_val(1),
                right: make_val(2),
                val: None,
            })),
            right: make_val(3),
            val: None,
        });

        assert_eq!(result, expect);
    }

    #[test]
    fn parse_test_3() {
        let result = parse("[9,[8,7]]");
        let expect = Some(Node {
            left: make_val(9),
            right: Some(Box::new(Node {
                left: make_val(8),
                right: make_val(7),
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
    fn day10_example1() {
        if let Ok(lines) = read_lines("./input/example10.txt") {
            assert_eq!(0, part1(&lines));
        }
    }

    #[test]
    fn day10_example2() {
        if let Ok(lines) = read_lines("./input/example10.txt") {
            assert_eq!(0, part2(&lines));
        }
    }
}
