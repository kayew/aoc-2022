pub struct Elf {
    min: usize,
    max: usize,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<[Elf; 2]> {
    let mut data: Vec<[Elf; 2]> = vec![];

    for line in input.lines() {
        let val_iter: Vec<usize> = line
            .split(|x| x == ',' || x == '-')
            .map(|x| x.parse().unwrap())
            .collect();
        data.push([
            Elf {
                min: val_iter[0],
                max: val_iter[1],
            },
            Elf {
                min: val_iter[2],
                max: val_iter[3],
            },
        ]);
    }

    data
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<[Elf; 2]>) -> usize {
    // let mut total = 0;

    // for [elf1, elf2] in input {
    //     let case1 = (elf1.min >= elf2.min) && (elf1.max <= elf2.max);
    //     let case2 = (elf2.min >= elf1.min) && (elf2.max <= elf1.max);

    //     if case1 || case2 {
    //         total += 1;
    //     }
    // }

    // total

    input
        .iter()
        .filter(|[elf1, elf2]| {
            let case1 = (elf1.min >= elf2.min) && (elf1.max <= elf2.max);
            let case2 = (elf2.min >= elf1.min) && (elf2.max <= elf1.max);
            case1 || case2
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<[Elf; 2]>) -> usize {
    let mut total = 0;

    for [elf1, elf2] in input {
        if elf1.min <= elf2.max && elf2.min <= elf1.max {
            total += 1
        }
    }

    total
}
