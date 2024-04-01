use std::collections::HashSet;

use aoc2017::run_day;

fn hash(banks: &Vec<i32>) -> String {
    format!("{:?}", banks)
}

fn reallocate(banks: &mut Vec<i32>) {
    let max_bank = banks.iter().max().unwrap();
    let mut index = banks.iter().position(|&x| x == *max_bank).unwrap();
    let mut to_distribute = banks[index];
    banks[index] = 0;
    while to_distribute > 0 {
        index = (index + 1) % banks.len();
        banks[index] += 1;
        to_distribute -= 1;
    }
}

fn p1(input: &str) -> i32 {
    let mut banks = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut seen = HashSet::new();
    seen.insert(hash(&banks));

    loop {
        reallocate(&mut banks);
        let hsh = hash(&banks);
        if seen.contains(&hsh) {
            return seen.len() as i32;
        } else {
            seen.insert(hsh);
        }
    }
}

fn p2(input: &str) -> i32 {
    let mut banks = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut seen = HashSet::new();
    seen.insert(hash(&banks));

    loop {
        reallocate(&mut banks);
        let hsh = hash(&banks);
        if seen.contains(&hsh) {
            break;
        } else {
            seen.insert(hsh);
        }
    }

    let target_hash = hash(&banks);
    let mut cycles = 0;
    loop {
        reallocate(&mut banks);
        cycles += 1;
        if hash(&banks) == target_hash {
            return cycles;
        }
    }
}

fn main() {
    run_day(p1, p2);
}
