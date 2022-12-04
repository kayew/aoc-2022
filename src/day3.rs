use std::collections::HashSet;

fn determine_priority(item: char) -> i32 {
    if item.is_lowercase() {
        (item as i32) - ('a' as i32) + 1
    } else {
        (item as i32) - ('A' as i32) + 27
    }
}

#[aoc_generator(day3, part1)]
pub fn input_generator_d3p1(input: &str) -> Vec<[HashSet<char>; 2]> {
    input
        .lines()
        .map(|line| {
            let split = line.split_at(line.len() / 2);
            let left: HashSet<char> = split.0.chars().collect();
            let right: HashSet<char> = split.1.chars().collect();
            [left, right]
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<[HashSet<char>; 2]>) -> i32 {
    //     let mut total = 0;

    //     for [left, right] in input {
    //         let intersect: Vec<char> = left.intersection(right).copied().collect();
    //         for c in intersect {
    //             total += determine_priority(c);
    //         }
    //     }

    //     total

    input
        .iter()
        .map(|[left, right]| {
            left.intersection(right)
                .fold(0, |acc, &c| acc + determine_priority(c))
        })
        .sum()
}

#[aoc_generator(day3, part2)]
pub fn input_generator_d3p2(input: &str) -> Vec<HashSet<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .collect()
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<HashSet<char>>) -> i32 {
    // let mut total = 0;

    // for elfs in input.chunks(3) {
    //     let i1: HashSet<char> = elfs[0]
    //         .intersection(&elfs[1])
    //         .copied()
    //         .collect::<HashSet<char>>();
    //     let i2: Vec<char> = i1.intersection(&elfs[2]).copied().collect();

    //     for c in i2 {
    //         total += determine_priority(c);
    //     }
    // }

    // total

    input
        .chunks(3)
        .map(|elfs| {
            let i1: HashSet<char> = elfs[0].intersection(&elfs[1]).copied().collect();
            i1.intersection(&elfs[2])
                .fold(0, |acc, &c| acc + determine_priority(c))
        })
        .sum()
}
