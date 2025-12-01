use aoc::{
    common::{Day, file},
    run_part,
};

const DAY: Day = 1;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, &input, 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

struct Command(char, u32);

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|s| s.split_at(1))
        .map(|(d, l)| Command(d.chars().next().unwrap(), l.parse().unwrap()))
        .collect()
}

fn part_one(input: &str) -> Option<u32> {
    let commands = parse(input);
    let mut dial = 50;
    let mut counter = 0;

    for Command(l, m) in commands {
        if l == 'L' {
            let m = m % 100;
            if m > dial {
                dial = 100 - (m - dial);
            } else {
                dial -= m;
            }
        } else {
            dial = (dial + m) % 100;
        }
        if dial == 0 {
            counter += 1;
        }
    }

    Some(counter)
}

fn part_two(input: &str) -> Option<u32> {
    let commands = parse(input);
    let mut dial = 50;
    let mut counter = 0;

    for Command(l, m) in commands {
        if l == 'L' {
            for _ in 0..m {
                if dial == 0 {
                    dial = 100;
                    counter += 1;
                }
                dial -= 1;
            }
        } else {
            for _ in 0..m {
                if dial == 0 {
                    counter += 1;
                }
                dial += 1;
                if dial == 100 {
                    dial = 0;
                }
            }
        }
    }

    Some(counter)
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
        assert_eq!(result, Some(6));
    }
}
