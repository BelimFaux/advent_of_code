use aoc::{
    common::{Day, file, util::point::Point},
    run_part,
};

const DAY: Day = 9;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, &input, 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

fn parse(input: &str) -> Vec<Point> {
    input.lines().map(|l| Point::parse(l).unwrap()).collect()
}

fn part_one(input: &str) -> Option<usize> {
    let points = parse(input);

    let mut max = 0;
    for p in points.iter() {
        for q in points.iter() {
            let d = p.area(q);
            if d > max {
                max = d;
            }
        }
    }

    Some(max)
}

fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(24));
    }
}
