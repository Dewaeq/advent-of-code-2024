use std::{
    collections::BinaryHeap,
    simd::{i32x16, i32x32, i32x4, i32x8, num::SimdInt},
};

use aoc_runner_derive::aoc;

#[aoc(day1, part1, normal)]
fn part1(input: &str) -> i32 {
    let mut v1 = Vec::with_capacity(1000);
    let mut v2 = Vec::with_capacity(1000);

    for line in input.lines() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..].parse::<i32>().unwrap();

        v1.push(a);
        v2.push(b);
    }

    v1.sort_unstable();
    v2.sort_unstable();
    v1.iter().zip(v2.iter()).map(|(&a, &b)| (a - b).abs()).sum()
}

#[aoc(day1, part1, array)]
fn part1_array(input: &str) -> i32 {
    let mut v1 = [0; 1000];
    let mut v2 = [0; 1000];

    for (i, line) in input.lines().enumerate() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..].parse::<i32>().unwrap();

        unsafe {
            *v1.get_unchecked_mut(i) = a;
            *v2.get_unchecked_mut(i) = b;
        }
    }

    v1.sort_unstable();
    v2.sort_unstable();
    v1.iter().zip(v2.iter()).map(|(&a, &b)| (a - b).abs()).sum()
}

#[aoc(day1, part1, binheap)]
fn part1_binheap(input: &str) -> i32 {
    let mut v1 = BinaryHeap::with_capacity(1000);
    let mut v2 = BinaryHeap::with_capacity(1000);

    for line in input.lines() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..].parse::<i32>().unwrap();
        v1.push(a);
        v2.push(b);
    }

    v1.into_iter_sorted()
        .zip(v2.into_iter_sorted())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[aoc(day1, part1, simd4)]
fn part1_simd4(input: &str) -> i32 {
    let mut v1 = [0; 1000];
    let mut v2 = [0; 1000];

    for (i, line) in input.lines().enumerate() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..].parse::<i32>().unwrap();

        unsafe {
            *v1.get_unchecked_mut(i) = a;
            *v2.get_unchecked_mut(i) = b;
        }
    }

    v1.sort_unstable();
    v2.sort_unstable();

    let chunk_size = 4;
    let len = v1.len();

    let mut i = 0;
    let mut sum = 0;

    unsafe {
        while i + chunk_size <= len {
            let a = i32x4::from_slice(v1.get_unchecked(i..i + chunk_size));
            let b = i32x4::from_slice(v2.get_unchecked(i..i + chunk_size));
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
#[aoc(day1, part1, simd8)]
fn part1_simd8(input: &str) -> i32 {
    let mut v1 = [0; 1000];
    let mut v2 = [0; 1000];

    for (i, line) in input.lines().enumerate() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..].parse::<i32>().unwrap();

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
#[aoc(day1, part1, simd16)]
fn part1_simd16(input: &str) -> i32 {
    let mut v1 = [0; 1000];
    let mut v2 = [0; 1000];

    for (i, line) in input.lines().enumerate() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..].parse::<i32>().unwrap();

        unsafe {
            *v1.get_unchecked_mut(i) = a;
            *v2.get_unchecked_mut(i) = b;
        }
    }

    v1.sort_unstable();
    v2.sort_unstable();

    let chunk_size = 16;
    let len = v1.len();

    let mut i = 0;
    let mut sum = 0;

    unsafe {
        while i + chunk_size <= len {
            let a = i32x16::from_slice(v1.get_unchecked(i..i + chunk_size));
            let b = i32x16::from_slice(v2.get_unchecked(i..i + chunk_size));
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
#[aoc(day1, part1, simd32)]
fn part1_simd32(input: &str) -> i32 {
    let mut v1 = [0; 1000];
    let mut v2 = [0; 1000];

    for (i, line) in input.lines().enumerate() {
        let a = line[..=4].parse::<i32>().unwrap();
        let b = line[8..].parse::<i32>().unwrap();

        unsafe {
            *v1.get_unchecked_mut(i) = a;
            *v2.get_unchecked_mut(i) = b;
        }
    }

    v1.sort_unstable();
    v2.sort_unstable();

    let chunk_size = 32;
    let len = v1.len();

    let mut i = 0;
    let mut sum = 0;

    unsafe {
        while i + chunk_size <= len {
            let a = i32x32::from_slice(v1.get_unchecked(i..i + chunk_size));
            let b = i32x32::from_slice(v2.get_unchecked(i..i + chunk_size));
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
fn part2(input: &str) -> String {
    todo!()
}
