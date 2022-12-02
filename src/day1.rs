#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let data: Vec<&str> = input.split("\n\n").collect();
    let mut max: Vec<i32> = vec![];

    for elf in data {
        let current_elf: Vec<i32> = elf.split("\n").map(|cal| cal.parse().unwrap()).collect();
        max.push(current_elf.iter().sum());
    }

    max.sort_unstable();

    max
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
    input[input.len() - 1]
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
    input.iter().rev().take(3).sum()
}
