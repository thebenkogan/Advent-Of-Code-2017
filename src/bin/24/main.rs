use std::collections::{HashMap, HashSet};

use aoc2017::run_day;

fn parse_graph(input: &str) -> HashMap<i32, HashSet<i32>> {
    let mut adj = HashMap::new();

    input
        .split('\n')
        .map(|line| {
            line.split_once('/')
                .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                .unwrap()
        })
        .for_each(|(a, b)| {
            adj.entry(a).or_insert_with(HashSet::new).insert(b);
            adj.entry(b).or_insert_with(HashSet::new).insert(a);
        });

    adj
}

fn p1(input: &str) -> i32 {
    let adj = parse_graph(input);

    let mut stack = vec![(0, 0, HashSet::new())];
    let mut max_strength = 0;
    while let Some((node, strength, seen)) = stack.pop() {
        max_strength = max_strength.max(strength);
        for &next in adj.get(&node).unwrap() {
            let pair = if node < next {
                (node, next)
            } else {
                (next, node)
            };
            let mut new_seen = seen.clone();
            if new_seen.insert(pair) {
                stack.push((next, strength + node + next, new_seen));
            }
        }
    }

    max_strength
}

fn p2(input: &str) -> i32 {
    let adj = parse_graph(input);

    let mut stack = vec![(0, 0, 0, HashSet::new())];
    let mut max_strength = 0;
    let mut max_length = 0;
    while let Some((node, strength, length, seen)) = stack.pop() {
        if length > max_length || (length == max_length && strength > max_strength) {
            max_length = length;
            max_strength = strength
        }
        for &next in adj.get(&node).unwrap() {
            let pair = if node < next {
                (node, next)
            } else {
                (next, node)
            };
            let mut new_seen = seen.clone();
            if new_seen.insert(pair) {
                stack.push((next, strength + node + next, length + 1, new_seen));
            }
        }
    }

    max_strength
}

fn main() {
    run_day(p1, p2);
}
