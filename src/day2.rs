const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<[char; 2]> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            [bytes[0] as char, bytes[2] as char]
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<[char; 2]>) -> i32 {
    let mut total_score = 0;

    for [opp, you] in input {
        match opp {
            'A' => match you {
                    'Y' => total_score += PAPER + WIN,
                    'Z' => total_score += SCISSORS + LOSE,
                    _ => total_score += ROCK + DRAW, // x
            },
            'B' => match you {
                    'Z' => total_score += SCISSORS + WIN,
                    'X' => total_score += ROCK + LOSE,
                    _ => total_score += PAPER + DRAW, // y
            },
            _ =>
                match you {
                    'X' => total_score += ROCK + WIN,
                    'Y' => total_score += PAPER + LOSE,
                    _ => total_score += SCISSORS + DRAW, // z
            },
        }
    }

    total_score
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<[char; 2]>) -> i32 {
    let mut total_score = 0;

    for [opp, you] in input {
        match opp {
            'A' => match you {
                'X' => total_score += SCISSORS + LOSE,
                'Y' => total_score += ROCK + DRAW,
                _ => total_score += PAPER + WIN,
            }
            'B' => match you {
                'X' => total_score += ROCK + LOSE,
                'Y' => total_score += PAPER + DRAW,
                _ => total_score += SCISSORS + WIN,
            },
            _ => match you {
                'X' => total_score += PAPER + LOSE,
                'Y' => total_score += SCISSORS + DRAW,
                _ => total_score += ROCK + WIN,
            }
        }
    }


    return total_score;
}

