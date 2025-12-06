use aoc::{
    common::{Day, file},
    run_part,
};

const DAY: Day = 6;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, &input, 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

#[derive(Clone, Debug)]
enum Operation {
    Addition,
    Multiplication,
}

impl Operation {
    fn from(op: &str) -> Operation {
        match op {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => Operation::Addition,
        }
    }

    fn neutral_elem(&self) -> u64 {
        match self {
            Operation::Addition => 0,
            Operation::Multiplication => 1,
        }
    }

    fn perform(&self, rhs: u64, lhs: u64) -> u64 {
        match self {
            Operation::Addition => rhs + lhs,
            Operation::Multiplication => rhs * lhs,
        }
    }
}

#[derive(Clone, Debug)]
struct Problem(Vec<u64>, Operation);

impl Problem {
    fn solve(&self) -> u64 {
        self.0.iter().fold(self.1.neutral_elem(), |acc, elem| {
            self.1.perform(acc, *elem)
        })
    }
}

fn parse_one(input: &str) -> Vec<Problem> {
    let mut ret = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    let last_line = lines[lines.len() - 1];
    for op in last_line.split_whitespace() {
        let problem = Problem(Vec::new(), Operation::from(op));
        ret.push(problem);
    }

    for line in &lines[..lines.len() - 1] {
        line.split_whitespace().enumerate().for_each(|(i, val)| {
            ret[i].0.push(val.parse().unwrap());
        });
    }

    ret
}

fn part_one(input: &str) -> Option<u64> {
    let problems = parse_one(input);

    Some(problems.iter().map(|problem| problem.solve()).sum())
}

fn parse_two(input: &str) -> Vec<Problem> {
    let mut ret = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    let last_line = lines[lines.len() - 1];

    let mut problem = Problem(Vec::new(), Operation::Addition);
    for i in 0..last_line.len() {
        if &last_line[i..i + 1] != " " {
            problem = Problem(Vec::new(), Operation::from(&last_line[i..i + 1]));
        } else if last_line.len() >= i + 2 && &last_line[i + 1..i + 2] != " " {
            ret.push(problem.clone());
            continue;
        }
        let number = &lines[..lines.len() - 1]
            .iter()
            .fold(String::new(), |acc, e| {
                if e.len() > i && &e[i..i + 1] != " " {
                    acc + &e[i..i + 1]
                } else {
                    acc
                }
            });
        problem.0.push(number.parse().unwrap());
    }
    ret.push(problem);

    ret
}

fn part_two(input: &str) -> Option<u64> {
    let problems = parse_two(input);

    Some(problems.iter().map(|problem| problem.solve()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(3263827));
    }
}
