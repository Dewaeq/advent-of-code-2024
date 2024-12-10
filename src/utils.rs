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

pub fn read_grid<T>(input: &str, parse: fn(char) -> T) -> Grid<T> {
    input
        .lines()
        .map(|line| line.chars().map(|c| parse(c)).collect())
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

pub fn dist_squared(a: Point, b: Point) -> i32 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

pub fn slope(a: Point, b: Point) -> f32 {
    (a.1 - b.1) as f32 / (a.0 - b.0) as f32
}

pub fn colinear(a: Point, b: Point, c: Point) -> bool {
    let slope1 = slope(a, b);
    let slope2 = slope(a, c);
    let slope3 = slope(c, b);

    slope1 == slope2 && slope1 == slope3 && slope2 == slope3
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
