use aoc_runner_derive::aoc;
use std::{
    collections::HashMap,
    simd::{i32x8, num::SimdInt},
};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut v1 = [0; 1000];
    let mut v2 = [0; 1000];

    for (i, line) in input.lines().enumerate() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..=12].parse::<i32>().unwrap();

        unsafe {
            *v1.get_unchecked_mut(i) = a;
            *v2.get_unchecked_mut(i) = b;
        }
    }

    v1.sort_unstable();
    v2.sort_unstable();

    let chunk_size = 8;
    let len = v1.len();

    let mut i = 0;
    let mut sum = 0;

    unsafe {
        while i + chunk_size <= len {
            let a = i32x8::from_slice(v1.get_unchecked(i..i + chunk_size));
            let b = i32x8::from_slice(v2.get_unchecked(i..i + chunk_size));
            sum += (a - b).abs().reduce_sum();
            i += chunk_size;
        }

        while i < len {
            sum += (v1.get_unchecked(i) - v2.get_unchecked(i)).abs();
            i += 1;
        }
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut v1 = [0; 1000];
    let mut count = HashMap::with_capacity(1000);

    for (i, line) in input.lines().enumerate() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..=12].parse::<i32>().unwrap();

        unsafe { *v1.get_unchecked_mut(i) = a };
        count
            .entry(b)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    v1.iter().map(|&x| x * count.get(&x).unwrap_or(&0)).sum()
}
