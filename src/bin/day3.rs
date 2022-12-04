use std::path::Path;

use aoc_lib::file_utils::read_file_to_string_lines;

fn get_item_priority(c: &char) -> u32 {
    let ascii_value = *c as u32;
    match ascii_value {
        65..=90 => ascii_value - 38,
        97..=122 => ascii_value - 96,
        _ => panic!("Unexpected char encountered"),
    }
}

pub fn part1() -> u32 {
    let lines = read_file_to_string_lines(Path::new("./data/day3.txt"));

    let bag_duplicates: Vec<char> = lines
        .iter()
        .map(|line| {
            let (c1, c2) = line.split_at(line.len() / 2);

            for c in c1.chars() {
                if c2.contains(&c.to_string()) {
                    return c;
                }
            }

            panic!("Didn't find a duplicate in one of the bags")
        })
        .collect();

    bag_duplicates.iter().map(get_item_priority).sum()
}

pub fn part2() -> u32 {
    let lines = read_file_to_string_lines(Path::new("./data/day3.txt"));
    let groups: Vec<&[String]> = lines.chunks(3).collect();

    let group_badges = groups.iter().map(|group| {
        for c in group[0].chars() {
            if group[1].contains(&c.to_string()) & group[2].contains(&c.to_string()) {
                return c;
            }
        }

        panic!("No common item found in group");
    });

    let group_badge_priority_sum = group_badges.fold(0, |acc, cur| acc + get_item_priority(&cur));

    group_badge_priority_sum
}

pub fn main() {
    println!("Part 1 result: {}", part1());
    println!("Part 2 result: {}", part2());
}
