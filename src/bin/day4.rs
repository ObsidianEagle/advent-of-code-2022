use std::path::Path;

use aoc_lib::file_utils::read_file_to_string_lines;

pub fn part1() -> u32 {
    let lines = read_file_to_string_lines(Path::new("./data/day4.txt"));

    lines
        .iter()
        .filter(|line| {
            let split: Vec<Vec<u32>> = line
                .split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect()
                })
                .collect();

            (split[0][0] <= split[1][0] && split[0][1] >= split[1][1])
                || (split[1][0] <= split[0][0] && split[1][1] >= split[0][1])
        })
        .collect::<Vec<&String>>()
        .len() as u32
}

pub fn part2() -> u32 {
    let lines = read_file_to_string_lines(Path::new("./data/day4.txt"));

    lines
        .iter()
        .filter(|line| {
            let split: Vec<Vec<u32>> = line
                .split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect()
                })
                .collect();

            !((split[0][1] < split[1][0]) || split[0][0] > split[1][1])
        })
        .collect::<Vec<&String>>()
        .len() as u32
}

pub fn main() {
    println!("Part 1 result: {}", part1());
    println!("Part 2 result: {}", part2());
}
