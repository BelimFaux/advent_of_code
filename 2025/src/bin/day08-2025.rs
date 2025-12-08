use std::collections::BTreeMap;

use aoc::{
    common::{Day, file, util::unionfind::UnionFind},
    run_part,
};

const DAY: Day = 8;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, (&input, 1000), 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point3(i64, i64, i64);

impl Point3 {
    fn parse(line: &str) -> Option<Point3> {
        let mut nums = line.split(',');
        Some(Point3(
            nums.next()?.parse().unwrap(),
            nums.next()?.parse().unwrap(),
            nums.next()?.parse().unwrap(),
        ))
    }

    fn sq_distance(&self, lhs: &Point3) -> i64 {
        (self.0 - lhs.0) * (self.0 - lhs.0)
            + (self.1 - lhs.1) * (self.1 - lhs.1)
            + (self.2 - lhs.2) * (self.2 - lhs.2)
    }
}

fn parse(input: &str) -> Vec<Point3> {
    input.lines().map(|l| Point3::parse(l).unwrap()).collect()
}

fn get_distances(points: &[Point3]) -> BTreeMap<i64, (usize, usize)> {
    let mut distances = BTreeMap::new();
    for (pi, p) in points.iter().enumerate() {
        for (qi, q) in points.iter().enumerate() {
            if p == q {
                continue;
            }
            distances.insert(p.sq_distance(q), (pi, qi));
        }
    }
    distances
}

fn part_one(input: (&str, u32)) -> Option<usize> {
    let (input, connections) = input;
    let points = parse(input);

    let mut distances = get_distances(&points);
    let mut uf = UnionFind::new(points.len());
    for _ in 0..connections {
        let (_, (p, q)) = distances.pop_first().unwrap();
        uf.union(p, q);
    }

    let mut counts = uf.sizes();
    counts.sort();
    let val: usize = counts.iter().rev().take(3).product();
    Some(val)
}

fn part_two(input: &str) -> Option<i64> {
    let points = parse(input);

    let mut distances = get_distances(&points);
    let mut uf = UnionFind::new(points.len());
    loop {
        let (_, (p, q)) = distances.pop_first().unwrap();
        uf.union(p, q);
        if uf.size_of(0) == points.len() {
            return Some(points[p].0 * points[q].0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one((&file::read_test_file_part(DAY), 10));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(25272));
    }
}
