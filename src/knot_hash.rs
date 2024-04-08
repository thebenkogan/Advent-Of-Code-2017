pub const LIST_SIZE: i32 = 256;

pub fn do_knot_hash_round(
    nums: &mut [i32],
    lengths: &Vec<i32>,
    curr: i32,
    skip: i32,
) -> (i32, i32) {
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

const NUM_ROUNDS: i32 = 64;

pub fn knot_hash(input: &str) -> String {
    let lengths = input
        .chars()
        .map(|c| c as i32)
        .chain([17, 31, 73, 47, 23])
        .collect();

    let mut curr = 0;
    let mut skip = 0;
    let mut nums: Vec<i32> = (0..LIST_SIZE).collect();
    for _ in 0..NUM_ROUNDS {
        let (next_curr, next_skip) = do_knot_hash_round(&mut nums, &lengths, curr, skip);
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
