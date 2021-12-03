use aoc::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./input/day3.txt") {
        let mut line_couunt = 0;
        let mut counts = vec![0,0,0,0,0,0,0,0,0,0,0,0];
        //let mut counts = vec![0,0,0,0,0];

        for l in lines.iter() {
            for i in 0..l.len() {
                if l.chars().nth(i) == Some('1') {
                    counts[i] += 1;
                }
            }
            line_couunt += 1;
        }
        println!("{:?}", counts);
        let mut pop : u32 = 0;
        let mut pow = 1;
        for c in counts.iter().rev() {
            if *c > line_couunt/2 {
                pop += pow;
            }
            pow *= 2;
        }
        println!("{} {}", pop, int_to_bin_string(pop));

        let base : u32 = 2;
        let n : u32 = base.pow(counts.len() as u32)-1;        
        let unpop = pop ^ n;
        println!("{} {}", unpop, int_to_bin_string(unpop));
        println!("Part 1: {}", pop * unpop);

        let mut o2_gen = lines.clone();
        let mut idx : usize = 0;
        while o2_gen.len() > 1 {
            o2_gen = filter(o2_gen, idx);
            println!("Keeping: {:?}", o2_gen);
            idx += 1;
        }
        println!("{:?}", o2_gen);

        let mut co2_scrub = lines.clone(); 
        idx = 0;
        while co2_scrub.len() > 1 {
            co2_scrub = filter2(co2_scrub, idx);
            
            idx += 1;
        }
        println!("{:?}", co2_scrub);
        let co2 = isize::from_str_radix(co2_scrub.get(0).unwrap(), 2).unwrap();
        let o2 = isize::from_str_radix(o2_gen.get(0).unwrap(), 2).unwrap();
        println!("Part 2: {} {} {}",co2, o2,  co2 * o2);
    }

}

fn filter(inputs: Vec<String>, bit: usize) -> Vec<String> {
    let mut one_count = 0;
    for i in inputs.iter() {
        if i.chars().nth(bit) == Some('1') {
            one_count += 1usize;
        }
    }
    let zero_count = inputs.len() - one_count;
    let char_to_keep = Some(if one_count > zero_count || one_count == zero_count { '1' } else { '0' });
    println!("Keeping: {:?} in pos {} one_count {}", char_to_keep, bit, one_count);
    return inputs.iter().filter(|i| i.chars().nth(bit) == char_to_keep).map(|x| x.to_string()).collect();

}

fn filter2(inputs: Vec<String>, bit: usize) -> Vec<String> {
    let mut one_count = 0;
    for i in inputs.iter() {
        if i.chars().nth(bit) == Some('1') {
            one_count += 1usize;
        }
    }
    let char_to_keep = Some(if one_count < (inputs.len()-one_count) { '1' } else { '0' });
    return inputs.iter().filter(|i| i.chars().nth(bit) == char_to_keep).map(|x| x.to_string()).collect();
}

fn int_to_bin_string(mut i: u32) -> String {
    let mut result = Vec::new();
    while i > 0 {
        if i & 0x1 == 0x1 {
            result.push("1");
        } else {
            result.push("0");
        }
        i /= 2;
    }
    result.reverse();
    return result.join("");
}