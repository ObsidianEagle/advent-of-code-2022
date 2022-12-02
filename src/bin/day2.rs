use std::path::Path;

use aoc_lib::file_utils::read_file_to_string_lines;

pub fn part1() -> u32 {
    let lines = read_file_to_string_lines(Path::new("./data/day2.txt"));

    let mut total_score: u32 = 0;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let their_move = chars[0];
        let your_move = chars[2];

        let your_move_score: u32 = match your_move {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Encountered unexpected move")
        };

        let round_score: u32 = match (their_move, your_move) {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ => 3
        };

        total_score += your_move_score + round_score;
    }

    total_score
}

pub fn part2() -> u32 {
    0
}

pub fn main() {
    println!("Part 1 result: {}", part1());
    println!("Part 2 result: {}", part2());
}
