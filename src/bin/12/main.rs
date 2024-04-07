use std::collections::{HashMap, HashSet};

use aoc2017::run_day;

fn build_graph(input: &str) -> HashMap<i32, Vec<i32>> {
    let mut adj = HashMap::new();
    input.split('\n').for_each(|line| {
        let (a, b) = line.split_once(" <-> ").unwrap();
        let src = a.parse::<i32>().unwrap();
        let dsts: Vec<i32> = b.split(", ").map(|x| x.parse::<i32>().unwrap()).collect();
        adj.insert(src, dsts);
    });
    adj
}

fn p1(input: &str) -> i32 {
    let adj = build_graph(input);

    let mut total = 0;
    let mut stack = vec![0];
    let mut seen = HashSet::new();
    seen.insert(0);
    while let Some(node) = stack.pop() {
        total += 1;
        for &dst in adj.get(&node).unwrap() {
            if !seen.contains(&dst) {
                stack.push(dst);
                seen.insert(dst);
            }
        }
    }

    total
}

fn p2(input: &str) -> i32 {
    let adj = build_graph(input);

    let mut groups = 0;
    let mut seen = HashSet::new();
    for &key in adj.keys() {
        if seen.contains(&key) {
            continue;
        }
        groups += 1;
        seen.insert(key);
        let mut stack = vec![key];
        while let Some(node) = stack.pop() {
            for &dst in adj.get(&node).unwrap() {
                if !seen.contains(&dst) {
                    stack.push(dst);
                    seen.insert(dst);
                }
            }
        }
    }

    groups
}

fn main() {
    run_day(p1, p2);
}
