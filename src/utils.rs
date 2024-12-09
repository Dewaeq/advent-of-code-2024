#![allow(unused)]

use std::{
    fmt::Display,
    ops::{Add, Sub},
};

pub type Grid<T> = Vec<Vec<T>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub const fn zero() -> Self {
        Point(0, 0)
    }

    pub const fn orth_dirs() -> [Point; 4] {
        [Point(0, -1), Point(1, 0), Point(0, 1), Point(-1, 0)]
    }

    pub fn dist_squared(self, b: Point) -> i32 {
        (self.0 - b.0).pow(2) + (self.1 - b.1).pow(2)
    }

    pub fn slope(self, b: Point) -> f32 {
        (self.1 - b.1) as f32 / (self.0 - b.0) as f32
    }

    pub fn colinear(self, b: Point, c: Point) -> bool {
        let slope1 = self.slope(b);
        let slope2 = self.slope(c);
        let slope3 = c.slope(b);

        slope1 == slope2 && slope1 == slope3 && slope2 == slope3
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Point;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn read_grid<T>(input: &str, mut parse: impl FnMut(Point, char) -> T) -> Grid<T> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| parse(Point(x as _, y as _), c))
                .collect()
        })
        .collect()
}

pub fn print_grid(grid: &Grid<impl Display>) {
    for line in grid {
        for val in line {
            print!("{val}");
        }
        println!();
    }

    println!();
}

pub fn get<T: Clone>(grid: &Grid<T>, pos: Point) -> Option<T> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }

    grid.get(pos.1 as usize)
        .map(|row| row.get(pos.0 as usize).cloned())
        .flatten()
}

pub fn get_mut<T: Clone>(grid: &mut Grid<T>, pos: Point) -> Option<&mut T> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }

    grid.get_mut(pos.1 as usize)
        .map(|row| row.get_mut(pos.0 as usize))
        .flatten()
}

pub fn set<T: Clone>(grid: &mut Grid<T>, pos: Point, val: T) {
    if pos.0 < 0 || pos.1 < 0 {
        return;
    }

    grid.get_mut(pos.1 as usize)
        .map(|x| x.get_mut(pos.0 as usize).map(|x| *x = val));
}

pub fn enumerate<T>(grid: &Grid<T>) -> impl Iterator<Item = (Point, &T)> {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, val)| (Point(x as i32, y as i32), val))
        })
        .flatten()
}

pub fn enumerate_mut<T>(grid: &mut Grid<T>) -> impl Iterator<Item = (Point, &mut T)> {
    grid.iter_mut()
        .enumerate()
        .map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, val)| (Point(x as i32, y as i32), val))
        })
        .flatten()
}
