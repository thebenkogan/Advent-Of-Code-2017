use aoc2017::run_day;

fn p1(input: &str) -> i32 {
    let mut jumps: Vec<i32> = input
        .split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut i = 0;
    let mut steps = 0;
    while i >= 0 && i < jumps.len() as i32 {
        jumps[i as usize] += 1;
        i += jumps[i as usize] - 1;
        steps += 1;
    }
    steps
}

fn p2(input: &str) -> i32 {
    let mut jumps: Vec<i32> = input
        .split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut i = 0;
    let mut steps = 0;
    while i >= 0 && i < jumps.len() as i32 {
        let jump = jumps[i as usize];
        jumps[i as usize] += if jump >= 3 { -1 } else { 1 };
        i += jump;
        steps += 1;
    }
    steps
}

fn main() {
    run_day(p1, p2);
}
