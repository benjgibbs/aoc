use std::collections::HashMap;

fn part1(p1: usize, p2: usize) -> usize {
    let mut scores = [0usize, 0usize];
    let mut pos = [p1 - 1, p2 - 1];

    let mut die = 0usize;
    let mut player = 0;

    while scores[0] < 1000 && scores[1] < 1000 {
        let rolls = 3 * ((die % 100) + 1) + 3;
        die += 3;
        pos[player] = (pos[player] + rolls) % 10;
        scores[player] += pos[player] + 1;
        player = (player + 1) % 2;
    }

    scores[0].min(scores[1]) * die
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    pos: [usize;2],
    score: [usize;2],
}

const WAYS_TO_MAKE : [(usize,usize); 7] = [
    (3,1), // (1, 1, 1)
    (4,3), // (1, 1, 2) * 3
    (5,6), // (1, 1, 3) * 3, (1, 2, 2) * 3
    (6,7), // (1, 2, 3) * 6, (2, 2, 2) 
    (7,6), // (2, 2, 1) * 3, (3, 3, 1) * 3
    (8,3), // (2, 2, 3) * 3
    (9, 1)
];


fn part2(p1: usize, p2: usize) -> usize {

    let mut player = 0;

    let mut games_in_state_count = HashMap::from([(State{pos: [p1-1, p2-1], score: [0,0]}, 1)]);
    let mut wins = [0usize, 0usize];

    loop {
        let mut new_state_counts = HashMap::new();
        for (state, count) in games_in_state_count.iter(){
            for (total, way_count) in WAYS_TO_MAKE {
                let pos = (state.pos[player] + total) % 10;
                let score = state.score[player] + pos + 1;
                if score > 20 {
                    wins[player] += way_count * *count;
                } else {
                    let mut new_state: State = state.clone();
                    new_state.score[player] = score;
                    new_state.pos[player] = pos;
                    *new_state_counts.entry(new_state).or_insert(0) += way_count * *count;
                }
            }
        }
        if new_state_counts.is_empty() {
            break;
        } else {
            games_in_state_count = new_state_counts;
        }
        player = (player + 1) % 2;
    }
    *wins.iter().max().unwrap()
}

fn main() {
    println!("Part 1: {}", part1(6, 9));
    println!("Part 2: {}", part2(6, 9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_example1() {
        assert_eq!(739785, part1(4, 8));
    }

    #[test]
    fn day21_example2() {
        assert_eq!(444356092776315, part2(4, 8));
    }
}
