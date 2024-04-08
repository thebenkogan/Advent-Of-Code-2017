use aoc2017::{
    knot_hash::{do_knot_hash_round, knot_hash, LIST_SIZE},
    run_day,
};

fn p1(input: &str) -> String {
    let lengths: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut nums: Vec<i32> = (0..LIST_SIZE).collect();
    do_knot_hash_round(&mut nums, &lengths, 0, 0);
    let ans = nums[0] * nums[1];
    ans.to_string()
}

fn p2(input: &str) -> String {
    knot_hash(input)
}

fn main() {
    run_day(p1, p2);
}
