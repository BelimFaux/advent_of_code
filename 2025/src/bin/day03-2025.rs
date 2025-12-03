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

fn turn_on(bank: &str, batteries: usize) -> u64 {
    let mut num = String::with_capacity(batteries);
    let mut i = 0;
    for r in (0..batteries).rev() {
        let substr = &bank[i..bank.len() - r];
        let c = substr.chars().max().unwrap();
        i += substr.find(c).unwrap() + 1;
        num.push(c);
    }
    num.parse().unwrap()
}

fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|bank| turn_on(bank, 2)).sum())
}

fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|bank| turn_on(bank, 12)).sum())
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
