use aoc::{
    common::{Day, file},
    run_part,
};

const DAY: Day = 2;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, &input, 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

struct Range(u64, u64);

fn parse(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|r| r.split_once('-'))
        .map(|o| o.expect("ranges should be in this format"))
        .map(|(f, l)| {
            (
                f.trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("'{f}' is not integer")),
                l.trim()
                    .parse()
                    .unwrap_or_else(|_| panic!("'{f}' is not integer")),
            )
        })
        .map(|(f, l)| Range(f, l))
        .collect()
}

fn is_invalid(id: &str) -> bool {
    if !id.len().is_multiple_of(2) {
        return false;
    }
    let e = id.len() / 2;
    let substr = &id[0..e];
    if substr.repeat(2) == id {
        return true;
    }

    false
}

fn part_one(input: &str) -> Option<u64> {
    let ranges = parse(input);
    let mut acc = 0;

    for Range(f, l) in ranges {
        for n in f..=l {
            let nstr = n.to_string();
            acc += if is_invalid(&nstr) { n } else { 0 };
        }
    }

    Some(acc)
}

fn is_invalid_at_least2(id: &str) -> bool {
    for e in 1..=(id.len() / 2) {
        let n = id.len() / e;
        if !id.len().is_multiple_of(n) {
            continue;
        }

        let substr = &id[0..e];
        if substr.repeat(n) == id {
            return true;
        }
    }
    false
}

fn part_two(input: &str) -> Option<u64> {
    let ranges = parse(input);
    let mut acc = 0;

    for Range(f, l) in ranges {
        for n in f..=l {
            let nstr = n.to_string();
            acc += if is_invalid_at_least2(&nstr) { n } else { 0 };
        }
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(4174379265));
    }
}
