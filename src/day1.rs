#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let data: Vec<&str> = input.split("\n\n").collect();
    let mut max = vec![];

    // println!("{:?}", data);
    
    for elf in data {
        let current_elf: Vec<&str> = elf.split("\n").collect();
        let mut total_elf_cal = 0;
        for cal in current_elf {
            total_elf_cal += cal.parse::<i32>().unwrap();
        }
        max.push(total_elf_cal);
    }

    max.sort();

    return max[max.len()-1];
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let data: Vec<&str> = input.split("\n\n").collect();
    let mut max = vec![];
    
    for elf in data {
        let current_elf: Vec<&str> = elf.split("\n").collect();
        let mut total_elf_cal = 0;
        for cal in current_elf {
            total_elf_cal += cal.parse::<i32>().unwrap();
        }
        max.push(total_elf_cal);
    }

    max.sort();

    return max.iter().rev().take(3).sum();
}