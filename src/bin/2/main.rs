use aoc2017::run_day;
use itertools::Itertools;

fn p1(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .minmax();
            let (min, max) = nums.into_option().unwrap();
            max - min
        })
        .sum()
}

fn p2(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            for i in 0..nums.len() {
                for j in 0..nums.len() {
                    if i != j && nums[i] % nums[j] == 0 {
                        return nums[i] / nums[j];
                    }
                }
            }
            unreachable!("No evenly divisible pair found")
        })
        .sum()
}

fn main() {
    run_day(p1, p2);
}
