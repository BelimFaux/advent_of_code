#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point(usize, usize);

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point(x, y)
    }

    pub fn parse(line: &str) -> Option<Point> {
        let mut nums = line.split(',');
        Some(Point(
            nums.next()?.parse().unwrap(),
            nums.next()?.parse().unwrap(),
        ))
    }

    pub fn area(&self, lhs: &Point) -> usize {
        (self.0.abs_diff(lhs.0) + 1) * (self.1.abs_diff(lhs.1) + 1)
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Point> {
        NEIGHBOR_OFFSETS.iter().filter_map(|(dx, dy)| {
            if (self.0 as isize) < -dx || (self.1 as isize) < -dy {
                return None;
            }
            Some(Point(
                (self.0 as isize + dx) as usize,
                (self.1 as isize + dy) as usize,
            ))
        })
    }

    pub fn inc_x(&mut self) {
        self.0 += 1;
    }

    pub fn inc_y(&mut self) {
        self.1 += 1;
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }
}

macro_rules! tuple_add {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Add<$rhs> for $lhs {
            type Output = Point;
            fn add(self, rhs: $rhs) -> Point {
                Point(self.0 + rhs.0, self.1 + rhs.1)
            }
        }
    };
}

tuple_add!(Point, Point);
tuple_add!(Point, &Point);
tuple_add!(&Point, Point);
tuple_add!(&Point, &Point);

macro_rules! tuple_sub {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = Point;
            fn sub(self, rhs: $rhs) -> Point {
                Point(self.0 - rhs.0, self.1 - rhs.1)
            }
        }
    };
}

tuple_sub!(Point, Point);
tuple_sub!(Point, &Point);
tuple_sub!(&Point, Point);
tuple_sub!(&Point, &Point);
