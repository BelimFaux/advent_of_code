use std::cmp::{max, min};

use aoc::{
    common::{Day, file},
    run_part,
};

const DAY: Day = 5;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, &input, 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

#[derive(Clone, Copy, Debug)]
struct Range(u64, u64);

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, available) = input.split_once("\n\n").unwrap();
    (
        ranges
            .lines()
            .map(|s| {
                let (l, u) = s.split_once('-').unwrap();
                Range(l.parse().unwrap(), u.parse().unwrap())
            })
            .collect(),
        available
            .lines()
            .map(|s| s.parse::<u64>().unwrap())
            .collect(),
    )
}

fn in_any_range(ranges: &[Range], ingredient: u64) -> bool {
    ranges
        .iter()
        .any(|r| ingredient >= r.0 && ingredient <= r.1)
}

fn part_one(input: &str) -> Option<usize> {
    let (ranges, available) = parse(input);

    Some(
        available
            .iter()
            .filter(|ingredient| in_any_range(&ranges, **ingredient))
            .count(),
    )
}

// fold ranges into non-overlapping ranges by merging subsequent ranges if they overlap
fn fold_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by(|rhs, lhs| rhs.0.cmp(&lhs.0));
    let mut ret = Vec::<Range>::new();
    for range in ranges {
        let last = ret.pop();
        if let Some(lr) = last {
            if range.0 <= lr.1 {
                let range = Range(min(range.0, lr.0), max(range.1, lr.1));
                ret.push(range);
            } else {
                ret.push(lr);
                ret.push(range);
            }
        } else {
            ret.push(range);
        }
    }
    ret
}

fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse(input);
    let folded = fold_ranges(ranges);
    Some(folded.iter().map(|range| range.1 - range.0 + 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(14));
    }
}
