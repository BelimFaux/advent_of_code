use std::collections::{BTreeMap, HashSet};

use aoc::{
    common::{Day, file},
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

fn get_distances(points: &[Point3]) -> BTreeMap<i64, (Point3, Point3)> {
    let mut distances = BTreeMap::new();
    for p in points.iter() {
        for q in points.iter() {
            if p == q {
                continue;
            }
            distances.insert(p.sq_distance(q), (*p, *q));
        }
    }
    distances
}

fn make_connection(p: Point3, q: Point3, buckets: &mut Vec<HashSet<Point3>>) {
    let mut contained: Vec<_> = buckets
        .iter_mut()
        .filter(|b| b.iter().any(|e| e == &p || e == &q))
        .collect();

    if contained.is_empty() {
        buckets.push(HashSet::from([p, q]));
    } else if contained.len() == 1 {
        contained[0].insert(p);
        contained[0].insert(q);
    } else {
        let mut bucket: HashSet<Point3> = HashSet::from_iter(
            contained
                .iter()
                .flat_map(|s| s.iter().copied().collect::<Vec<Point3>>()),
        );
        bucket.insert(p);
        bucket.insert(q);
        buckets.retain(|b| !b.iter().any(|e| e == &p || e == &q));
        buckets.push(bucket);
    }
}

fn part_one(input: (&str, u32)) -> Option<usize> {
    let (input, connections) = input;
    let points = parse(input);

    let mut distances = get_distances(&points);
    let mut buckets: Vec<HashSet<Point3>> = Vec::new();
    for _ in 0..connections {
        let (_, (p, q)) = distances.pop_first().unwrap();
        make_connection(p, q, &mut buckets);
    }

    let mut counts: Vec<_> = buckets.iter().map(|b| b.len()).collect();
    counts.sort();
    let val: usize = counts.iter().rev().take(3).product();
    Some(val)
}

fn part_two(input: &str) -> Option<i64> {
    let points = parse(input);

    let mut distances = get_distances(&points);
    let mut buckets: Vec<HashSet<Point3>> = Vec::new();
    loop {
        let (_, (p, q)) = distances.pop_first().unwrap();
        make_connection(p, q, &mut buckets);
        if buckets[0].len() == points.len() {
            return Some(p.0 * q.0);
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
