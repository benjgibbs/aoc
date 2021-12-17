use std::cmp::max;

fn run(x1: i32, x2: i32, y1: i32, y2: i32) -> (i32, u32) {
    let mut result = i32::MIN;
    let mut count = 0;

    for dy in y1..150 {
        for dx in 0..=x2 {
            let mut hit = false;
            let mut max_y = i32::MIN;

            let mut pos = (0, 0);
            let mut dx2 = dx;
            let mut dy2 = dy;
            if dx == 6 && dy == 3 {
                println!();
            }
            while pos.1 >= y1 {
                max_y = max(max_y, pos.1);
                if pos.0 >= x1 && pos.0 <= x2 && pos.1 >= y1 && pos.1 <= y2 {
                    hit = true;
                    break;
                }

                pos.0 += dx2;
                pos.1 += dy2;
                
                if dx2 > 0 {
                    dx2 -= 1;
                } else if dx2 < 0 {
                    dx2 += 1;
                }
                dy2 -= 1;
            }

            if hit {
                //println!("Hit: ({},{}), max: {}", dx, dy, max_y);
                result = max(max_y, result);
                count += 1;
            }
        }
    }

    return (result, count);
}

fn main() {
    let result = run(143, 177, -106, -71);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_example() {
        let result = run(20, 30, -10, -5);
        assert_eq!(45, result.0);
        assert_eq!(112, result.1);
    }

    #[test]
    fn day17_result() {
        let result = run(143, 177, -106, -71);
        assert_eq!(5565, result.0);
        assert_eq!(2118, result.1);
    }
}
