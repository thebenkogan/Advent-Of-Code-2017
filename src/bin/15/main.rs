use aoc2017::run_day;
use itertools::Itertools;

fn next_value(prev: u64, factor: u64) -> u64 {
    (prev * factor) % 2147483647
}

const SIXTEEN_ONES: u64 = 0xFFFF;

fn p1(input: &str) -> i32 {
    let a_factor = 16807;
    let b_factor = 48271;

    let (mut a_val, mut b_val) = input
        .split('\n')
        .map(|line| line.split(' ').last().unwrap().parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut total = 0;
    for _ in 0..40_000_000 {
        a_val = next_value(a_val, a_factor);
        b_val = next_value(b_val, b_factor);
        if (a_val & SIXTEEN_ONES) == (b_val & SIXTEEN_ONES) {
            total += 1
        }
    }

    total
}

fn next_value_multiple(prev: u64, factor: u64, multiple: u64) -> u64 {
    let mut val = prev;
    loop {
        val = (val * factor) % 2147483647;
        if val % multiple == 0 {
            return val;
        }
    }
}

fn p2(input: &str) -> i32 {
    let a_factor = 16807;
    let b_factor = 48271;
    let a_multiple = 4;
    let b_multiple = 8;

    let (mut a_val, mut b_val) = input
        .split('\n')
        .map(|line| line.split(' ').last().unwrap().parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut total = 0;
    for _ in 0..5_000_000 {
        a_val = next_value_multiple(a_val, a_factor, a_multiple);
        b_val = next_value_multiple(b_val, b_factor, b_multiple);
        if (a_val & SIXTEEN_ONES) == (b_val & SIXTEEN_ONES) {
            total += 1
        }
    }

    total
}

fn main() {
    run_day(p1, p2);
}
