use aoc::{
    common::{Day, file, util::grid::Grid},
    run_part,
};

const DAY: Day = 4;

fn main() {
    let input = file::read_file_part(DAY);
    println!("---- Day {DAY} ----");
    run_part(part_one, &input, 1);
    run_part(part_two, &input, 2);
    println!("------ {DAY} ------");
}

fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from(input);
    let mut counter = 0;

    for (p, c) in grid.get_elems() {
        if c != '@' {
            continue;
        }
        let neighbor_rolls = p
            .neighbors()
            .filter_map(|n| if grid.c_at(&n)? == '@' { Some(1) } else { None })
            .count();
        if neighbor_rolls < 4 {
            counter += 1;
        }
    }

    Some(counter)
}

fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from(input);
    let mut total_counter = 0;

    // bruteforce...
    loop {
        let mut counter = 0;
        let mut changed = Vec::new();

        for (p, c) in grid.get_elems() {
            if c != '@' {
                continue;
            }
            let neighbor_rolls = p
                .neighbors()
                .filter_map(|n| if grid.c_at(&n)? == '@' { Some(1) } else { None })
                .count();
            if neighbor_rolls < 4 {
                changed.push(p);
                counter += 1;
            }
        }
        changed.iter().for_each(|p| {
            grid.try_set('.', p);
        });
        total_counter += counter;
        if counter == 0 {
            break;
        }
    }

    Some(total_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&file::read_test_file_part(DAY));
        assert_eq!(result, Some(43));
    }
}
