
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    if let Ok(lines) = read_lines("./input/day1.txt") {
        let mut last = 0;
        let mut count = 0;
        let mut first = true;
        let mut depths : Vec<i32> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                let current = ip.parse::<i32>().unwrap(); 
                depths.push(current);
                if first {
                    first = false;
                } else {
                    if current > last {
                        count += 1;
                    }
                }
                last = current;
            }
    
        }
        println!("Depth increased: {}", count);

        
        let mut sums : Vec<i32> = Vec::new();
        count = 0;

        for i in 2..depths.len() {
            sums.push(depths[i] + depths[i-1] + depths[i-2]);
        }
        last = sums[0];
        for i in 1..sums.len() {
            if sums[i] > last {
                count += 1;
            }
            last = sums[i];
        }

        println!("Depth increased: {}", count);

    }

}