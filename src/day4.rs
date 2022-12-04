pub struct Elf {
    min: usize,
    max: usize,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<[Elf; 2]> {
    let mut data: Vec<[Elf; 2]> = vec![];

    for line in input.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        let range1: Vec<usize> = ranges[0].split("-").map(|r| r.parse().unwrap()).collect();
        let range2: Vec<usize> = ranges[1].split("-").map(|r| r.parse().unwrap()).collect();
        data.push([Elf { min: range1[0], max: range1[1] }, Elf { min: range2[0], max: range2[1]}]);
    }

    data
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<[Elf; 2]>) -> usize {
    let mut total = 0;

    for [elf1, elf2] in input {
        let case1 = (elf1.min >= elf2.min) && (elf1.max <= elf2.max);
        let case2 = (elf2.min >= elf1.min) && (elf2.max <= elf1.max);

        if case1 || case2 {
            total += 1;
        }
    }

    total
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<[Elf; 2]>) -> usize {
    let mut total = 0;


    for [elf1, elf2] in input {
        let case1 = (elf2.min <= elf1.max) && (elf1.max <= elf2.max);
        let case2 = (elf1.min <= elf2.max) && (elf2.max <= elf1.max);
        if case1 || case2 {
            total += 1
        }
    }

    total
}
