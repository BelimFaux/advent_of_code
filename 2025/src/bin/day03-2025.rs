use aoc::{
    common::{Day, file},
    run_part,
};

const DAY: Day = 3;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, &input, 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

fn get_joltage(f: char, l: char) -> u32 {
    f.to_digit(10).unwrap() * 10 + l.to_digit(10).unwrap()
}

fn turn_on_2(bank: &str) -> u32 {
    let f = bank[..bank.len() - 1].chars().max().unwrap();
    let i = bank.find(f).unwrap();
    let l = bank[i + 1..].chars().max().unwrap();
    get_joltage(f, l)
}

fn turn_on_12(bank: &str) -> u64 {
    let mut num = String::with_capacity(12);
    let mut i = 0;
    for r in (0..12).rev() {
        let substr = &bank[i..bank.len() - r];
        let c = substr.chars().max().unwrap();
        i += substr.find(c).unwrap() + 1;
        num.push(c);
    }
    num.parse().unwrap()
}

fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(turn_on_2).sum())
}

fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(turn_on_12).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
