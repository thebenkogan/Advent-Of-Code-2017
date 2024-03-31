use aoc2017::run_day;

fn matching_sum(input: &str, skip: usize) -> i32 {
    let nums = input.chars().map(|c| c.to_digit(10).unwrap());
    nums.clone()
        .zip(nums.cycle().skip(skip))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .sum::<u32>() as i32
}

fn p1(input: &str) -> i32 {
    matching_sum(input, 1)
}

fn p2(input: &str) -> i32 {
    let steps = input.len() / 2;
    matching_sum(input, steps)
}

fn main() {
    run_day(p1, p2);
}
