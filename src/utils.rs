#![allow(unused)]

use std::fmt::Display;

pub type Point = (i32, i32);

pub fn print_grid(grid: &Vec<Vec<impl Display>>) {
    for line in grid {
        for val in line {
            print!("{val}");
        }
        println!();
    }

    println!();
}

pub fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

pub fn sub(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
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

pub fn get<T: Clone>(grid: &Vec<Vec<T>>, pos: (i32, i32)) -> Option<T> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }

    grid.get(pos.1 as usize)
        .map(|row| row.get(pos.0 as usize).cloned())
        .flatten()
}

pub fn get_mut<T: Clone>(grid: &mut Vec<Vec<T>>, pos: (i32, i32)) -> Option<&mut T> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }

    grid.get_mut(pos.1 as usize)
        .map(|row| row.get_mut(pos.0 as usize))
        .flatten()
}

pub fn set<T: Clone>(grid: &mut Vec<Vec<T>>, pos: (i32, i32), val: T) {
    if pos.0 < 0 || pos.1 < 0 {
        return;
    }

    grid.get_mut(pos.1 as usize)
        .map(|x| x.get_mut(pos.0 as usize).map(|x| *x = val));
}

pub fn enumerate<T>(grid: &Vec<Vec<T>>) -> impl Iterator<Item = ((i32, i32), &T)> {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, val)| ((x as i32, y as i32), val))
        })
        .flatten()
}

pub fn enumerate_mut<T>(grid: &mut Vec<Vec<T>>) -> impl Iterator<Item = ((i32, i32), &mut T)> {
    grid.iter_mut()
        .enumerate()
        .map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, val)| ((x as i32, y as i32), val))
        })
        .flatten()
}
