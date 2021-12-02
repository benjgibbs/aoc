use aoc::{get_nums, read_lines};

fn main() {
  let moves = read_lines("./input/day2.txt");
  let mut x = 0;
  let mut y = 0;

  for m in moves.iter() {
    let dist: i32 = *get_nums(&m).get(0).unwrap();

    if m.starts_with("up") {
      y -= dist;
    } else if m.starts_with("down") {
      y += dist;
    } else if m.starts_with("forward") {
      x += dist;
    } else {
      println!("Unexpected: {}", m);
    }
  }
  println!("Part1: {}, {}: {}", x, y, x * y);

  x = 0;
  y = 0;
  let mut aim = 0;

  for m in moves.iter() {
    let dist: i32 = *get_nums(&m).get(0).unwrap();

    if m.starts_with("up") {
      aim -= dist;
    } else if m.starts_with("down") {
      aim += dist;
    } else if m.starts_with("forward") {
      x += dist;
      y += aim * dist;
    } else {
      println!("Unexpected: {}", m);
    }
  }
  println!("Part2: {}, {}, {}: {}", x, y, aim, x * y);
}
