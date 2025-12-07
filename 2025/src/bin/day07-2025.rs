use std::collections::{HashMap, HashSet};

use aoc::{
    common::{
        Day, file,
        util::{grid::Grid, point::Point},
    },
    run_part,
};

const DAY: Day = 7;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, &input, 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

fn parse(input: &str) -> Grid {
    Grid::from(input)
}

fn try_hit(grid: &Grid, start: Point) -> Option<Point> {
    let mut point = start;
    loop {
        match grid.c_at(&point) {
            Some('^') => return Some(point),
            None => return None,
            _ => point = Point::new(point.x(), point.y() + 1),
        }
    }
}

fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let mut frontier = Vec::<Point>::new();
    let mut hits = HashSet::<Point>::new();
    let start = grid.find('S').unwrap();
    frontier.push(start);

    if let Some(hit) = try_hit(&grid, start)
        && !hits.contains(&hit)
    {
        frontier.push(hit);
        hits.insert(hit);
    }
    while let Some(split) = frontier.pop() {
        if split.x() != 0 {
            let left = Point::new(split.x() - 1, split.y());
            if let Some(hit) = try_hit(&grid, left)
                && !hits.contains(&hit)
            {
                frontier.push(hit);
                hits.insert(hit);
            }
        }
        if split.x() != grid.width() {
            let right = Point::new(split.x() + 1, split.y());
            if let Some(hit) = try_hit(&grid, right)
                && !hits.contains(&hit)
            {
                frontier.push(hit);
                hits.insert(hit);
            }
        }
    }

    Some(hits.len())
}

// memoization
struct Timelines<'a> {
    memory: HashMap<Point, usize>,
    grid: &'a Grid,
}

impl<'a> Timelines<'a> {
    pub fn new(grid: &'a Grid) -> Timelines<'a> {
        Timelines {
            memory: HashMap::new(),
            grid,
        }
    }

    pub fn get_timelines(&mut self, start: Point) -> usize {
        if let Some(t) = self.memory.get(&start) {
            return *t;
        }
        if let Some(hit) = try_hit(self.grid, start) {
            let left = if hit.x() != 0 {
                let left = Point::new(hit.x() - 1, hit.y());
                self.get_timelines(left)
            } else {
                0
            };
            let right = if hit.x() != self.grid.width() {
                let right = Point::new(hit.x() + 1, hit.y());
                self.get_timelines(right)
            } else {
                0
            };

            self.memory.insert(start, left + right);
            left + right
        } else {
            self.memory.insert(start, 1);
            1
        }
    }
}

fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let start = grid.find('S').unwrap();
    let mut timelines = Timelines::new(&grid);

    Some(timelines.get_timelines(start))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(40));
    }
}
