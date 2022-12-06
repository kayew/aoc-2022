use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn push_to_set(arr: &[char]) -> bool {
    let mut hs = HashSet::new();
    arr.iter().all(move |x| hs.insert(x))
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<char>) -> usize {
    for i in 0..input.len() {
        if push_to_set(&input[i..(i+4)]) {
            return i + 4
        }
    }

    0
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<char>) -> usize {
    for i in 0..input.len() {
        if push_to_set(&input[i..(i+14)]) {
            return i + 14
        }
    }

    0
}
