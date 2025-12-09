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

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Line {
        Line { p1, p2 }
    }
}

struct Rectangle {
    min: Point,
    max: Point,
}

impl Rectangle {
    fn new(c1: Point, c2: Point) -> Rectangle {
        let min = Point::new(c1.x().min(c2.x()), c1.y().min(c2.y()));
        let max = Point::new(c1.x().max(c2.x()), c1.y().max(c2.y()));
        Rectangle { min, max }
    }

    fn intersects(&self, line: &Line) -> bool {
        let lxmin = line.p1.x().min(line.p2.x());
        let lxmax = line.p1.x().max(line.p2.x());
        let lymin = line.p1.y().min(line.p2.y());
        let lymax = line.p1.y().max(line.p2.y());

        // check if the line is strictly within the rectangle
        // check different component based on line orientation
        if lxmin == lxmax {
            let lx = lxmin;
            (self.min.x() < lx && lx < self.max.x())
                && ((lymin <= self.min.y() && lymax > self.min.y())
                    || (lymin < self.max.y() && lymax >= self.max.y()))
        } else {
            let ly = lymin;
            (self.min.y() < ly && ly < self.max.y())
                && ((lxmin <= self.min.x() && lxmax > self.min.x())
                    || (lxmin < self.max.x() && lxmax >= self.max.x()))
        }
    }
}

fn part_two(input: &str) -> Option<usize> {
    let points = parse(input);
    let mut lines: Vec<_> = points
        .windows(2)
        .map(|ps| Line::new(ps[0], ps[1]))
        .collect();
    lines.push(Line::new(points[0], points[points.len() - 1]));

    let mut max = 0;
    for p in points.iter() {
        for q in points.iter() {
            let d = p.area(q);
            if d > max {
                let r = Rectangle::new(*p, *q);

                if lines.iter().any(|l| r.intersects(l)) {
                    continue;
                }

                max = d;
            }
        }
    }

    Some(max)
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
