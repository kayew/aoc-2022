#[derive(Clone)]
pub struct CrateMover {
    crates: Vec<Vec<char>>,
    instructions: Vec<usize>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> CrateMover {
    /*
                        [Q]     [P] [P]
                    [G] [V] [S] [Z] [F]
                [W] [V] [F] [Z] [W] [Q]
            [V] [T] [N] [J] [W] [B] [W]
        [Z] [L] [V] [B] [C] [R] [N] [M]
    [C] [W] [R] [H] [H] [P] [T] [M] [B]
    [Q] [Q] [M] [Z] [Z] [N] [G] [G] [J]
    [B] [R] [B] [C] [D] [H] [D] [C] [N]
    1   2   3   4   5   6   7   8   9
    */

    let crates = vec![
        vec!['B', 'Q', 'C'],
        vec!['R', 'Q', 'W', 'Z'],
        vec!['B', 'M', 'R', 'L', 'V'],
        vec!['C', 'Z', 'H', 'V', 'T', 'W'],
        vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G'],
        vec!['H', 'N', 'P', 'C', 'J', 'F', 'V', 'Q'],
        vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S'],
        vec!['C', 'G', 'M', 'N', 'B', 'W', 'Z', 'P'],
        vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P'],
    ];

    let data: Vec<&str> = input.split("\n\n").collect();
    let inst: Vec<usize> = data[1]
        .split(|x| x == ' ' || x == '\n')
        .filter_map(|x| x.parse().ok())
        .collect();

    CrateMover {
        crates: crates,
        instructions: inst,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &CrateMover) -> String {
    let mut result = String::new();
    let instructions = input.instructions.clone();
    let mut crates = input.crates.clone();

    for inst in instructions.chunks(3) {
        let count = inst[0];
        let from_crate = inst[1] - 1;
        let to_crate = inst[2] - 1;

        for _ in 0..count {
            let popped = crates[from_crate].pop().unwrap();
            crates[to_crate].push(popped);
        }
    }

    for c in crates {
        result.push(*c.last().unwrap())
    }

    result
}

#[aoc(day5, part2)]
pub fn part2(input: &CrateMover) -> String {
    let mut result = String::new();
    let instructions = input.instructions.clone();
    let mut crates = input.crates.clone();

    for inst in instructions.chunks(3) {
        let count = inst[0];
        let from_crate = inst[1] - 1;
        let to_crate = inst[2] - 1;

        let mut temp: Vec<char> = vec![];

        for _ in 0..count {
            let popped = crates[from_crate].pop().unwrap();
            temp.push(popped);
        }

        temp.reverse();
        crates[to_crate].append(&mut temp);
    }

    for c in crates {
        result.push(*c.last().unwrap())
    }

    result
}
