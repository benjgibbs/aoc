use aoc::{read_lines, bin_string_to_int};

fn main() {
    if let Ok(lines) = read_lines("./input/day3.txt") {
        let mut counts = vec![0; lines.get(0).unwrap().len()];

        for l in lines.iter() {
            for i in 0..l.len() {
                if l.chars().nth(i) == Some('1') {
                    counts[i] += 1;
                }
            }
        }
        let mut pop: u32 = 0;
        let mut pow = 1;
        for one_count in counts.iter().rev() {
            let zero_count = lines.len() - one_count;
            if *one_count >= zero_count {
                pop += pow;
            }
            pow *= 2;
        }

        let base: u32 = 2;
        let n: u32 = base.pow(counts.len() as u32) - 1;
        let unpop = pop ^ n;
        println!("Part 1 (693486): {}", pop * unpop);

        let mut o2_gen = lines.clone();
        let mut idx: usize = 0;
        while o2_gen.len() > 1 {
            o2_gen = filter(o2_gen, idx, |one_count, zero_count| one_count >= zero_count);
            idx += 1;
        }

        let mut co2_scrub = lines.clone();
        idx = 0;
        while co2_scrub.len() > 1 {
            co2_scrub = filter(co2_scrub, idx, |one_count, zero_count| {
                one_count < zero_count
            });
            idx += 1;
        }
        let co2 = bin_string_to_int(co2_scrub.get(0).unwrap());
        let o2 = bin_string_to_int(o2_gen.get(0).unwrap());
        println!("Part 2 (3379326) {}", co2 * o2);
    }
}

fn filter<F>(inputs: Vec<String>, bit: usize, comp: F) -> Vec<String>
where
    F: Fn(usize, usize) -> bool,
{
    let mut one_count = 0;
    for i in inputs.iter() {
        if i.chars().nth(bit) == Some('1') {
            one_count += 1usize;
        }
    }
    let zero_count = inputs.len() - one_count;
    let char_to_keep = Some(if comp(one_count, zero_count) {
        '1'
    } else {
        '0'
    });
    return inputs
        .iter()
        .filter(|i| i.chars().nth(bit) == char_to_keep)
        .map(|x| x.to_string())
        .collect();
}
