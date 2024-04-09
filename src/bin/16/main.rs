use std::collections::VecDeque;

use aoc2017::run_day;

fn run_dance(input: &str, q: &mut VecDeque<char>) {
    input.split(',').for_each(|mv| {
        if mv.starts_with('s') {
            let amnt = mv.strip_prefix('s').unwrap().parse::<usize>().unwrap();
            q.rotate_right(amnt);
        } else if mv.starts_with('x') {
            if let Some((a, b)) = mv.strip_prefix('x').unwrap().split_once('/') {
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                q.swap(a, b);
            }
        } else if let Some((a, b)) = mv.strip_prefix('p').unwrap().split_once('/') {
            let a = a.chars().next().unwrap();
            let b = b.chars().next().unwrap();
            let a_index = q.iter().position(|&c| c == a).unwrap();
            let b_index = q.iter().position(|&c| c == b).unwrap();
            q.swap(a_index, b_index);
        }
    });
}

fn p1(input: &str) -> String {
    let mut q = VecDeque::new();
    for c in 97..113 {
        q.push_back(char::from_u32(c).unwrap());
    }

    run_dance(input, &mut q);

    q.iter().collect::<String>()
}

fn p2(input: &str) -> String {
    let mut q = VecDeque::new();
    for c in 97..113 {
        q.push_back(char::from_u32(c).unwrap());
    }

    let start_hash = q.iter().collect::<String>();
    let mut cycle = 0;
    loop {
        run_dance(input, &mut q);
        cycle += 1;
        if start_hash == q.iter().collect::<String>() {
            break;
        }
    }

    for _ in 0..(1_000_000_000 % cycle) {
        run_dance(input, &mut q);
    }

    q.iter().collect::<String>()
}

fn main() {
    run_day(p1, p2);
}
