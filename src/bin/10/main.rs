use aoc2017::run_day;

const LIST_SIZE: i32 = 256;

fn do_round(nums: &mut [i32], lengths: &Vec<i32>, curr: i32, skip: i32) -> (i32, i32) {
    let mut curr = curr;
    let mut skip = skip;
    for &length in lengths {
        let mut temp: Vec<i32> = Vec::new();
        for i in 0..length {
            temp.push(nums[((curr + i) % LIST_SIZE) as usize]);
        }
        temp.reverse();
        for i in 0..length {
            nums[((curr + i) % LIST_SIZE) as usize] = temp[i as usize];
        }
        curr = (curr + length + skip) % LIST_SIZE;
        skip += 1;
    }
    (curr, skip)
}

fn p1(input: &str) -> String {
    let lengths: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut nums: Vec<i32> = (0..LIST_SIZE).collect();
    do_round(&mut nums, &lengths, 0, 0);
    let ans = nums[0] * nums[1];
    ans.to_string()
}

const NUM_ROUNDS: i32 = 64;

fn p2(input: &str) -> String {
    let lengths: Vec<i32> = input
        .chars()
        .map(|c| c as i32)
        .chain(vec![17, 31, 73, 47, 23])
        .collect();

    let mut curr = 0;
    let mut skip = 0;
    let mut nums: Vec<i32> = (0..LIST_SIZE).collect();
    for _ in 0..NUM_ROUNDS {
        let (next_curr, next_skip) = do_round(&mut nums, &lengths, curr, skip);
        curr = next_curr;
        skip = next_skip;
    }

    let hex_chars: Vec<String> = nums
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
        .map(|x| format!("{:02x}", x))
        .collect();

    hex_chars.join("")
}

fn main() {
    run_day(p1, p2);
}
