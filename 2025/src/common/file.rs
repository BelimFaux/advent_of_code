use std::{fs, path::PathBuf};

use crate::common::{Day, YEAR};

fn read_file(folder: &str, filename: &str) -> String {
    let mut filepath = PathBuf::new();
    filepath.push(folder);
    filepath.push(filename);

    fs::read_to_string(filepath).expect("could not read input file")
}

#[must_use]
pub fn read_file_part(day: Day) -> String {
    const FOLDER: &str = "./data/input";
    let filename = format!("day{:0>2}-{}.txt", day, YEAR);
    read_file(FOLDER, &filename)
}

#[must_use]
pub fn read_test_file_part(day: Day) -> String {
    const FOLDER: &str = "./data/examples";
    let filename = format!("day{:0>2}-{}.txt", day, YEAR);
    read_file(FOLDER, &filename)
}
