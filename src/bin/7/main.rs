use std::collections::HashMap;

use aoc2017::run_day;
use itertools::Itertools;

fn build_graph(input: &str) -> (HashMap<&str, Vec<&str>>, HashMap<&str, i32>, String) {
    let mut adj = HashMap::new();
    let mut indegree = HashMap::new();
    let mut node_to_weight = HashMap::new();

    input.split('\n').for_each(|line| {
        let (node, neighbors) = line
            .split_once(" -> ")
            .map(|(l, n)| (l, Some(n)))
            .unwrap_or((line, None));
        let (name, weight) = node.split_once(' ').unwrap();
        node_to_weight.insert(name, weight[1..weight.len() - 1].parse::<i32>().unwrap());
        let neighbors: Vec<&str> = neighbors.map(|n| n.split(", ").collect()).unwrap_or(vec![]);
        indegree.entry(name).or_insert(0);
        neighbors.iter().for_each(|n| {
            *indegree.entry(*n).or_insert(0) += 1;
        });
        adj.insert(name, neighbors);
    });

    let (root, _) = indegree.iter().find(|(_, &v)| v == 0).unwrap();
    (adj, node_to_weight, root.to_string())
}

fn p1(input: &str) -> String {
    let (_, _, root) = build_graph(input);
    root
}

static mut FIXED_WEIGHT: i32 = 0;

fn dfs(node: &str, adj: &HashMap<&str, Vec<&str>>, node_to_weight: &HashMap<&str, i32>) -> i32 {
    let weight = *node_to_weight.get(node).unwrap();
    let children_weights = adj[node]
        .iter()
        .map(|c| dfs(c, adj, node_to_weight))
        .collect::<Vec<i32>>();
    if !children_weights.iter().all_equal() {
        let counts = children_weights.iter().counts();
        let (&target_weight, _) = counts.iter().find(|(_, &v)| v != 1).unwrap();
        let (&bad_weight, _) = counts.iter().find(|(_, &v)| v == 1).unwrap();
        let (bad_child_index, _) = children_weights
            .iter()
            .find_position(|&w| w == bad_weight)
            .unwrap();
        let child = adj[node][bad_child_index];
        let child_weight = *node_to_weight.get(child).unwrap();
        let diff = target_weight - bad_weight;
        unsafe {
            if FIXED_WEIGHT == 0 {
                FIXED_WEIGHT = child_weight + diff;
            }
        }
    }
    weight + children_weights.iter().sum::<i32>()
}

fn p2(input: &str) -> String {
    let (adj, node_to_weight, root) = build_graph(input);

    dfs(&root, &adj, &node_to_weight);

    unsafe { FIXED_WEIGHT.to_string() }
}

fn main() {
    run_day(p1, p2);
}
