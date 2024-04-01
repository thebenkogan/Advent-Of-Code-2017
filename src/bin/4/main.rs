use aoc2017::run_day;
use itertools::Itertools;

fn p1(input: &str) -> i32 {
    input
        .split('\n')
        .filter(|&line| line.split(' ').all_unique())
        .count() as i32
}

fn p2(input: &str) -> i32 {
    input
        .split('\n')
        .filter(|&line| {
            line.split(' ')
                .map(|word| {
                    let mut chars = word.chars().collect::<Vec<_>>();
                    chars.sort();
                    chars.iter().collect::<String>()
                })
                .all_unique()
        })
        .count() as i32
}

fn main() {
    run_day(p1, p2);
}
