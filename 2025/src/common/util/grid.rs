use crate::common::util::point::Point;

pub struct Grid {
    elems: Vec<String>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn from(input: &str) -> Grid {
        let elems: Vec<_> = input.lines().map(|s| s.to_owned()).collect();
        let width = elems[0].len();
        let height = elems.len();
        Grid {
            elems,
            width,
            height,
        }
    }

    pub fn c_at(&self, point: &Point) -> Option<char> {
        self.elems.get(point.y())?.chars().nth(point.x())
    }

    pub fn try_set(&mut self, to: char, point: &Point) {
        if let Some(line) = self.elems.get_mut(point.y()) {
            line.replace_range(point.x()..point.x() + 1, &to.to_string());
        }
    }

    pub fn get_elems(&self) -> impl Iterator<Item = (Point, char)> {
        let mut x = 0;
        let mut y = 0;

        self.elems.iter().flat_map(|s| s.chars()).map(move |c| {
            let p = Point::new(x, y);
            if x < self.width - 1 {
                x += 1;
            } else {
                y += 1;
                x = 0;
            }

            (p, c)
        })
    }
}
