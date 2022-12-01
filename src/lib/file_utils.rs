use std::{fs, path::Path};

pub fn read_file_to_string_lines(filepath: &Path) -> Vec<String> {
    let data_file = fs::read_to_string(filepath).expect("Failed to read file");
    let lines: Vec<String> = data_file
        .trim()
        .split("\n")
        .map(|line| String::from(line))
        .collect();

    lines
}

pub fn read_file_to_blocks_of_string_lines(filepath: &Path) -> Vec<Vec<String>> {
    let lines = read_file_to_string_lines(filepath);
    let blocks = lines
        .split(|l| l.len() == 0)
        .map(|b| b.to_owned())
        .collect();

    blocks
}
