use std::path::Path;

use aoc_lib::file_utils::read_file_to_blocks_of_string_lines;

pub fn part1() -> u32 {
    let blocks = read_file_to_blocks_of_string_lines(Path::new("./data/day1.txt"));

    let largest_block_sum = blocks
        .iter()
        .map(|b| b.iter().map(|l| l.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap();

    largest_block_sum
}

pub fn part2() -> u32 {
    let blocks = read_file_to_blocks_of_string_lines(Path::new("./data/day1.txt"));

    let mut block_sums = blocks
        .iter()
        .map(|b| b.iter().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    block_sums.sort_by(|a, b| b.partial_cmp(a).unwrap());

    block_sums[0] + block_sums[1] + block_sums[2]
}

pub fn main() {
    println!("Part 1 result: {}", part1());
    println!("Part 2 result: {}", part2());
}
