use aoc_runner_derive::aoc;

use crate::utils::{get, get_mut, read_grid, set, Point};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    // keep track of the previous direction
    Visited(u8),
    Obstacle,
    Unvisited,
}

impl Cell {
    pub fn add_dir(&mut self, dir: (i32, i32)) {
        if let Cell::Visited(mask) = self {
            *mask |= dir_mask(dir);
        } else {
            *self = Cell::Visited(dir_mask(dir));
        }
    }
}

fn next_dir(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

fn dir_mask(dir: (i32, i32)) -> u8 {
    match dir {
        (0, -1) => 1,
        (1, 0) => 2,
        (0, 1) => 4,
        (-1, 0) => 8,
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> (Point, Vec<Vec<Cell>>) {
    let mut start_pos = Point::zero();
    let grid = read_grid(input, |pos, c| {
        if c == '^' {
            start_pos = pos;
        }
        match c {
            '#' => Cell::Obstacle,
            '^' => Cell::Visited(dir_mask((0, -1))),
            _ => Cell::Unvisited,
        }
    });

    (start_pos, grid)
}

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let (mut pos, mut grid) = parse_input(input);
    let mut dir = (0, -1);

    let mut count = 1;

    while let Some(_) = get(&grid, pos) {
        if get(&grid, pos + dir).is_some_and(|in_front| in_front == Cell::Obstacle) {
            dir = next_dir(dir);
            continue;
        }
        count += (get(&grid, pos) == Some(Cell::Unvisited)) as i32;
        set(&mut grid, pos, Cell::Visited(0));
        pos = pos + dir;
    }

    count
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    let (mut pos, mut grid) = parse_input(input);
    let mut dir = (0, -1);
    let mut count = 0;

    while let Some(next_cell) = get(&grid, pos + dir) {
        let next_pos = pos + dir;

        if next_cell == Cell::Obstacle {
            dir = next_dir(dir);
            get_mut(&mut grid, pos).unwrap().add_dir(dir);
            continue;
        }

        if next_cell == Cell::Unvisited {
            let mut alt_grid = grid.clone();
            set(&mut alt_grid, next_pos, Cell::Obstacle);
            let mut alt_pos = pos;
            let mut alt_dir = next_dir(dir);

            while let Some(next_cell) = get(&alt_grid, alt_pos + alt_dir) {
                if next_cell == Cell::Obstacle {
                    alt_dir = next_dir(alt_dir);
                    get_mut(&mut alt_grid, alt_pos).unwrap().add_dir(alt_dir);
                    continue;
                }

                if let Cell::Visited(mask) = next_cell {
                    if mask & dir_mask(alt_dir) != 0 {
                        count += 1;
                        break;
                    }
                }
                alt_pos = alt_pos + alt_dir;
                get_mut(&mut alt_grid, alt_pos).unwrap().add_dir(alt_dir);
            }
        }

        pos = pos + dir;
        get_mut(&mut grid, pos).unwrap().add_dir(dir);
    }

    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            41
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            6
        );
    }
}
