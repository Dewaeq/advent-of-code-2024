use aoc_runner_derive::aoc;

use crate::utils::{print_grid, set, Point};

fn parse(line: &str) -> (Point, Point) {
    let (p_parts, v_parts) = line.split_once(" ").unwrap();
    let (px, py) = p_parts[2..].split_once(",").unwrap();
    let (vx, vy) = v_parts[2..].split_once(",").unwrap();

    let p = Point(px.parse().unwrap(), py.parse().unwrap());
    let v = Point(vx.parse().unwrap(), vy.parse().unwrap());
    (p, v)
}

fn sim(n: i32, p: Point, v: Point, nrows: i32, ncols: i32) -> Point {
    let res = p + n * v;

    Point(res.0.rem_euclid(ncols), res.1.rem_euclid(nrows))
}

#[aoc(day14, part1)]
fn part1(input: &str) -> i32 {
    let (nrows, ncols) = if input.lines().count() == 12 {
        (7, 11)
    } else {
        (103, 101)
    };

    let mut counts = [0; 4];

    for line in input.lines() {
        let (p, v) = parse(line);
        let new_pos = sim(100, p, v, nrows, ncols);

        let (middle_row, middle_col) = (nrows / 2, ncols / 2);
        if new_pos.0 == middle_col || new_pos.1 == middle_row {
            continue;
        }

        let is_top = new_pos.1 < middle_row;
        let is_left = new_pos.0 < middle_col;
        counts[is_top as usize + (is_left as usize) * 2] += 1;
    }

    counts.iter().product()
}

fn density(positions: &Vec<Point>) -> f32 {
    let mut dist = 0;
    for pos1 in positions {
        for pos2 in positions {
            dist += pos1.dist_squared(*pos2);
        }
    }

    1. / dist as f32
}

#[aoc(day14, part2)]
fn part2(input: &str) -> i32 {
    let (nrows, ncols) = (103, 101);
    let robots = input.lines().map(|line| parse(line)).collect::<Vec<_>>();

    let mut positions = vec![];
    let mut seconds = 1;
    let mut highest_density = 0.;
    let mut best = -1;

    loop {
        positions.clear();

        for (p, v) in &robots {
            let new_pos = sim(seconds, *p, *v, nrows, ncols);
            positions.push(new_pos);
        }
        let current_density = density(&positions);
        if current_density > highest_density {
            highest_density = current_density;
            best = seconds;
        }

        seconds += 1;
        if seconds > 100_000 {
            let mut grid = vec![vec!['.'; ncols as usize]; nrows as usize];
            for (p, v) in &robots {
                let new_pos = sim(best, *p, *v, nrows, ncols);
                set(&mut grid, new_pos, '#');
            }
            print_grid(&grid);

            return best;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST), 12);
    }
}
